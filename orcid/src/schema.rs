use serde::{Serialize, Deserialize};
use serde_json::Value;

use schema_docs::export::{
    Field,
    Description,
    Visibility
};
use schema_docs::types::{
    SchemaRecord,
    SchemaVariant,
    BasicDescriptionContext
};

use std::collections::HashSet;

#[derive(Serialize, Debug)]
pub(crate) struct SchemaDescription {
    pub id: String,
    pub display_name: String,
    pub desc: Value,
    pub notes: Option<String>,
    pub types: Option<Vec<Value>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SchemaRecordDescription {
    record_type_name: String,
    displayed_record_type_name: String,
    notes: Option<String>,
    fields: Vec<Field>,
    deprecated_fields: Option<Vec<Field>>,
    visibility: Option<Visibility>
}

pub(crate) fn generate_schema_description_for_type<T: SchemaRecord>(variant: SchemaVariant) -> SchemaDescription {

    let record_metadata = T::get_metadata();
    let display_name = match &record_metadata.display_name {
        Some(display_name) => display_name.to_owned(),
        None => T::get_type_name()
    };

    let default_tag : Option<String> = None;
    let context = BasicDescriptionContext::new(default_tag);

    let mut schema_desc = convert_base_record_to_schema_description(
        &record_metadata.identifier,
        &display_name,
        T::get_description(variant, &context),
        variant
    );

    if let Value::Object(record_schema_desc) = &mut schema_desc.desc {

        record_schema_desc.insert("id".to_owned(), Value::String(record_metadata.identifier));

        if let Some(dsl_link) = record_metadata.dsl_link {
            record_schema_desc.insert("dsl_link".to_owned(), Value::String(dsl_link));
        }

        if let Some(display_name) = record_metadata.display_name {
            record_schema_desc.insert("display_name".to_owned(), Value::String(display_name));
        }
    }

    schema_desc
}

fn simplify_record_description_no_child_descriptions(record_desc: &Description, variant: SchemaVariant) -> SchemaRecordDescription {

    let mut fields = record_desc.fields.clone();

    fields.sort_by(|a, b| {
        a.field_key.partial_cmp(&b.field_key).unwrap()
    });

    for field in &mut fields {
        field.record_description = None;
    }

    // If this is the external variant we are wanting, we need to strip
    // all internal fields.
    if variant == SchemaVariant::External {
        fields.retain(|r| {
            match r.visibility {
                Some(Visibility::Internal) => false,
                _ => true
            }
        });
    }

    // Extract out any deprecated fields for the record description.
    let mut filtered_fields_deprecated = fields.clone();
    filtered_fields_deprecated.retain(|r| r.deprecated == true);
    let deprecated_fields = if filtered_fields_deprecated.len() > 0 {
        Some(filtered_fields_deprecated)
    } else {
        None
    };

    fields.retain(|r| r.deprecated != true);

    SchemaRecordDescription {
        record_type_name: record_desc.record_type_name.to_owned(),
        displayed_record_type_name: record_desc.displayed_record_type_name.to_owned(),
        notes: record_desc.notes.clone(),
        fields,
        deprecated_fields,
        visibility: record_desc.visibility.clone()
    }
}

fn convert_base_record_to_schema_description(
    record_id: &str,
    display_name: &str,
    record_desc: Description,
    variant: SchemaVariant
) -> SchemaDescription {

    let simple_record_desc = match serde_json::to_value(simplify_record_description_no_child_descriptions(&record_desc, variant)) {
        Ok(record_description) => record_description,
        Err(_error) => {
            panic!("unable to convert record description to JSON representation.");
        }
    };

    let mut schema_desc = SchemaDescription {
        id: record_id.to_owned(),
        display_name: display_name.to_owned(),
        notes: record_desc.notes.clone(),
        desc: simple_record_desc,
        types: None
    };
    let mut types_processed : HashSet<String> = HashSet::new();

    flatten_record_descriptions(&record_desc.fields, &mut schema_desc, &mut types_processed, variant, 0);

    if let Some(ref mut types) = schema_desc.types {
        types.sort_by(|a, b| {
            let a_type_name = match a {
                Value::Object(map) => {
                    match map.get("displayed_record_type_name") {
                        Some(Value::String(type_name)) => Some(type_name.to_owned()),
                        _ => None
                    }
                },
                _ => None
            };
            let b_type_name = match b {
                Value::Object(map) => {
                    match map.get("displayed_record_type_name") {
                        Some(Value::String(type_name)) => Some(type_name.to_owned()),
                        _ => None
                    }
                },
                _ => None
            };

            a_type_name.partial_cmp(&b_type_name).unwrap()
        });
    }

    schema_desc
}

fn flatten_record_descriptions(
    fields: &Vec<Field>,
    schema_desc: &mut SchemaDescription,
    types_processed: &mut HashSet<String>,
    variant: SchemaVariant,
    level: usize
) {

    for field in fields {
        if field.record_description.is_none() {
            continue;
        }

        if types_processed.contains(&field.field_displayed_type) {
            continue;
        }

        let record_desc = field.record_description.as_ref().unwrap();

        let simple_record_desc = match serde_json::to_value(simplify_record_description_no_child_descriptions(record_desc, variant)) {
            Ok(record_description) => record_description,
            Err(_error) => {
                panic!("Unable to convert record description to JSON representation.");
            }
        };

        types_processed.insert(field.field_displayed_type.to_owned());
        if let Some(ref mut types) = schema_desc.types {
            types.push(simple_record_desc);
        } else {
            schema_desc.types = Some(vec![simple_record_desc]);
        }

        flatten_record_descriptions(&record_desc.fields, schema_desc, types_processed, variant, level + 1);
    }
}
