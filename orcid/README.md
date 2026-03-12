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
