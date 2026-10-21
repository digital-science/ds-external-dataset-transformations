# Shared helpers, sourced by the other scripts.
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK_DIR="${WORK_DIR:-$PWD}"   # downloads, staging and logs go under here

file_size() { stat -f%z "$1" 2>/dev/null || stat -c%s "$1" 2>/dev/null || echo 0; }
file_md5() { if command -v md5 >/dev/null; then md5 -q "$1"; else md5sum "$1" | cut -d' ' -f1; fi; }
# Keep a Mac awake for long runs; no-op elsewhere.
keep_awake() { if command -v caffeinate >/dev/null; then caffeinate -i "$@"; else "$@"; fi; }
