#!/usr/bin/env bash
# Create the destination GCS bucket (idempotent).
# Usage: PROJECT=<gcp-project> BUCKET=<bucket> [LOCATION=us-central1] ./create_bucket.sh
set -euo pipefail
PROJECT="${PROJECT:?set PROJECT}"
BUCKET="${BUCKET:?set BUCKET}"
LOCATION="${LOCATION:-us-central1}"
if gcloud storage buckets describe "gs://$BUCKET" --format='value(location)' 2>/dev/null; then
  echo "bucket gs://$BUCKET already exists"
else
  gcloud storage buckets create "gs://$BUCKET" --project="$PROJECT" \
    --location="$LOCATION" --uniform-bucket-level-access --public-access-prevention
fi
gcloud storage buckets describe "gs://$BUCKET" --format='table(name,location,locationType,storageClass)'
