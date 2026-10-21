// Streaming ingest directly from an ORCID summaries `.tar.gz`, without extracting it to disk.
//
// The tarball is read sequentially on a blocking thread. Entries are grouped by their parent
// directory (e.g. `ORCID_2025_10_summaries/000/…`), and each directory is handed to its own task,
// which parses the XML in parallel and writes Avro files into a local staging directory. Each
// completed Avro file is uploaded to GCS (with retries) and then deleted, so local disk usage is
// bounded by `concurrent_uploads × file size`. Output object names match the directory mode.

use gcloud_sdk::GoogleRestApi;
use gcloud_sdk::google_rest_apis::storage_v1::Error as GcsError;
use gcloud_sdk::google_rest_apis::storage_v1::objects_api::{
    storage_objects_insert_ext_stream,
    StoragePeriodObjectsPeriodInsertParams,
};

use apache_avro::Schema;
use apache_avro::stream::writer::Writer as AvroWriter;

use futures::AsyncWriteExt;
use futures::stream::{self, StreamExt};

use tokio::sync::{mpsc, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinHandle;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use tokio_util::io::ReaderStream;

use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use log::{debug, info, warn, error};

use crate::{model, BaseError, parse_xml_bytes};


const READ_BUFFER_SIZE : usize = 4 * 1024 * 1024;

const MAX_PENDING_FILES_PER_DIRECTORY : usize = 512;

const PARSE_PARALLELISM : usize = 8;

// Directories are mostly contiguous in the tarball, but the 2026 snapshot interleaves entries of
// neighbouring directories, and a few entries appear far from the rest of their directory. A directory
// stays open until this many entries pass without seeing it; entries arriving after that are written
// to a separate "late" output (orcid_late_*.avro), so each record is still written exactly once.
const MAX_INTERLEAVE_ENTRIES : u64 = 20_000;

const LATE_ENTRIES_DIR : &str = "late";

const MAX_UPLOAD_ATTEMPTS : u32 = 8;

const UPLOAD_TIMEOUT : Duration = Duration::from_secs(20 * 60);


pub struct TarballOptions {
    pub tarball: String,
    pub dest_bucket: String,
    pub dest_prefix: String,
    pub staging_dir: PathBuf,
    pub max_records_per_file: usize,
    pub concurrent_processing: usize,
    pub concurrent_uploads: usize,
    pub completed_paths: HashSet<String>,
}

struct Context {
    gcp_client: GoogleRestApi,
    schema: &'static Schema,
    dest_bucket: String,
    dest_prefix: String,
    staging_dir: PathBuf,
    max_records_per_file: usize,
    upload_slots: Arc<Semaphore>,
    failed: AtomicBool,
}

type XmlEntry = (String, Vec<u8>);

struct OpenDirectory {
    // None when the directory was completed by a previous run and is being skipped.
    sender: Option<mpsc::Sender<XmlEntry>>,
    last_seen: u64,
}

type AvroFileWriter = AvroWriter<'static, Compat<tokio::fs::File>>;


/// Key recorded in the "Completed processing prefix" log line, and matched against the history file.
fn history_key(tarball: &str, dir: &str) -> String {
    format!("{}#{}", tarball, dir)
}

pub async fn process_tarball(opts: TarballOptions, gcp_client: GoogleRestApi, schema: &'static Schema) -> Result<usize, BaseError> {

    tokio::fs::create_dir_all(&opts.staging_dir).await?;

    let ctx = Arc::new(Context {
        gcp_client,
        schema,
        dest_bucket: opts.dest_bucket,
        dest_prefix: opts.dest_prefix,
        staging_dir: opts.staging_dir,
        max_records_per_file: opts.max_records_per_file,
        upload_slots: Arc::new(Semaphore::new(opts.concurrent_uploads)),
        failed: AtomicBool::new(false),
    });
    let directory_slots = Arc::new(Semaphore::new(opts.concurrent_processing));
    let opts_concurrent_processing = opts.concurrent_processing;
    let tarball = opts.tarball;
    let completed_paths = opts.completed_paths;

    let reader_ctx = ctx.clone();
    let handle = tokio::runtime::Handle::current();

    let reader = tokio::task::spawn_blocking(move || -> Result<Vec<JoinHandle<Result<usize, BaseError>>>, BaseError> {

        let ctx = reader_ctx;
        let file = std::fs::File::open(&tarball)?;
        let gz = flate2::read::GzDecoder::new(BufReader::with_capacity(READ_BUFFER_SIZE, file));
        let mut archive = tar::Archive::new(gz);

        let mut directory_tasks = vec![];
        let mut open_dirs : HashMap<String, OpenDirectory> = HashMap::new();
        let mut closed_dirs : HashSet<String> = HashSet::new();
        let mut index : u64 = 0;
        let mut interleaved_entries : u64 = 0;
        let mut late_entries : u64 = 0;
        let mut late_sender : Option<Option<mpsc::Sender<XmlEntry>>> = None;
        let mut last_dir = String::new();

        for entry in archive.entries()? {

            if ctx.failed.load(Ordering::SeqCst) {
                warn!("stopping tarball read early because a directory failed");
                break;
            }

            let mut entry = entry?;
            if !entry.header().entry_type().is_file() {
                continue;
            }

            let path = entry.path()?.into_owned();
            let is_xml = path.file_name()
                .and_then(|f| f.to_str())
                .map(|f| f.to_ascii_lowercase().ends_with(".xml") && !f.starts_with("._"))
                .unwrap_or(false);
            if !is_xml {
                continue;
            }
            let dir = match path.parent().and_then(|p| p.file_name()).and_then(|d| d.to_str()) {
                Some(dir) => dir.to_owned(),
                None => continue,
            };

            index += 1;

            // Dropping a stale directory's sender closes its channel, letting it finish.
            open_dirs.retain(|name, open| {
                let keep = index - open.last_seen <= MAX_INTERLEAVE_ENTRIES;
                if !keep { closed_dirs.insert(name.clone()); }
                keep
            });

            // Output object names are derived from the directory, so an entry arriving after its directory
            // was closed goes to the late output rather than reopening (and overwriting) that directory.
            if closed_dirs.contains(&dir) {
                late_entries += 1;
                if late_sender.is_none() {
                    let key = history_key(&tarball, LATE_ENTRIES_DIR);
                    late_sender = Some(if completed_paths.contains(&key) {
                        info!("Skipping already completed processing prefix: {}", key);
                        None
                    } else {
                        let permit = handle.block_on(directory_slots.clone().acquire_owned())?;
                        let (sender, receiver) = mpsc::channel::<XmlEntry>(MAX_PENDING_FILES_PER_DIRECTORY);
                        info!("Starting processing at prefix: {}", key);
                        directory_tasks.push(handle.spawn(process_directory(ctx.clone(), LATE_ENTRIES_DIR.to_owned(), key, receiver, permit)));
                        Some(sender)
                    });
                }
                let sender = match late_sender.as_ref().unwrap() {
                    Some(sender) => sender,
                    None => continue,
                };
                let mut xml = Vec::with_capacity(entry.size() as usize);
                entry.read_to_end(&mut xml)?;
                if sender.blocking_send((path.display().to_string(), xml)).is_err() {
                    ctx.failed.store(true, Ordering::SeqCst);
                    break;
                }
                continue;
            }

            if !open_dirs.contains_key(&dir) {
                if open_dirs.values().filter(|o| o.sender.is_some()).count() >= opts_concurrent_processing {
                    return Err(format!("more than {} directories interleaved at once", opts_concurrent_processing).into());
                }

                let key = history_key(&tarball, &dir);
                let sender = if completed_paths.contains(&key) {
                    info!("Skipping already completed processing prefix: {}", key);
                    None
                } else {
                    let permit = handle.block_on(directory_slots.clone().acquire_owned())?;
                    let (sender, receiver) = mpsc::channel::<XmlEntry>(MAX_PENDING_FILES_PER_DIRECTORY);
                    info!("Starting processing at prefix: {}", key);
                    directory_tasks.push(handle.spawn(process_directory(ctx.clone(), dir.clone(), key, receiver, permit)));
                    Some(sender)
                };
                open_dirs.insert(dir.clone(), OpenDirectory { sender, last_seen: index });
            } else if dir != last_dir {
                interleaved_entries += 1;
            }
            last_dir.clone_from(&dir);

            let open = open_dirs.get_mut(&dir).unwrap();
            open.last_seen = index;
            let sender = match open.sender.as_ref() {
                Some(sender) => sender,
                None => continue, // directory already completed in a previous run
            };

            let mut xml = Vec::with_capacity(entry.size() as usize);
            entry.read_to_end(&mut xml)?;

            if sender.blocking_send((path.display().to_string(), xml)).is_err() {
                // The directory task has exited (it failed); the error is reported from that task.
                ctx.failed.store(true, Ordering::SeqCst);
                break;
            }
        }

        info!("Tarball read complete: {} XML entries, {} switches back to an already-open directory, {} late entries", index, interleaved_entries, late_entries);
        Ok(directory_tasks)
    });

    let directory_tasks = reader.await??;

    let mut total_count = 0;
    let mut first_error : Option<BaseError> = None;
    for task in directory_tasks {
        match task.await {
            Ok(Ok(count)) => total_count += count,
            Ok(Err(error)) => { first_error.get_or_insert(error); },
            Err(error) => { first_error.get_or_insert(Box::new(error)); },
        }
    }

    match first_error {
        Some(error) => Err(error),
        None => Ok(total_count),
    }
}

async fn process_directory(
    ctx: Arc<Context>,
    dir: String,
    key: String,
    receiver: mpsc::Receiver<XmlEntry>,
    _directory_slot: OwnedSemaphorePermit,
) -> Result<usize, BaseError> {

    let result = write_directory(&ctx, &dir, receiver).await;
    match &result {
        Ok(file_count) => info!("Completed processing prefix: {} – generated {} files.", key, file_count),
        Err(error) => {
            ctx.failed.store(true, Ordering::SeqCst);
            error!("processing directory failed, caused by: {} [dir='{}']", error, key);
        }
    }
    result
}

async fn write_directory(ctx: &Arc<Context>, dir: &str, receiver: mpsc::Receiver<XmlEntry>) -> Result<usize, BaseError> {

    // Parse in parallel but keep tarball order, so output files are identical across re-runs.
    let entries = stream::unfold(receiver, |mut receiver| async move {
        receiver.recv().await.map(|entry| (entry, receiver))
    });
    let mut records = std::pin::pin!(entries
        .map(|(name, xml)| tokio::task::spawn_blocking(move || parse_xml_bytes(&xml, &name)))
        .buffered(PARSE_PARALLELISM));

    let mut file_count : usize = 0;
    let mut current_file_record_count = 0;
    let mut current : Option<(AvroFileWriter, PathBuf, String, OwnedSemaphorePermit)> = None;
    let mut uploads : Vec<JoinHandle<Result<(), BaseError>>> = vec![];

    while let Some(record) = records.next().await {

        let record : model::Record = match record?? {
            Some(record) => record,
            None => continue,
        };

        if current.is_none() {
            // Waiting for an upload slot here bounds how many Avro files are staged on disk.
            let upload_slot = ctx.upload_slots.clone().acquire_owned().await?;
            file_count += 1;
            let file_name = format!("orcid_{}_{:0>8}.avro", dir, file_count);
            let destination_key = format!("{}/{}", ctx.dest_prefix, file_name);
            let local_path = ctx.staging_dir.join(&file_name);
            let file = tokio::fs::File::create(&local_path).await?;
            current = Some((AvroWriter::new(ctx.schema, file.compat_write()), local_path, destination_key, upload_slot));
        }

        let (writer, _, _, _) = current.as_mut().unwrap();
        writer.append_ser(&record).await?;
        current_file_record_count += 1;

        if current_file_record_count >= ctx.max_records_per_file {
            current_file_record_count = 0;
            let (writer, local_path, destination_key, upload_slot) = current.take().unwrap();
            finish_avro_file(writer).await?;
            uploads.push(tokio::spawn(upload_with_retry(ctx.clone(), local_path, destination_key, upload_slot)));
        }
    }

    if let Some((writer, local_path, destination_key, upload_slot)) = current.take() {
        finish_avro_file(writer).await?;
        uploads.push(tokio::spawn(upload_with_retry(ctx.clone(), local_path, destination_key, upload_slot)));
    }

    for upload in uploads {
        upload.await??;
    }

    Ok(file_count)
}

async fn finish_avro_file(mut writer: AvroFileWriter) -> Result<(), BaseError> {
    writer.flush().await?;
    let mut inner = writer.into_inner().await?;
    inner.close().await?;
    Ok(())
}

enum UploadError {
    Retryable(BaseError),
    Fatal(BaseError),
}

async fn upload_with_retry(ctx: Arc<Context>, local_path: PathBuf, destination_key: String, _upload_slot: OwnedSemaphorePermit) -> Result<(), BaseError> {

    let mut delay = Duration::from_secs(5);

    for attempt in 1..=MAX_UPLOAD_ATTEMPTS {

        let error = match tokio::time::timeout(UPLOAD_TIMEOUT, upload_once(&ctx, &local_path, &destination_key)).await {
            Ok(Ok(())) => {
                tokio::fs::remove_file(&local_path).await?;
                debug!("uploaded gs://{}/{}", ctx.dest_bucket, destination_key);
                return Ok(());
            },
            Ok(Err(UploadError::Fatal(error))) => {
                return Err(format!("upload of '{}' failed permanently: {}", destination_key, error).into());
            },
            Ok(Err(UploadError::Retryable(error))) => error.to_string(),
            Err(_) => format!("timed out after {:?}", UPLOAD_TIMEOUT),
        };

        if attempt == MAX_UPLOAD_ATTEMPTS {
            return Err(format!("upload of '{}' failed after {} attempts: {}", destination_key, attempt, error).into());
        }
        warn!("upload of '{}' failed (attempt {}/{}), retrying in {:?}: {}", destination_key, attempt, MAX_UPLOAD_ATTEMPTS, delay, error);
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(120));
    }

    unreachable!()
}

async fn upload_once(ctx: &Context, local_path: &PathBuf, destination_key: &str) -> Result<(), UploadError> {

    // A fresh config per attempt picks up refreshed credentials.
    let config = ctx.gcp_client.create_google_storage_v1_config().await
        .map_err(|e| UploadError::Retryable(format!("failed to create GCS config: {}", e).into()))?;

    let file = tokio::fs::File::open(local_path).await
        .map_err(|e| UploadError::Fatal(Box::new(e)))?;

    let params = StoragePeriodObjectsPeriodInsertParams {
        bucket: ctx.dest_bucket.clone(),
        name: Some(destination_key.to_owned()),
        ..Default::default()
    };

    match storage_objects_insert_ext_stream(&config, params, Some("application/avro".to_string()), ReaderStream::with_capacity(file, 1024 * 1024)).await {
        Ok(_) => Ok(()),
        Err(GcsError::ResponseError(response)) => {
            let status = response.status.as_u16();
            let error : BaseError = format!("HTTP {}: {}", status, response.content.chars().take(500).collect::<String>()).into();
            // 401 can be an expired token, which the next attempt's config refresh resolves.
            match status {
                401 | 408 | 429 | 500..=599 => Err(UploadError::Retryable(error)),
                _ => Err(UploadError::Fatal(error)),
            }
        },
        Err(error) => Err(UploadError::Retryable(Box::new(error))),
    }
}
