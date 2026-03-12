from google.cloud import bigquery
from apply import run_apply_docs_to_table
import argparse

parser = argparse.ArgumentParser(description="Apply schema descriptions to a BigQuery table.")
parser.add_argument("--project", required=True, help="GCP BigQuery billing project ID")
parser.add_argument("--dataset", required=True, help="Target dataset in the format 'project.dataset' or just 'dataset'")
parser.add_argument("--table", required=True, help="Target table name")
parser.add_argument("--schema", required=True, help="Path to the schema JSON file")
args = parser.parse_args()

dataset_parts = args.dataset.split(".")
if len(dataset_parts) == 2:
    dataset_ref = bigquery.DatasetReference(dataset_parts[0], dataset_parts[1])
else:
    dataset_ref = bigquery.DatasetReference(args.project, dataset_parts[0])

bq_client = bigquery.Client(project=args.project)
table_ref = dataset_ref.table(args.table)

run_apply_docs_to_table(bq_client, args.schema, table_ref)
