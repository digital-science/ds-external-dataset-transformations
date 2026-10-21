#!/usr/bin/env bash
# Load ORCID summaries Avro files from GCS into BigQuery with the same layout as previous years' tables
# (verified against summaries_2025: identical fields, types, modes, order and descriptions).
#
#   1. Load the Avro into a staging table (Avro-self-described schema).
#   2. CTAS into the target, dropping the two always-empty columns the Rust model adds
#      (ingest_series_id, date_imported_gbq); query output also makes every field NULLABLE.
#   3. Apply column descriptions from schema/schema.json via apply_table_desc.
#
# Usage: GCS_URI='gs://<bucket>/orcid_2026_10/summaries/*.avro' BILLING_PROJECT=<project> \
#        STAGING=<project.dataset.table> TARGET=<project.dataset.table> ./load_to_bq.sh
# Datasets must be in the US multi-region or the bucket's region. Refuses to overwrite existing tables.
set -euo pipefail
source "$(dirname "$0")/common.sh"
GCS_URI="${GCS_URI:?set GCS_URI, e.g. gs://bucket/prefix/*.avro}"
BILLING_PROJECT="${BILLING_PROJECT:?set BILLING_PROJECT}"
STAGING="${STAGING:?set STAGING (project.dataset.table)}"
TARGET="${TARGET:?set TARGET (project.dataset.table)}"
LOCATION="${LOCATION:-US}"

bq_ref() { echo "${1%%.*}:${1#*.}"; }   # project.dataset.table -> project:dataset.table

for t in "$STAGING" "$TARGET"; do
  if bq show "$(bq_ref "$t")" >/dev/null 2>&1; then
    echo "refusing to continue: $t already exists"; exit 1
  fi
done

echo "1/3 loading $GCS_URI -> $STAGING"
bq --project_id="$BILLING_PROJECT" --location="$LOCATION" load --source_format=AVRO --use_avro_logical_types \
  "$(bq_ref "$STAGING")" "$GCS_URI"

echo "2/3 creating $TARGET"
bq query --project_id="$BILLING_PROJECT" --location="$LOCATION" --use_legacy_sql=false --format=none \
  "CREATE TABLE \`$TARGET\` AS SELECT * EXCEPT(ingest_series_id, date_imported_gbq) FROM \`$STAGING\`"

echo "3/3 applying column descriptions"
( cd "$REPO_DIR/apply_table_desc" && python3 main.py --project "$BILLING_PROJECT" --dataset "${TARGET%.*}" \
    --table "${TARGET##*.}" --schema "$REPO_DIR/schema/schema.json" )

bq show --format=prettyjson "$(bq_ref "$TARGET")" | python3 -c 'import json,sys; d=json.load(sys.stdin); print("rows:", d["numRows"], "bytes:", d["numBytes"])'
echo "done. staging table $STAGING can be deleted once $TARGET is checked."
