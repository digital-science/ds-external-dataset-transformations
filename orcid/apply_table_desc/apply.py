from google.cloud import bigquery
import json
import logging


def run_apply_docs_to_table(bq_client, schema_file_path, table_ref):
    apply_docs_to_table(bq_client, table_ref, schema_file_path)


def apply_docs_to_table(bq_client, table_ref, json_file_path, append=False):

    # From the BQ client, load the table schema.
    table = bq_client.get_table(table_ref)

    if table is None:
        logging.error(("unable to find table to apply documentation updates "
                       f"[table='{table_ref.dataset_id}.{table_ref.table_id}']"))
        raise Exception("Unable to find table to apply documentation updates")

    # Load the JSON schema file which contains the field descriptions.
    with open(json_file_path) as json_file:
        docs_schema = json.load(json_file)

        # Load and resolve (recurisvely) all fields descriptions.
        type_docs = JSONDocsSchema(docs_schema)

        # The schema will remain the same, but each "SchemaField" will be updated with
        # a description if one is present in the JSON docs.
        updated = type_docs.apply_docs_to_table_schema(table.schema, append=append)
        table.schema = updated

        try:
            bq_client.update_table(table, ['schema'])
        except Exception as e:
            logging.error((f"failed to update table schema [error='{e}', "
                           f"table='{table_ref.dataset_id}.{table_ref.table_id}']"))
            raise Exception("Unable to update table to apply documentation updates")

    return True


class JSONDocsSchema:
    def __init__(self, json_type_data, **kwargs):
        desc = json_type_data["desc"]
        fields = desc["fields"] if "fields" in desc else None
        deprecated_fields = desc["deprecated_fields"] if "deprecated_fields" in desc else None

        self.fields = _convert_field_set_to_lookup(fields, deprecated_fields)

        # For each of the attached complex "types" (aka structures), create the same field lookup dictionary
        # for each of these.
        types = json_type_data["types"] if "types" in json_type_data else None
        type_lookup = {}

        if types is not None:
            for t in types:
                if "displayed_record_type_name" in t and t["displayed_record_type_name"] is not None:
                    f = t["fields"] if "fields" in t else None
                    df = t["deprecated_fields"] if "deprecated_fields" in t else None
                    type_lookup[t["displayed_record_type_name"]] = _convert_field_set_to_lookup(f, df)

        self.type_lookup = type_lookup

        for type_name, type_fields in self.type_lookup.items():
            _resolve_referenced_types(type_fields, self.type_lookup)

        _resolve_referenced_types(self.fields, self.type_lookup)

    def apply_docs_to_table_schema(self, table_schema, append=False):
        return self._apply_fields_to_schema_fields(table_schema, self.fields, append=append)

    def _apply_fields_to_schema_fields(self, schema_fields, field_lookup, append=False):
        new_fields = list()

        for f in schema_fields:
            # First check to see of the fields name is defined in our field lookup.
            # If found, apply our description onto it.
            if f.name in field_lookup:
                docs_field = field_lookup[f.name]

                sub_fields = f.fields
                if f.field_type == 'RECORD' and "type_lookup" in docs_field and sub_fields and len(sub_fields):
                    sub_fields = self._apply_fields_to_schema_fields(sub_fields, docs_field["type_lookup"], append=append)
                    if sub_fields is not None:
                        sub_fields = tuple(sub_fields)

                desc = docs_field["description"] if ("description" in docs_field and docs_field["description"]) else None
                if desc is not None:
                    desc = ' '.join(desc.split())
                if append and f.description is not None:
                    if len(f.description) > 0:
                        desc = f"{f.description} {desc}"

                # Recreate the new schema field with the changes from above.
                f = bigquery.SchemaField(name=f.name, field_type=f.field_type, mode=f.mode, description=desc, fields=sub_fields, policy_tags=f.policy_tags)

            new_fields.append(f)

        return new_fields


def _convert_field_set_to_lookup(fields, deprecated_fields):
    """
        Take an array of field and deprecated field descriptions and convert these
        into a single lookup object that resolves field names into the docs schema
        description object.
    """
    lookup = {}

    if deprecated_fields is not None:
        for f in deprecated_fields:
            if "field_key" in f and (("description" in f and f["description"] is not None) or ("refs_entity" in f and f["refs_entity"] is True)):
                lookup[f["field_key"]] = f

    if fields is not None:
        for f in fields:
            if "field_key" in f and (("description" in f and f["description"] is not None) or ("refs_entity" in f and f["refs_entity"] is True)):
                lookup[f["field_key"]] = f

    return lookup


def _resolve_referenced_types(fields, all_types):

    for field_name, field_desc in fields.items():
        if "refs_entity" in field_desc and field_desc["refs_entity"] is True:
            if "field_displayed_type" in field_desc:
                ref_type = field_desc["field_displayed_type"]
                if ref_type is not None:
                    resolved_type = all_types[ref_type] if ref_type in all_types else None
                    if resolved_type is not None:
                        field_desc["type_lookup"] = resolved_type
