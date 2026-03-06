use serde::{Serialize, Deserialize};
use avro_schema::*;
use schema_docs::*;

use super::shared::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Person {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[avro_schema(namespace="Name")]
    #[schema_docs(description = "The given-names, family-name and credit-name are the names the researcher \
    goes by, and can only be edited by the researcher (which is why no source information is returned in \
    this section). While given-names is a required field on all records, this section can be set to \
    private by the researcher.")]
    name: Option<Name>,

    #[serde(rename(deserialize = "other-names"))]
    #[schema_docs(description = "Other names is a repeatable field to capture other names the researcher \
    may be known by. In this example, the record has three other names, all of which were added by the \
    researcher. As with all sections in the record, there is a last-modified-date which matches the latest \
    last modified date of the most recently edited item in the section.")]
    #[avro_schema(namespace="OtherNames")]
    other_names: Option<OtherNames>,

    #[avro_schema(namespace="Biography")]
    #[schema_docs(description = "The biography field is non-repeatable and can only be edited by the \
    researcher. Since only the researcher can edit this field, no source information is included")]
    #[avro_schema(namespace="OtherNames")]
    biography: Option<Biography>,

    #[serde(rename(deserialize = "researcher-urls"))]
    #[avro_schema(namespace="ResearcherUrls")]
    #[schema_docs(description = "Researcher URLs (labeled as websites on the record) link to other webpages \
    about the researcher. Each webpage has a url tag, which is the link to the webpage itself, and an \
    optional url-name tag. In this example, the first website, which has the name 'Wikipedia Page,' has \
    been added by an API client, which is reflected in the source. The display index lets us know the \
    researcher has sorted this website to appear at the top of the websites list on their record, but the \
    researcher has also given this website a limited visibility setting (Trusted Parties) so it is not \
    shown on the public record")]
    researcher_urls: Option<ResearcherUrls>,

    #[serde(rename(deserialize = "emails"))]
    #[schema_docs(description = "Email addresses can only be edited by the researcher. Keep in mind that \
    email addresses default with the visibility setting ‘only me’ and most users do not change that \
    status, so often when reading a record an email address will not be returned.")]
    #[avro_schema(namespace="Emails")]
    emails: Option<Emails>,

    #[serde(rename(deserialize = "addresses"))]
    #[schema_docs(description = "The address field records countries or regions that the researcher works \
    or has worked in. The countries are returned in ISO 3166-1 alpha-2 format. In this example one \
    country has been added by the user and one by a client integration.")]
    #[avro_schema(namespace="Addresses")]
    addresses: Option<Addresses>,

    #[serde(rename(deserialize = "keywords"))]
    #[schema_docs(description = "The keywords sections is for words and phrases that describe the researcher’s \
    areas of study.&nbsp;In this example, there are three keywords: two added by the researcher and one by a \
    client integration. The client integration and the researcher have entered the same keyword, 'QA and testing'\
    , on the public ORCID record. This keyword will only appear once, with both sources listed. Using the API, \
    both keywords and all associated metadata are returned separately.")]
    #[avro_schema(namespace="Keywords")]
    keywords: Option<Keywords>,

    #[serde(rename(deserialize = "external-identifiers"))]
    #[schema_docs(description = "Person external identifiers record identifiers and links to the researcher in \
    other systems. Like all identifiers, the personal external identifier is described using the tags: type, \
    value, url, and relationship. However, personal identifiers should always have the relationship set to \
    'self.' The researcher cannot add external identifiers to the record manually, they must be added by a \
    client integration using the API. In this example, the researcher has not sorted the identifiers added to \
    their record, so the display index on both is '0.'")]
    #[avro_schema(namespace="ExternalIdentifiers")]
    external_identifiers: Option<ExternalIdentifiers>
}

impl Person {
    pub fn cleanup(&mut self) {
        if let Some(on) = self.other_names.as_mut() {
            if on.names.len() == 0 && on.last_modified_date.is_none() {
                self.other_names = None;
            }
        }

        if let Some(b) = self.biography.as_mut() {
            if b.content.is_none() && b.last_modified_date.is_none() {
                self.biography = None;
            }
        }

        if let Some(r) = self.researcher_urls.as_mut() {
            if r.urls.len() == 0 && r.last_modified_date.is_none() {
                self.researcher_urls = None;
            }
        }

        if let Some(r) = self.emails.as_mut() {
            if r.emails.len() == 0 && r.last_modified_date.is_none() {
                self.emails = None;
            }
        }

        if let Some(r) = self.addresses.as_mut() {
            if r.addresses.len() == 0 && r.last_modified_date.is_none() {
                self.addresses = None;
            }
        }

        if let Some(r) = self.keywords.as_mut() {
            if r.keywords.len() == 0 && r.last_modified_date.is_none() {
                self.keywords = None;
            }
        }

        if let Some(r) = self.external_identifiers.as_mut() {
            if r.identifiers.len() == 0 && r.last_modified_date.is_none() {
                self.external_identifiers = None;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Name {
    #[serde(rename(deserialize = "given-names"))]
    #[schema_docs(description = "The given-names, family-name and credit-name are the names the \
    researcher goes by, and can only be edited by the researcher (which is why no source information \
    is returned in this section). While given-names is a required field on all records, this section can \
    be set to private by the researcher.")]
    given_names: Option<String>,
    #[serde(rename(deserialize = "family-name"))]
    #[schema_docs(description = "The given-names, family-name and credit-name are the names the \
    researcher goes by, and can only be edited by the researcher (which is why no source information \
    is returned in this section). While given-names is a required field on all records, this section can \
    be set to private by the researcher.")]
    family_name: Option<String>,
    #[serde(rename(deserialize = "credit-name"))]
    #[schema_docs(description = "The given-names, family-name and credit-name are the names the \
    researcher goes by, and can only be edited by the researcher (which is why no source information \
    is returned in this section). While given-names is a required field on all records, this section can \
    be set to private by the researcher.")]
    credit_name: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    #[schema_docs(description = "Visibility indicates the privacy setting of the item, based on the user’s \
    set visibility preference. If a visibility tag is included when information is posted via the API, it \
    will be ignored. Instead the information will be posted per the user’s default visibility preference as \
    set in their account preferences. \
    Public items can be read by anyone via the API and are shown on the researcher’s public record, \
    Trusted party items are only returned via the API to clients who have read-limited access; they are \
    not on the researcher’s public record. Private items are visible only to the researcher and the \
    source of the item – the only private items returned via the API are those which were added to the \
    record via your integration.
    ")]
    visibility: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>
}

// External Identifiers
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ExternalIdentifiers {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-identifier"), default)]
    #[schema_docs(description = "")]
    identifiers: Vec<ExternalIdentifier>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ExternalIdentifier {

    #[serde(rename(deserialize = "external-id-type"))]
    r#type: String,
    #[serde(rename(deserialize = "external-id-value"))]
    value: String,

    #[serde(rename(deserialize = "external-id-relationship"))]
    relationship: Option<String>,
    #[serde(rename(deserialize = "external-id-url"))]
    url: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// Researcher Urls
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearcherUrls {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "researcher-url"), default)]
    urls: Vec<ResearcherUrl>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearcherUrl {

    #[serde(rename(deserialize = "url-name"))]
    url_name: Option<String>,
    #[serde(rename(deserialize = "url"))]
    url: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// Other Names
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct OtherNames {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "other-name"), default)]
    names: Vec<OtherName>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct OtherName {

    #[serde(rename(deserialize = "content"))]
    content: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// Emails
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Emails {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "email"), default)]
    emails: Vec<Email>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Email {

    #[serde(rename(deserialize = "email"))]
    email: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@verified"))]
    verified: Option<bool>,
    #[serde(rename(deserialize = "@primary"))]
    primary: Option<bool>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// Addresses
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Addresses {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "address"), default)]
    addresses: Vec<Address>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Address {

    #[serde(rename(deserialize = "country"))]
    country: String,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@verified"))]
    verified: Option<bool>,
    #[serde(rename(deserialize = "@primary"))]
    primary: Option<bool>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// Addresses
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Keywords {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "keyword"), default)]
    keywords: Vec<Keyword>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Keyword {

    #[serde(rename(deserialize = "content"))]
    content: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@verified"))]
    verified: Option<bool>,
    #[serde(rename(deserialize = "@primary"))]
    primary: Option<bool>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}


// Biography
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Biography {

    #[serde(rename(deserialize = "content"))]
    content: Option<String>,

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>
}
