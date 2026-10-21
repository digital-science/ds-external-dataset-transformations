use gcloud_sdk::GoogleRestApi;
use gcloud_sdk::google_rest_apis::storage_v1::objects_api::{
    storage_objects_insert_ext_stream,
    StoragePeriodObjectsPeriodInsertParams,
};

use avro_schema::*;
use apache_avro::Schema;
use apache_avro::stream::writer::Writer as AvroWriter;
use apache_avro::Error as AvroError;
use quick_xml::de::from_str;

use futures::AsyncWriteExt;
use futures::stream::{self, TryStreamExt};

use tokio::sync::mpsc;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio_util::io::ReaderStream;

use std::sync::atomic::{AtomicUsize, Ordering};

use log::{debug, info, error};
use env_logger;

use structopt::StructOpt;

mod model;

#[cfg(not(feature = "schema_docs"))]
mod tarball;

#[cfg(feature = "schema_docs")]
mod schema;


const DEST_WRITER_MAX_BUFFER_SIZE : usize = 2 * 1024 * 1024; // 2MB

const MAX_PENDING_RECORDS_IN_CHANNEL : usize = 2000;


#[derive(StructOpt, Debug)]
#[structopt(name = "orcid-snapshot-ingest", about = "Ingests ORCID snapshot XML data into Avro files in GCS.")]
struct Opt {
    /// Path to the local directory containing ORCID snapshot XML summaries.
    #[structopt(long, required_unless = "source-tarball", conflicts_with = "source-tarball")]
    source_path: Option<String>,

    /// Path to an ORCID summaries `.tar.gz`, streamed directly instead of extracting to disk.
    #[structopt(long)]
    source_tarball: Option<String>,

    /// Local directory for staging Avro files before upload (tarball mode only).
    #[structopt(long, default_value = "./staging")]
    staging_dir: String,

    /// Maximum number of Avro files staged/uploading at once (tarball mode only).
    #[structopt(long, default_value = "16")]
    concurrent_uploads: usize,

    /// GCS bucket name for the destination Avro files.
    #[structopt(long)]
    destination_bucket: String,

    /// Key prefix within the destination GCS bucket.
    #[structopt(long)]
    destination_prefix: String,

    /// Number of top-level directories to process concurrently.
    #[structopt(long, default_value = "32")]
    concurrent_processing: usize,

    /// Maximum number of records written per output Avro file.
    #[structopt(long, default_value = "10000")]
    max_records_per_file: usize,

    /// Optional path to a history file used to skip previously completed prefixes.
    #[structopt(long)]
    history_file: Option<String>,
}

type BaseError = Box<dyn std::error::Error + Sync + Send + 'static>;


#[cfg(not(feature = "schema_docs"))]
#[tokio::main(worker_threads = 32)]
async fn main() -> Result<(), BaseError> {

    env_logger::init();

    // rustls 0.23 needs an explicit crypto provider; none is selected via dependency features.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let opt = Opt::from_args();

    let gcp_client = GoogleRestApi::new().await.expect("GoogleRestApi client initialization");

    // We can optionally read in a history of previous processing to skip over source directories
    // that have already been successfully processed.
    let history = match &opt.history_file {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(file) => file,
            Err(error) => {
                error!("reading history file failed, caused by: {}", error);
                return Err(error.into());
            }
        },
        None => String::new(),
    };
    let lines : Vec<_> = history.split("\n").collect();

    let completed_lines : Vec<_> = lines.iter()
        .filter(|s| s.contains("Completed processing prefix"))
        .collect();

    let completed_path_regex = regex::Regex::new(r".*Completed processing prefix:\s+([^\s]*).*")?;
    let completed_paths : std::collections::HashSet<_> = completed_lines.iter()
        .map(|s| {
            if let Some(c) = completed_path_regex.captures(s) {
                return Some(c[1].to_owned());
            }
            None
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();

    for path in completed_paths.iter() {
        info!("Loaded from history already completed processing prefix: {}", path);
    }

    if let Some(source_tarball) = &opt.source_tarball {
        let json_schema = <model::Record as AvroSchemaRecord>::to_json_schema()?;
        let schema : &'static Schema = Box::leak(Box::new(Schema::parse_str(json_schema.as_str())?));
        let options = tarball::TarballOptions {
            tarball: source_tarball.clone(),
            dest_bucket: opt.destination_bucket.clone(),
            dest_prefix: opt.destination_prefix.clone(),
            staging_dir: opt.staging_dir.clone().into(),
            max_records_per_file: opt.max_records_per_file,
            concurrent_processing: opt.concurrent_processing,
            concurrent_uploads: opt.concurrent_uploads,
            completed_paths,
        };
        return match tarball::process_tarball(options, gcp_client, schema).await {
            Ok(total_count) => {
                info!("Completed processing all prefixes, total output count: {}", total_count);
                Ok(())
            },
            Err(error) => {
                error!("processing source tarball failed, caused by: {}", error);
                Err(error)
            }
        };
    }
    let source_location_path = opt.source_path.as_ref().unwrap();

    // We want to iterate the source directory and then iterate each directory found within that directory.
    let mut top_level_dir = tokio::fs::read_dir(source_location_path).await?;
    let mut top_level_dir_listing = vec![];

    while let Some(entry) = top_level_dir.next_entry().await? {
        match entry.metadata().await {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let path = entry.path();
                    let path_str = path.to_str().unwrap();
                    if !completed_paths.contains(path_str) {
                        top_level_dir_listing.push(entry);
                    } else {
                        info!("Skipping already completed processing prefix: {}", path.display());
                    }
                }
            },
            Err(error) => {
                error!("reading metadata entry for top-level directory failed, caused by: {}", error);
                return Err(error.into());
            }
        }
    }

    let json_schema = <model::Record as AvroSchemaRecord>::to_json_schema()?;
    let schema = Schema::parse_str(json_schema.as_str())?;
    let total_count = AtomicUsize::new(0);
    let tc = &total_count;

    let result = stream::iter(top_level_dir_listing.into_iter().map(|f| Ok(f))).try_for_each_concurrent(opt.concurrent_processing, |top_level_dir| {

        let path = top_level_dir.path();
        let last_path_part = path.components().last().unwrap();
        let dir_prefix = last_path_part.as_os_str().to_str().unwrap().to_owned();

        let gcp_client = &gcp_client;
        let schema = &schema;
        let dest_bucket = opt.destination_bucket.as_str();
        let dest_prefix = opt.destination_prefix.as_str();
        let max_records_per_file = opt.max_records_per_file;

        async move {

            info!("Starting processing at prefix: {}", path.display());

            let result = process_directory_into_destination(dir_prefix, top_level_dir, gcp_client, dest_bucket, dest_prefix, schema, max_records_per_file).await;
            match result {
                Ok(file_count) => {
                    info!("Completed processing prefix: {} – generated {} files.", path.display(), file_count);
                    tc.fetch_add(file_count, Ordering::SeqCst);
                    Ok::<_, BaseError>(())
                },
                Err(error) => {
                    error!("processing directory failed, caused by: {} [dir='{}']", error, path.display());
                    Err(error)
                }
            }
        }

    }).await;

    if let Err(error) = result {
        error!("processing source files failed, caused by: {}", error);
        return Err(error);
    }

    let total_count = total_count.into_inner();
    info!("Completed processing all prefixes, total output count: {}", total_count);

    Ok(())
}

async fn process_directory_into_destination<P: AsRef<str>, D: AsRef<str>>(
    prefix: P,
    directory: tokio::fs::DirEntry,
    gcp_client: &GoogleRestApi,
    dest_bucket: &str,
    dest_prefix: D,
    schema: &Schema,
    max_records_per_file: usize,
) -> Result<usize, BaseError> {

    let (sender, mut receiver) = mpsc::channel::<model::Record>(MAX_PENDING_RECORDS_IN_CHANNEL);
    let prefix = prefix.as_ref().to_owned();

    let dir_processor = tokio::spawn(async move {
        let result = process_directory(directory, sender).await;
        result
    });

    let writer = async move {

        let mut file_count : usize = 0;
        let mut current_file_record_count = 0;
        let prefix = prefix;
        let dest_prefix = dest_prefix.as_ref();

        let mut dest_stream = None;
        let mut dest_file_writer = None;

        while let Some(record) = receiver.recv().await {

            let current_dest_stream = match dest_stream.as_mut() {
                Some(stream) => {
                    stream
                },
                None => {
                    let (dest_writer, dest_reader) = tokio::io::duplex(DEST_WRITER_MAX_BUFFER_SIZE);
                    let dest_writer = dest_writer.compat_write();
                    dest_stream = Some(AvroWriter::new(schema, dest_writer));

                    file_count = file_count + 1;
                    let destination_key = format!("{}/orcid_{}_{:0>8}.avro", dest_prefix, prefix, file_count);
                    let bucket_name = dest_bucket.to_owned();

                    let config = gcp_client.create_google_storage_v1_config()
                        .await
                        .map_err(|e| -> BaseError { format!("failed to create GCS config: {}", e).into() })?;

                    dest_file_writer = Some(tokio::spawn(async move {
                        let params = StoragePeriodObjectsPeriodInsertParams {
                            bucket: bucket_name,
                            name: Some(destination_key),
                            ..Default::default()
                        };
                        let reader_stream = ReaderStream::new(dest_reader);
                        storage_objects_insert_ext_stream(
                            &config,
                            params,
                            Some("application/avro".to_string()),
                            reader_stream,
                        ).await.map_err(|e| -> BaseError { Box::new(e) })
                    }));

                    dest_stream.as_mut().unwrap()
                }
            };

            if let Err(error) = current_dest_stream.append_ser(&record).await {
                let _ = close_stream(dest_stream.take().unwrap()).await;
                error!("appending record to avro failed: {}", error);
                return Err::<_, BaseError>(Box::new(error));
            }

            current_file_record_count = current_file_record_count + 1;

            // If we have reached the maximum record count, close the stream.
            if current_file_record_count >= max_records_per_file {

                current_file_record_count = 0;

                let mut dest_stream = dest_stream.take().unwrap();
                dest_stream.flush().await.map_err(|e| Box::new(e))?;
                close_stream(dest_stream).await.map_err(|e| Box::new(e))?;

                let dest_file_writer = dest_file_writer.take().unwrap();
                match dest_file_writer.await {
                    Ok(Ok(_)) => {
                        debug!("destination file writer completed");
                    },
                    Ok(Err(error)) => {
                        error!("destination file writer failed, caused by: {}", error);
                        return Err(error);
                    },
                    Err(error) => {
                        error!("destination file writer failed, caused by: {}", error);
                        return Err(Box::new(error));
                    }
                }
            }
        }

        if current_file_record_count > 0 {

            let mut dest_stream = dest_stream.take().unwrap();
            dest_stream.flush().await.map_err(|e| Box::new(e))?;
            close_stream(dest_stream).await.map_err(|e| Box::new(e))?;

            let dest_file_writer = dest_file_writer.take().unwrap();
            match dest_file_writer.await {
                Ok(Ok(_)) => {
                    debug!("destination file writer completed");
                },
                Ok(Err(error)) => {
                    error!("destination file writer failed, caused by: {}", error);
                    return Err(error);
                },
                Err(error) => {
                    error!("destination file writer failed, caused by: {}", error);
                    return Err(Box::new(error));
                }
            }
        }

        Ok::<_, BaseError>(file_count)
    };

    match tokio::join!(writer, dir_processor) {
        (Err(error), _) => {
            error!("output Avro writer operation failed, caused by: {}", error);
            Err(error)
        },
        (_, Err(error)) => {
            error!("directory reader operation failed (thread failure), caused by: {}", error);
            Err(Box::new(error))
        },
        (Ok(_), Ok(Err(error))) => {
            error!("directory reader operation failed, caused by: {}", error);
            Err(error)
        }
        (Ok(file_count), Ok(Ok(_))) => {
            Ok(file_count)
        }
    }
}


async fn process_directory(directory : tokio::fs::DirEntry, record_channel: mpsc::Sender<model::Record>) -> Result<(), BaseError> {

    // Note: we process multiple directories at the same time rather than processing multiple
    // files at the same time. Regardless, we will saturate the available compute resources so
    // there is no need to try and parallelize this part of the code.

    let mut dir_listing = tokio::fs::read_dir(directory.path()).await?;

    while let Some(entry) = dir_listing.next_entry().await? {
        match entry.metadata().await {
            Ok(metadata) => {
                if metadata.is_file() {
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        if let Some(filename) = filename.to_str() {
                            if filename.to_ascii_lowercase().ends_with(".xml") {

                                // This is an XML file we want to process.
                                let record = match process_xml_file(path).await {
                                    Ok(Some(record)) => record,
                                    Ok(None) => continue,
                                    Err(error) => {
                                        error!("processing XML file failed, caused by: {}", error);
                                        return Err(error.into());
                                    }
                                };

                                // Send record to the mpsc channel that handles writing data back out.
                                if let Err(error) = record_channel.send(record).await {
                                    error!("sending parsed XML record from failed, caused by: {}", error);
                                    return Err(error.into());
                                }
                            }
                        }
                    }
                }
            },
            Err(error) => {
                error!("reading metadata entry for inner-directory failed, caused by: {}", error);
                return Err(error.into());
            }
        }
    }

    Ok(())
}

async fn process_xml_file(file_path: std::path::PathBuf) -> Result<Option<model::Record>, BaseError> {

    let xml = tokio::fs::read(&file_path).await?;
    parse_xml_bytes(&xml, &file_path.display().to_string())
}

pub(crate) fn parse_xml_bytes(xml: &[u8], name: &str) -> Result<Option<model::Record>, BaseError> {

    // Some of the XML files just contain an error, so we skip over these.
    let xml_content = String::from_utf8_lossy(xml);
    if xml_content.contains("<error:error") {
        debug!("Skipping file: {} – contains `error:error`", name);
        return Ok(None);
    }

    let mut record : model::Record = match from_str(&xml_content) {
        Ok(record) => record,
        Err(error) => {
            error!("Error parsing file at path, '{}'. Caused by: {}", name, error);
            return Err(error.into());
        }
    };
    record.cleanup();

    Ok(Some(record))
}

// ====

async fn close_stream(writer: AvroWriter<'_, tokio_util::compat::Compat<tokio::io::DuplexStream>>) -> Result<(), AvroError> {

    let mut inner = writer.into_inner().await?;
    let _ = inner.close().await;
    Ok(())
}

// ====
// Feature: SchemaDocs JSON creation
// ====

#[cfg(feature = "schema_docs")]
use std::path::PathBuf;

#[cfg(feature = "schema_docs")]
#[tokio::main]
async fn main() -> Result<(), BaseError> {

    // Note: this binary expects the CWD to be the top-level directory of this project when being executed.
    write_schema_json("./schema/schema.json").await;
    return Ok(());
}

#[cfg(feature = "schema_docs")]
async fn write_schema_json<D: Into<PathBuf>>(destination: D) {

    use schema_docs::types::SchemaVariant;
    use std::fs::File;
    use std::io::prelude::*;

    let dest_file = destination.into();
    let data = schema::generate_schema_description_for_type::<model::Record>(SchemaVariant::External);
    let path_display = dest_file.display();

    let json_output = match serde_json::to_string_pretty(&data) {
        Ok(json_output) => json_output,
        Err(error) => {
            panic!("Error: unable to create JSON representation of schema descriptions [error='{}']", error);
        }
    };

    let mut output_file = match File::create(&dest_file) {
        Ok(file) => file,
        Err(error) => panic!("Error: couldn't create out file at path {} [error='{}']", path_display, error),
    };

    match output_file.write_all(json_output.as_bytes()) {
        Ok(_) => println!("Successfully wrote JSON schema description to {}", path_display),
        Err(error) => panic!("Error: couldn't write JSON data file at path {} [error='{}']", path_display, error),
    }
}
