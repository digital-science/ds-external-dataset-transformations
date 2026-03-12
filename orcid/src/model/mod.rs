use serde::{Serialize, Deserialize};
use avro_schema::*;
use schema_docs::*;

mod activities;
mod person;
mod shared;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Record {

    #[serde(rename(deserialize = "orcid-identifier"))]
    #[schema_docs(description="All ORCID iDs expressed in a record use the same elements: uri, which \
      is the full URI to the record; path, which is the 16-digit ORCID identifier; and host, which is \
      the domain the record is on.")]
    #[avro_schema(namespace="RecordIdentifier")]
    orcid_identifier : shared::OrcidIdentifier,

    #[serde(rename(deserialize = "preferences"))]
    #[schema_docs(description="The preferences section records the language the user selected when viewing \
     their ORCID record. The languages are listed in https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes, \
     with the exception of Chinese: Simplified is zh_CN, and traditional is zh_TW.")]
    #[avro_schema(namespace="Preferences")]
    preferences: Option<Preferences>,

    #[serde(rename(deserialize = "history"))]
     #[schema_docs(description="The history section has information about the creation and current \
     status of the record:")]
    #[avro_schema(namespace="History")]
    history: History,

    #[serde(rename(deserialize = "person"))]
    #[schema_docs(description="Person Details")]
    #[avro_schema(namespace="Person")]
    person: person::Person,

    #[serde(rename(deserialize = "activities-summary"))]
    #[schema_docs(description="The activities sections record information about the researcher’s work \
    and outcomes, enabling them to get recognition for a wide range of contributions and affiliations. \
    There are eleven subsections")]
    #[avro_schema(namespace="ActivitiesSummary")]
    activities: activities::ActivitiesSummary,

    #[serde(skip_deserializing)]
    pub ingest_series_id: String,
    #[serde(skip_deserializing)]
    pub date_imported_gbq: String
}

impl Record {
    pub fn cleanup(&mut self) {
        self.person.cleanup();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Preferences {
    locale: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct History {
    #[serde(rename(deserialize = "creation-method"))]
    #[schema_docs(description="How the record was created.")]
    creation_method: Option<String>,

    #[serde(rename(deserialize = "completion-date"))]
    completion_date: Option<String>,

    #[serde(rename(deserialize = "submission-date"))]
    #[schema_docs(description="The date the record was created by the user if the creation method is Direct, \
    Website, or Member-referred, or if the creation method is API; it is the date the researcher claimed the \
    record")]
    submission_date: Option<String>,

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description="All items on the ORCID record include metadata about when they were created and \
    when they were modified. These dates are hierarchical; the create date for the section is set to when an \
    item was first added to that section, and the last modified date of sections and groups will match the \
    latest last modified date of any item within the section or group. Created and last modified dates will be \
    returned even if the information they refer to is private. The date is presented as year, month, day, \
    followed by the time given in UCT. Information which has not been edited since it was created will have \
    the same create date and last modified date.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "deactivation-date"))]
    deactivation_date: Option<String>,

    #[schema_docs(description="Fields which can be edited by either the API or the user include the ORCID iD \
    and the name of the client which added the information.")]
    source: Option<shared::Source>,

    #[schema_docs(description="Shows that the record has been claimed by the researcher. All records created \
    by the researcher will have this field set to true. A small number of records that were created by an \
    institution for a researcher and that have not been accessed by the researcher will have the claimed \
    status set to false – note that this option was only available for a short period of time and is no \
    longer allowed")]
    claimed: Option<bool>,

    #[serde(rename(deserialize = "verified-email"))]
    #[schema_docs(description="If any email address on the record has been verified, this will be set to true /
    (to verify an email address the user must prove they have access to that email address inbox by following a /
    link sent to them)")]
    verified_email: Option<bool>,

    #[serde(rename(deserialize = "verified-primary-email"))]
    #[schema_docs(description="If the primary email address on the record has been verified")]
    verified_primary_email: Option<bool>
}
