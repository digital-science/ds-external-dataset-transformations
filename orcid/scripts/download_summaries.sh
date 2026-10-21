#!/usr/bin/env bash
# Resumable download of an ORCID summaries tarball from figshare, with integrity verification.
# figshare gives an MD5 for published files; private share links (pre-DOI) give none, so the S3 multipart
# ETag can be used instead (from the response headers of a ranged request to the download URL).
#
# Usage: URL=<figshare download url> SIZE=<bytes> (MD5=<md5> | ETAG=<etag>) [OUT=<path>] ./download_summaries.sh
set -euo pipefail
source "$(dirname "$0")/common.sh"
URL="${URL:?set URL, e.g. https://ndownloader.figshare.com/files/<file-id>}"
SIZE="${SIZE:?set SIZE (bytes)}"
MD5="${MD5:-}"
ETAG="${ETAG:-}"
[[ -n "$MD5$ETAG" ]] || { echo "set MD5 or ETAG"; exit 1; }
OUT="${OUT:-$WORK_DIR/data/download/$(basename "${URL%%\?*}").tar.gz}"
mkdir -p "$(dirname "$OUT")"

for attempt in $(seq 1 50); do
  have=$(file_size "$OUT")
  [[ "$have" -ge "$SIZE" ]] && break
  echo "$(date) attempt $attempt, have $have/$SIZE bytes"
  # -C - resumes; figshare redirects to a fresh signed S3 URL on each request.
  curl -L --fail -C - --retry 5 --retry-delay 10 --connect-timeout 30 \
       --speed-limit 10000 --speed-time 120 -sS -o "$OUT" "$URL" || { echo "curl exit $?"; sleep 15; }
done

have=$(file_size "$OUT")
[[ "$have" -eq "$SIZE" ]] || { echo "size mismatch: $have != $SIZE"; exit 1; }
echo "$(date) download complete, verifying $OUT"
if [[ -n "$MD5" ]]; then
  got=$(file_md5 "$OUT")
  [[ "$got" == "$MD5" ]] || { echo "MD5 MISMATCH: $got != $MD5"; exit 1; }
else
  python3 -I "$(dirname "$0")/verify_s3_etag.py" "$OUT" "$ETAG" || { echo "ETAG MISMATCH"; exit 1; }
fi
echo "$(date) VERIFIED OK"
