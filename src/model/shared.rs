use serde::{Serialize, Deserialize};
use avro_schema::*;
use schema_docs::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct OrcidIdentifier {
    #[schema_docs(description="ORCID iD in URI form (preferred).")]
    uri: String,
    #[schema_docs(description="ORCID path (16-character identifier) of the ORCID iD.")]
    path: String,
     #[schema_docs(description="ORCID host (base URL) of the ORCID iD.")]
    host: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct FuzzyDate {
    #[schema_docs(description="Used with the fuzzy-date type. Represents the year of the date.")]
    year: Option<String>, // ?? not always present
    #[schema_docs(description="Used with the fuzzy-date type. Represents the month of the date.")]
    month: Option<String>,
    #[schema_docs(description="Used with the fuzzy-date type. Represents the day of the date.")]
    day: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Source {

    #[serde(rename(deserialize = "source-orcid"))]
    #[schema_docs(description="The ORCID identifier for the individual user (or \
	legacy client application) that created the item. For identifiers of individual \
	users, it may be used to look up ORCID record details via the API.")]
    #[avro_schema(namespace="SourceOrcid")]
    source_orcid: Option<OrcidIdentifier>,
    #[serde(rename(deserialize = "source-client-id"))]
    #[schema_docs(description="The client ID for the client application (Member organization \
    system) that created the item.")]
    #[avro_schema(namespace="SourceClientId")]
    source_client_id: Option<OrcidIdentifier>,
    #[serde(rename(deserialize = "source-name"))]
    #[schema_docs(description="The human-readable name of the client application
	(Member organization system) or individual user that created the
	item. Value for the same person/organization could change over time.
	source-orcid or source-client-id fields are more appropriate for
	disambiguated matching.")]
    source_name: Option<String>,

    // V3
    #[serde(rename(deserialize = "assertion-origin-orcid"))]
    #[schema_docs(description="The ORCID identifier for the individual user (or
	legacy client application) made the assertion. If missing, source is the same as assertion origin.")]
    #[avro_schema(namespace="AssertionOriginOrcid")]
    assertion_origin_orcid: Option<OrcidIdentifier>,
    #[serde(rename(deserialize = "assertion-origin-client-id"))]
    #[schema_docs(description="The client ID for the client application (Member organization system) \
    that made the assertion. If missing, source is the same as assertion origin.")]
    #[avro_schema(namespace="AssertionOriginClientId")]
    assertion_origin_client_id: Option<OrcidIdentifier>,
    #[serde(rename(deserialize = "assertion-origin-name"))]
    #[schema_docs(description="The human-readable name of the client application \
	(Member organization system) or individual user that created the \
	item. Value for the same person/organization could change over time. \
	source-orcid or source-client-id fields are more appropriate for \
	disambiguated matching.")]
    assertion_origin_name: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct WorkTitle {
    #[schema_docs(description="The main name or title of the work. For a \
	spin-off company, include use the company name")]
    title: Option<String>,
    #[schema_docs(description="If the work has a subtitle, include it here.")]
    subtitle: Option<String>,
    #[serde(rename(deserialize = "translated-title"))]
    #[schema_docs(description="The main title of the work or funding translated \
	into another language. The translated language will be included in \
	the language-code attribute.")]
    translated_title: Option<TranslatedTitle>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct TranslatedTitle {
    #[serde(rename(deserialize = "$value"))]
    value: Option<String>,
    #[serde(rename(deserialize = "@language-code"))]
    #[schema_docs(description="A non-empty string that includes a language code to \
	indicate the language used for the string's value.")]
    language_code: Option<String>,
}

// External IDs
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ExternalIDs {

    //#[serde(rename(deserialize = "last-modified-date"))]
    //last_modified_date: Option<&'a str>,

    #[serde(rename(deserialize = "external-id"), default)]
    #[schema_docs(description="Identifiers for items on the ORCID record are recorded using the common \
    external-ids tag, this includes identifiers in the person, works, funding, and peer review sections. \
    The identifier is described in four fields:\
     \n * External id type is required and records the type of identifier (identifiers in the works, funding, \
     and peer review sections must a supported identifiers (https://pub.orcid.org/v3.0/identifiers) \
     \n * External id value is required and contains the identifier itself
     \n External id URL is optional and contains a link the identifier will resolve to
     \n External id relationship is required and indicates if the identifier refers to the item itself (such as a\
     DOI for an article or a person identifier) or is the identifier is for a whole that the item is a part of \
     (such as an ISSN for a journal article, or an ISBN for a book chapter); part of identifiers are not used \
     when grouping items. Identifiers can also be for alternate versions of the work, and can be grouped with \
     self and version of identifiers
    ")]
    identifiers: Vec<ExternalID>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ExternalID {

    #[serde(rename(deserialize = "external-id-type"))]
    #[schema_docs(description="Supported external-id-type values can be found at https://pub.orcid.org/v2.0/identifiers")]
    r#type: String,
    #[serde(rename(deserialize = "external-id-value"))]
    value: Option<String>,
    #[serde(rename(deserialize = "external-id-normalized"))]
    normalized: Option<String>,

    #[serde(rename(deserialize = "external-id-url"))]
    url: Option<String>,
    #[serde(rename(deserialize = "external-id-relationship"))]
    relationship: Option<String>
}
