#!/usr/bin/env bash
# Stream the ORCID summaries tarball -> Avro -> GCS (no extraction). Re-runnable: the log doubles as the
# history file, so directories already logged as "Completed processing prefix" are skipped on retry.
#
# Usage: TARBALL=<path> BUCKET=<bucket> DEST_PREFIX=orcid_2026_10/summaries ./run_ingest.sh
set -uo pipefail
source "$(dirname "$0")/common.sh"
TARBALL="${TARBALL:?set TARBALL}"
BUCKET="${BUCKET:?set BUCKET}"
DEST_PREFIX="${DEST_PREFIX:?set DEST_PREFIX, e.g. orcid_2026_10/summaries}"
STAGING="${STAGING:-$WORK_DIR/data/staging}"
LOG="${LOG:-$WORK_DIR/logs/ingest.log}"
BIN="$REPO_DIR/target/release/orcid-dataset-generator"
[[ -x "$BIN" ]] || (cd "$REPO_DIR" && cargo build --release) || exit 1
mkdir -p "$(dirname "$LOG")" "$STAGING"; touch "$LOG"
export RUST_LOG="${RUST_LOG:-info}"
for attempt in $(seq 1 "${ATTEMPTS:-10}"); do
  echo "$(date) ingest attempt $attempt" >> "$LOG"
  if keep_awake "$BIN" --source-tarball "$TARBALL" --staging-dir "$STAGING" \
       --destination-bucket "$BUCKET" --destination-prefix "$DEST_PREFIX" \
       --concurrent-processing "${CONCURRENCY:-8}" --concurrent-uploads "${UPLOADS:-16}" \
       --history-file "$LOG" >> "$LOG" 2>&1; then
    echo "$(date) ingest finished OK" >> "$LOG"; exit 0
  fi
  echo "$(date) ingest attempt $attempt failed; retrying in 60s" >> "$LOG"; sleep 60
done
exit 1
