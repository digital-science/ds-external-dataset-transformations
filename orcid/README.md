# ORCID Dataset Generator

Ingests [ORCID](https://orcid.org/) snapshot XML summaries, parses them into structured records, and writes the output as Avro files to Google Cloud Storage. The resulting data can then be loaded into BigQuery for analysis.

## Features

- Parses ORCID snapshot XML files and writes structured Avro files to a GCS bucket.
- Supports concurrent processing of source directories with configurable parallelism.
- Tracks previously completed prefixes via an optional history file to enable resumable runs.
- Generates a JSON schema description file and applies column-level descriptions to a BigQuery table.

## Usage

```bash
cargo build
./target/debug/orcid-snapshot-ingest \
  --source-path <path-to-xml-dir> \
  --destination-bucket <gcs-bucket> \
  --destination-prefix <key-prefix> \
  [--concurrent-processing 32] \
  [--max-records-per-file 10000] \
  [--history-file <path>]
```

### Streaming directly from the tarball

The extracted summaries are ~800 GB, so the `.tar.gz` from figshare can instead be streamed without extracting it.
Avro files are staged locally, uploaded with retries, and deleted once uploaded.

```bash
cargo build --release
RUST_LOG=info ./target/release/orcid-dataset-generator \
  --source-tarball ORCID_2026_10_summaries.tar.gz \
  --destination-bucket <gcs-bucket> \
  --destination-prefix <key-prefix> \
  [--staging-dir ./staging] \
  [--concurrent-processing 8] \
  [--concurrent-uploads 16] \
  [--history-file <log-file>] 2>&1 | tee -a <log-file>
```

- Each top-level directory in the tarball becomes `orcid_<dir>_<n>.avro`. Completed directories are logged as
  `Completed processing prefix: <tarball>#<dir>`, so passing the log back as `--history-file` resumes a failed run.
- Directories are interleaved in the tarball. A directory stays open until 20,000 entries pass without one of its
  files appearing. Any entries that arrive after that go to `orcid_late_<n>.avro` (32 entries in the 2026 snapshot).
- XML files that only contain an ORCID `<error:error>` response (locked, deactivated or deprecated records) are
  skipped, as in directory mode.

### End-to-end pipeline scripts

`scripts/` wraps the full figshare → GCS → BigQuery process. All configuration is through environment variables.
Downloads, staging files and logs go under `WORK_DIR`, which defaults to the current directory.

```bash
# 1. Destination bucket (idempotent)
PROJECT=<gcp-project> BUCKET=<bucket> LOCATION=us-central1 orcid/scripts/create_bucket.sh

# 2. Resumable download + verification. Use MD5 from figshare, or for a private (pre-DOI) share link,
#    the S3 multipart ETag from a ranged request's response headers.
URL='https://ndownloader.figshare.com/files/<file-id>[?private_link=<token>]' SIZE=<bytes> \
  ETAG=<etag> OUT=data/download/ORCID_2026_10_summaries.tar.gz orcid/scripts/download_summaries.sh

# 3. Stream tarball -> Avro -> GCS, retried up to 10 times and resuming from the log (logs/ingest.log)
TARBALL=data/download/ORCID_2026_10_summaries.tar.gz BUCKET=<bucket> DEST_PREFIX=orcid_2026_10/summaries \
  orcid/scripts/run_ingest.sh

# 4. Load into BigQuery: staging load, CTAS dropping ingest_series_id/date_imported_gbq, then column descriptions
GCS_URI='gs://<bucket>/orcid_2026_10/summaries/*.avro' BILLING_PROJECT=<project> \
  STAGING=<project.dataset.staging_table> TARGET=<project.dataset.summaries_2026> orcid/scripts/load_to_bq.sh
```

The 2026 run (31.2M XML files → 3,301 Avro files, 163.6 GB) took about 4h15m over a ~10 MB/s home uplink.
The BigQuery load takes a few minutes.

## Generating & Applying Schema Descriptions

1. Rebuild the JSON description schema file (`schema/schema.json`):
   ```bash
   cargo build --features "schema_docs"
   ./target/debug/orcid-snapshot-ingest
   ```

2. Apply the schema descriptions to a BigQuery table:
   ```bash
   cd apply_table_desc
   python3 main.py \
     --project <billing-project> \
     --dataset <project.dataset> \
     --table <table-name> \
     --schema ../schema/schema.json
   ```
