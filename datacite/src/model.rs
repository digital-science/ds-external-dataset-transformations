use serde::{Deserialize, Serialize};
use avro_schema::*;
use schema_docs::*;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{
    empty_vec_is_none,
    empty_vec_is_none_filter_nulls,
    empty_vec_is_none_string_or_integer,
    optional_string_or_integer,
    optional_coerce_f64,
    optional_vec_or_singular,
    optional_bool_or_string
};

// ===

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
#[schema_docs(id = "data-cite-records", display_name = "DataCite Records")]
pub struct DataCiteRecord {
    #[schema_docs(description = "The DataCite DOI (Digital Object Identifier) string.")]
    pub id: OptionalString,

    #[schema_docs(description = "The type of the entity (e.g., \"dois\").")]
    #[serde(rename = "type")]
    #[avro_schema(rename = "type")]
    pub r#type: OptionalString,

    #[schema_docs(description = "The container for all metadata attributes associated with the DOI.")]
    pub attributes: Option<Attributes>,

    #[schema_docs(description = "A container for relationships to other API entities (JSON API spec).")]
    pub relationships: Option<Relationships>
}

impl DataCiteRecord {
    pub fn clean(self) -> Self {
        let relationships = self.relationships.map(|r| r.clean());
        Self { relationships, ..self }
    }
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[schema_docs(description = "An identifier or identifiers other than the primary Identifier (DOI) applied to the resource being registered (e.g., ISSN, ISBN).")]
    #[serde(rename = "alternateIdentifiers")]
    #[avro_schema(rename = "alternateIdentifiers")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub alternate_identifiers: Option<Vec<AlternateIdentifier>>,

    #[schema_docs(description = "The number of citations for this resource as tracked by DataCite Event Data.")]
    #[serde(rename = "citationCount")]
    #[avro_schema(rename = "citationCount")]
    pub citation_count: Option<i64>,

    #[schema_docs(description = "A breakdown of citation counts grouped by year.")]
    #[serde(rename = "citationsOverTime")]
    #[avro_schema(rename = "citationsOverTime")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub citations_over_time: Option<Vec<CitationOverTime>>,

    #[schema_docs(description = "Descriptive information about the container (e.g., journal, book) if the resource is part of a larger work.")]
    pub container: Option<Container>,

    #[schema_docs(description = "The direct URL to the content file (if different from the landing page URL).")]
    #[serde(rename = "contentUrl")]
    #[avro_schema(rename = "contentUrl")]
    #[serde(deserialize_with = "empty_vec_is_none_filter_nulls", default)]
    pub content_url: Option<Vec<String>>,

    #[schema_docs(description = "The institution or person responsible for collecting, managing, distributing, or otherwise contributing to the development of the resource.")]
    #[avro_schema(namespace = "contributors")]
    pub contributors: Vec<Contributor>,

    #[schema_docs(description = "The date and time the DOI was first created in the DataCite system.")]
    pub created: OptionalString,

    #[schema_docs(description = "The main researchers involved in producing the data, or the authors of the publication, in priority order.")]
    #[avro_schema(namespace = "creators")]
    pub creators: Vec<Creator>,

    #[schema_docs(description = "Different dates relevant to the work (e.g., Accepted, Available, Copyrighted).")]
    pub dates: Vec<Date>,

    #[schema_docs(description = "All additional information that does not fit in any of the other categories (e.g., Abstract, Methods).")]
    pub descriptions: Vec<Description>,

    #[schema_docs(description = "The Digital Object Identifier registered for the resource.")]
    pub doi: OptionalString,

    #[schema_docs(description = "The total number of downloads for this resource.")]
    #[serde(rename = "downloadCount")]
    #[avro_schema(rename = "downloadCount")]
    pub download_count: Option<i64>,

    #[schema_docs(description = "A breakdown of download counts grouped by month/year.")]
    #[serde(rename = "downloadsOverTime")]
    #[avro_schema(rename = "downloadsOverTime")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub downloads_over_time: Option<Vec<DownloadOverTime>>,

    #[schema_docs(description = "Technical format of the resource (e.g., file extension or MIME type).")]
    #[serde(deserialize_with = "empty_vec_is_none_filter_nulls", default)]
    pub formats: Option<Vec<String>>,

    #[schema_docs(description = "Information about financial support (funding) for the resource being registered.")]
    #[serde(rename = "fundingReferences")]
    #[avro_schema(rename = "fundingReferences")]
    pub funding_references: Vec<FundingReference>,

    #[schema_docs(description = "Spatial region or named place where the data was gathered or about which the data is focused.")]
    #[serde(rename = "geoLocations")]
    #[avro_schema(rename = "geoLocations")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub geo_locations: Option<Vec<GeoLocation>>,

    #[schema_docs(description = "A list of identifiers associated with the resource (API field).")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub identifiers: Option<Vec<Identifier>>,

    #[schema_docs(description = "Indicates if the DOI is currently in an active state (Findable).")]
    #[serde(rename = "isActive")]
    #[avro_schema(rename = "isActive")]
    #[serde(deserialize_with = "optional_bool_or_string", default)]
    pub is_active: Option<bool>,

    #[schema_docs(description = "The primary language of the resource (RFC 5646 codes).")]
    pub language: OptionalString,

    #[schema_docs(description = "The version of the metadata record in the DataCite system.")]
    #[serde(rename = "metadataVersion")]
    #[avro_schema(rename = "metadataVersion")]
    pub metadata_version: Option<i64>,

    #[schema_docs(description = "The number of parts associated with this resource.")]
    #[serde(rename = "partCount")]
    #[avro_schema(rename = "partCount")]
    pub part_count: Option<i64>,

    #[schema_docs(description = "The number of collections this resource is a part of.")]
    #[serde(rename = "partOfCount")]
    #[avro_schema(rename = "partOfCount")]
    pub part_of_count: Option<i64>,

    #[schema_docs(description = "The DOI prefix (e.g., 10.1234) indicating the allocating member.")]
    pub prefix: OptionalString,

    #[schema_docs(description = "The year when the data was or will be made publicly available.")]
    #[serde(rename = "publicationYear")]
    #[avro_schema(rename = "publicationYear")]
    pub publication_year: Option<i64>,

    #[schema_docs(description = "The publication year or date (API display field).")]
    #[serde(deserialize_with="optional_string_or_integer")]
    pub published: Option<String>,

    #[schema_docs(description = "The entity that holds, archives, publishes, prints, distributes, releases, issues, or produces the resource. This property is used to formulate the citation.")]
    #[serde(deserialize_with="deserialize_publisher")]
    pub publisher: Option<Publisher>,

    #[schema_docs(description = "The reason for a state change (e.g., why a DOI was hidden).")]
    pub reason: OptionalString,

    #[schema_docs(description = "The number of references cited by this resource.")]
    #[serde(rename = "referenceCount")]
    #[avro_schema(rename = "referenceCount")]
    pub reference_count: Option<i64>,

    #[schema_docs(description = "The timestamp when the DOI was registered.")]
    pub registered: OptionalString,

    #[schema_docs(description = "Identifiers of related resources. These must be globally unique identifiers.")]
    #[serde(rename = "relatedIdentifiers")]
    #[avro_schema(rename = "relatedIdentifiers")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub related_identifiers: Option<Vec<RelatedIdentifier>>,

    #[schema_docs(description = "Information about a resource related to the one being registered (e.g. a journal, book, or series).")]
    #[serde(rename = "relatedItems")]
    #[avro_schema(rename = "relatedItems")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub related_items: Option<Vec<RelatedItem>>,

    #[schema_docs(description = "Rights information for this resource (e.g., license or usage rights).")]
    #[serde(rename = "rightsList")]
    #[avro_schema(rename = "rightsList")]
    pub rights_list: Vec<Rights>,

    #[schema_docs(description = "The version of the DataCite Metadata Schema used (e.g., \"https://schema.datacite.org/meta/kernel-4/\").")]
    #[serde(rename = "schemaVersion")]
    #[avro_schema(rename = "schemaVersion")]
    pub schema_version: OptionalString,

    #[schema_docs(description = "Unstructured size information about the resource (e.g., \"15 pages\", \"6 MB\").")]
    #[serde(deserialize_with = "empty_vec_is_none_string_or_integer", default)]
    pub sizes: Option<Vec<String>>,

    #[schema_docs(description = "The source of the metadata (API field).")]
    pub source: OptionalString,

    #[schema_docs(description = "The current state of the DOI (Draft, Registered, or Findable).")]
    pub state: OptionalString,

    #[schema_docs(description = "Subject, keyword, classification code, or key phrase describing the resource.")]
    pub subjects: Vec<Subject>,

    #[schema_docs(description = "The suffix part of the DOI (the unique string after the slash).")]
    pub suffix: OptionalString,

    #[schema_docs(description = "A name or title by which the resource is known.")]
    pub titles: Vec<Title>,

    #[schema_docs(description = "A container for resource type information and export formats.")]
    pub types: Option<Types>,

    #[schema_docs(description = "The timestamp when the metadata was last updated.")]
    pub updated: OptionalString,

    #[schema_docs(description = "The URL the DOI resolves to (the landing page).")]
    pub url: OptionalString,

    #[schema_docs(description = "The version number of the resource.")]
    pub version: OptionalString,

    #[schema_docs(description = "The number of versions associated with this resource.")]
    #[serde(rename = "versionCount")]
    #[avro_schema(rename = "versionCount")]
    pub version_count: Option<i64>,

    #[schema_docs(description = "The number of resources this resource is a version of.")]
    #[serde(rename = "versionOfCount")]
    #[avro_schema(rename = "versionOfCount")]
    pub version_of_count: Option<i64>,

    #[schema_docs(description = "The number of views for this resource landing page.")]
    #[serde(rename = "viewCount")]
    #[avro_schema(rename = "viewCount")]
    pub view_count: Option<i64>,

    #[schema_docs(description = "A breakdown of view counts grouped by month/year.")]
    #[serde(rename = "viewsOverTime")]
    #[avro_schema(rename = "viewsOverTime")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub views_over_time: Option<Vec<ViewOverTime>>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AlternateIdentifier {
    #[schema_docs(description = "The value of the AlternateIdentifier.")]
    #[serde(rename = "alternateIdentifier")]
    #[avro_schema(rename = "alternateIdentifier")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub alternate_identifier: Option<String>,

    #[schema_docs(description = "The type of the AlternateIdentifier (e.g., ISBN, ISSN, ARK).")]
    #[serde(rename = "alternateIdentifierType")]
    #[avro_schema(rename = "alternateIdentifierType")]
    pub alternate_identifier_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CitationOverTime {
    #[schema_docs(description = "The total number of citations for the specific year.")]
    pub total: Option<i64>,

    #[schema_docs(description = "The year of the citation count.")]
    pub year: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[schema_docs(description = "The first page of the resource within the container.")]
    #[serde(rename = "firstPage")]
    #[avro_schema(rename = "firstPage")]
    #[serde(deserialize_with = "deserialize_optional_string_or_number_ignore_array", default)]
    pub first_page: Option<String>,

    #[schema_docs(description = "The unique identifier for the container (e.g., the ISSN of the journal).")]
    pub identifier: OptionalString,

    #[schema_docs(description = "The type of identifier used for the container (e.g., ISSN, ISBN).")]
    #[serde(rename = "identifierType")]
    #[avro_schema(rename = "identifierType")]
    pub identifier_type: OptionalString,

    #[schema_docs(description = "The issue number of the container.")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub issue: Option<String>,

    #[schema_docs(description = "The last page of the resource within the container.")]
    #[serde(rename = "lastPage")]
    #[avro_schema(rename = "lastPage")]
    #[serde(deserialize_with = "deserialize_optional_string_or_number_ignore_array", default)]
    pub last_page: Option<String>,

    #[schema_docs(description = "The title of the container (e.g., Journal Name).")]
    pub title: OptionalString,

    #[schema_docs(description = "The type of the container (e.g., Journal, Series).")]
    #[serde(rename = "type")]
    #[avro_schema(rename = "type")]
    pub r#type: OptionalString,

    #[schema_docs(description = "The volume number of the container.")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub volume: Option<String>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {

    #[schema_docs(description = "The organizational or institutional affiliation of the contributor.")]
    #[serde(deserialize_with = "deserialize_affiliations", default)]
    pub affiliation: Option<Vec<Affiliation>>,

    #[schema_docs(description = "The type of contributor (e.g., ContactPerson, DataCollector).")]
    #[serde(rename = "contributorType")]
    #[avro_schema(rename = "contributorType")]
    pub contributor_type: OptionalString,

    #[schema_docs(description = "The surname or family name of the contributor.")]
    #[serde(rename = "familyName")]
    #[avro_schema(rename = "familyName")]
    pub family_name: OptionalString,

    #[schema_docs(description = "The personal or first name of the contributor.")]
    #[serde(rename = "givenName")]
    #[avro_schema(rename = "givenName")]
    pub given_name: OptionalString,

    #[schema_docs(description = "The language of the contributor's name.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The full name of the contributor.")]
    pub name: OptionalString,

    #[schema_docs(description = "Uniquely identifies an individual or legal entity, according to various schemes (e.g., ORCID).")]
    #[serde(rename = "nameIdentifiers")]
    #[avro_schema(rename = "nameIdentifiers")]
    #[serde(deserialize_with = "optional_vec_or_singular", default)]
    pub name_identifiers: Option<Vec<NameIdentifier>>,

    #[schema_docs(description = "The type of name (Personal or Organizational).")]
    #[serde(rename = "nameType")]
    #[avro_schema(rename = "nameType")]
    pub name_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Creator {
    #[schema_docs(description = "The organizational or institutional affiliation of the creator.")]
    #[serde(deserialize_with = "deserialize_affiliations", default)]
    pub affiliation: Option<Vec<Affiliation>>,

    #[schema_docs(description = "The surname or family name of the creator.")]
    #[serde(rename = "familyName")]
    #[avro_schema(rename = "familyName")]
    pub family_name: OptionalString,

    #[schema_docs(description = "The personal or first name of the creator.")]
    #[serde(rename = "givenName")]
    #[avro_schema(rename = "givenName")]
    pub given_name: OptionalString,

    #[schema_docs(description = "The language of the creator's name.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The full name of the creator.")]
    pub name: OptionalString,

    #[schema_docs(description = "Uniquely identifies the creator (e.g., ORCID).")]
    #[serde(rename = "nameIdentifiers")]
    #[avro_schema(rename = "nameIdentifiers")]
    #[serde(deserialize_with = "optional_vec_or_singular", default)]
    pub name_identifiers: Option<Vec<NameIdentifier>>,

    #[schema_docs(description = "The type of name (Personal or Organizational).")]
    #[serde(rename = "nameType")]
    #[avro_schema(rename = "nameType")]
    pub name_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Affiliation {
    #[schema_docs(description = "The unique identifier of the affiliation (e.g., ROR ID).")]
    #[serde(rename = "affiliationIdentifier")]
    #[avro_schema(rename = "affiliationIdentifier")]
    pub affiliation_identifier: OptionalString,

    #[schema_docs(description = "The name of the scheme used for the affiliation identifier (e.g., ROR).")]
    #[serde(rename = "affiliationIdentifierScheme")]
    #[avro_schema(rename = "affiliationIdentifierScheme")]
    pub affiliation_identifier_scheme: OptionalString,

    #[schema_docs(description = "The name of the affiliation.")]
    pub name: OptionalString,

    #[schema_docs(description = "The URI of the affiliation identifier scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NameIdentifier {
    #[schema_docs(description = "The value of the name identifier.")]
    #[serde(rename = "nameIdentifier")]
    #[avro_schema(rename = "nameIdentifier")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub name_identifier: Option<String>,

    #[schema_docs(description = "The name of the name identifier scheme.")]
    #[serde(rename = "nameIdentifierScheme")]
    #[avro_schema(rename = "nameIdentifierScheme")]
    pub name_identifier_scheme: OptionalString,

    #[schema_docs(description = "The URI of the name identifier scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    #[schema_docs(description = "The date value (YYYY, YYYY-MM, or YYYY-MM-DD).")]
    #[serde(deserialize_with="optional_string_or_integer", default)]
    pub date: Option<String>,

    #[schema_docs(description = "Specific information about the date (e.g., \"published in print\").")]
    #[serde(rename = "dateInformation")]
    #[avro_schema(rename = "dateInformation")]
    pub date_information: OptionalString,

    #[schema_docs(description = "The type of date (e.g., Issued, Created, Updated).")]
    #[serde(rename = "dateType")]
    #[avro_schema(rename = "dateType")]
    pub date_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    #[schema_docs(description = "The content of the description.")]
    #[serde(deserialize_with = "deserialize_optional_string_bool_or_integer", default)]
    pub description: Option<String>,

    #[schema_docs(description = "The type of description (e.g., Abstract, Methods, SeriesInformation).")]
    #[serde(rename = "descriptionType")]
    #[avro_schema(rename = "descriptionType")]
    pub description_type: OptionalString,

    #[schema_docs(description = "The language of the description.")]
    pub lang: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadOverTime {
    #[schema_docs(description = "The total number of downloads for the specific time period.")]
    pub total: Option<i64>,

    #[schema_docs(description = "The year and month of the download count (YYYY-MM).")]
    #[serde(rename = "yearMonth")]
    #[avro_schema(rename = "yearMonth")]
    pub year_month: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FundingReference {
    #[schema_docs(description = "The code assigned by the funder to the grant or award.")]
    #[serde(rename = "awardNumber")]
    #[avro_schema(rename = "awardNumber")]
    pub award_number: OptionalString,

    #[schema_docs(description = "The human-readable title of the award.")]
    #[serde(rename = "awardTitle")]
    #[avro_schema(rename = "awardTitle")]
    pub award_title: OptionalString,

    #[schema_docs(description = "The URI leading to a page with more information about the award.")]
    #[serde(rename = "awardUri")]
    #[avro_schema(rename = "awardUri")]
    #[serde(deserialize_with = "deserialize_optional_string_ignore_bool", default)]
    pub award_uri: Option<String>,

    #[schema_docs(description = "Uniquely identifies the funding entity (e.g., a Crossref Funder ID).")]
    #[serde(rename = "funderIdentifier")]
    #[avro_schema(rename = "funderIdentifier")]
    pub funder_identifier: OptionalString,

    #[schema_docs(description = "The type of funder identifier used.")]
    #[serde(rename = "funderIdentifierType")]
    #[avro_schema(rename = "funderIdentifierType")]
    pub funder_identifier_type: OptionalString,

    #[schema_docs(description = "The name of the funding provider.")]
    #[serde(rename = "funderName")]
    #[avro_schema(rename = "funderName")]
    pub funder_name: OptionalString,

    #[schema_docs(description = "The URI of the funder identifier scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocation {
    #[schema_docs(description = "A rectangular bounding box definition.")]
    #[serde(rename = "geoLocationBox", default)]
    #[avro_schema(rename = "geoLocationBox")]
    pub geo_location_box: Option<GeoLocationBox>,

    #[schema_docs(description = "A description of a geographic location (e.g., city name).")]
    #[serde(rename = "geoLocationPlace")]
    #[avro_schema(rename = "geoLocationPlace")]
    pub geo_location_place: OptionalString,

    #[schema_docs(description = "A specific point location defined by latitude and longitude.")]
    #[serde(rename = "geoLocationPoint", default)]
    #[avro_schema(rename = "geoLocationPoint")]
    pub geo_location_point: Option<GeoLocationPoint>,

    #[schema_docs(description = "A drawn polygon area defined by a set of points.")]
    #[serde(rename = "geoLocationPolygon")]
    #[avro_schema(rename = "geoLocationPolygon")]
    #[serde(deserialize_with = "deserialize_geolocation_polygons", default)]
    pub geo_location_polygon: Option<Vec<GeoLocationPolygon>>
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocationBox {
    #[schema_docs(description = "The easternmost longitude of the box.")]
    #[serde(rename = "eastBoundLongitude")]
    #[avro_schema(rename = "eastBoundLongitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub east_bound_longitude: Option<f64>,

    #[schema_docs(description = "The northernmost latitude of the box.")]
    #[serde(rename = "northBoundLatitude")]
    #[avro_schema(rename = "northBoundLatitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub north_bound_latitude: Option<f64>,

    #[schema_docs(description = "The southernmost latitude of the box.")]
    #[serde(rename = "southBoundLatitude")]
    #[avro_schema(rename = "southBoundLatitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub south_bound_latitude: Option<f64>,

    #[schema_docs(description = "The westernmost longitude of the box.")]
    #[serde(rename = "westBoundLongitude")]
    #[avro_schema(rename = "westBoundLongitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub west_bound_longitude: Option<f64>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocationPoint {
    #[schema_docs(description = "The latitude of the point.")]
    #[serde(rename = "pointLatitude")]
    #[avro_schema(rename = "pointLatitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub point_latitude: Option<f64>,

    #[schema_docs(description = "The longitude of the point.")]
    #[serde(rename = "pointLongitude")]
    #[avro_schema(rename = "pointLongitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub point_longitude: Option<f64>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocationPolygon {
    #[schema_docs(description = "A point within the polygon.")]
    #[serde(rename = "polygonPoint")]
    #[avro_schema(rename = "polygonPoint")]
    pub polygon_point: Option<PolygonPoint>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonPoint {
    #[schema_docs(description = "The latitude of the polygon point.")]
    #[serde(rename = "pointLatitude")]
    #[avro_schema(rename = "pointLatitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub point_latitude: Option<f64>,

    #[schema_docs(description = "The longitude of the polygon point.")]
    #[serde(rename = "pointLongitude")]
    #[avro_schema(rename = "pointLongitude")]
    #[serde(deserialize_with = "optional_coerce_f64", default)]
    pub point_longitude: Option<f64>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    #[schema_docs(description = "The value of the identifier.")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub identifier: Option<String>,

    #[schema_docs(description = "The type of the identifier.")]
    #[serde(rename = "identifierType")]
    #[avro_schema(rename = "identifierType")]
    pub identifier_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    #[schema_docs(description = "The language of the publisher name.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The name of the publisher.")]
    pub name: OptionalString,

    #[schema_docs(description = "The unique identifier for the publisher (e.g., ROR).")]
    #[serde(rename = "publisherIdentifier")]
    #[avro_schema(rename = "publisherIdentifier")]
    pub publisher_identifier: OptionalString,

    #[schema_docs(description = "The name of the scheme used for the publisher identifier.")]
    #[serde(rename = "publisherIdentifierScheme")]
    #[avro_schema(rename = "publisherIdentifierScheme")]
    pub publisher_identifier_scheme: OptionalString,

    #[schema_docs(description = "The URI of the publisher identifier scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedIdentifier {
    #[schema_docs(description = "The related identifier string.")]
    pub id: OptionalString,

    #[schema_docs(description = "The value of the related identifier.")]
    #[serde(rename = "relatedIdentifier")]
    #[avro_schema(rename = "relatedIdentifier")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub related_identifier: Option<String>,

    #[schema_docs(description = "The type of the related identifier (e.g., DOI, URL).")]
    #[serde(rename = "relatedIdentifierType")]
    #[avro_schema(rename = "relatedIdentifierType")]
    pub related_identifier_type: OptionalString,

    #[schema_docs(description = "The name of the scheme if the identifier is a metadata scheme.")]
    #[serde(rename = "relatedMetadataScheme")]
    #[avro_schema(rename = "relatedMetadataScheme")]
    pub related_metadata_scheme: OptionalString,

    #[schema_docs(description = "The relationship type (e.g., Cites, IsCitedBy, IsPartOf).")]
    #[serde(rename = "relationType")]
    #[avro_schema(rename = "relationType")]
    pub relation_type: OptionalString,

    #[schema_docs(description = "The general type of the related resource.")]
    #[serde(rename = "resourceTypeGeneral")]
    #[avro_schema(rename = "resourceTypeGeneral")]
    pub resource_type_general: OptionalString,

    #[schema_docs(description = "The type of the related metadata scheme.")]
    #[serde(rename = "schemeType")]
    #[avro_schema(rename = "schemeType")]
    pub scheme_type: OptionalString,

    #[schema_docs(description = "The URI of the related metadata scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,

    #[schema_docs(description = "The title of the related resource (optional).")]
    pub title: OptionalString,

    #[schema_docs(description = "The resource type of the related resource.")]
    #[serde(rename = "type")]
    #[avro_schema(rename = "type")]
    pub r#type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedItem {
    #[schema_docs(description = "Contributors to the related item.")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub contributors: Option<Vec<RelatedItemContributor>>,

    #[schema_docs(description = "Creators of the related item.")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub creators: Option<Vec<RelatedItemCreator>>,

    #[schema_docs(description = "The edition of the related item.")]
    pub edition: OptionalString,

    #[schema_docs(description = "The first page of the related item.")]
    #[serde(rename = "firstPage")]
    #[avro_schema(rename = "firstPage")]
    #[serde(deserialize_with = "deserialize_optional_string_or_number_ignore_array", default)]
    pub first_page: Option<String>,

    #[schema_docs(description = "The issue number of the related item.")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub issue: Option<String>,

    #[schema_docs(description = "The last page of the related item.")]
    #[serde(rename = "lastPage")]
    #[avro_schema(rename = "lastPage")]
    #[serde(deserialize_with = "deserialize_optional_string_or_number_ignore_array", default)]
    pub last_page: Option<String>,

    #[schema_docs(description = "The number of the related item.")]
    pub number: OptionalString,

    #[schema_docs(description = "The type of number (e.g., article number).")]
    #[serde(rename = "numberType")]
    #[avro_schema(rename = "numberType")]
    pub number_type: OptionalString,

    #[schema_docs(description = "The publication year of the related item.")]
    #[serde(rename = "publicationYear")]
    #[avro_schema(rename = "publicationYear")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub publication_year: Option<String>,

    #[schema_docs(description = "The publisher of the related item.")]
    pub publisher: OptionalString,

    #[schema_docs(description = "The identifier for the related item.")]
    #[serde(rename = "relatedItemIdentifier")]
    #[avro_schema(rename = "relatedItemIdentifier")]
    pub related_item_identifier: Option<RelatedItemIdentifier>,

    #[schema_docs(description = "The general type of the related item (e.g., JournalArticle).")]
    #[serde(rename = "relatedItemType")]
    #[avro_schema(rename = "relatedItemType")]
    pub related_item_type: OptionalString,

    #[schema_docs(description = "Description of the relationship of the resource to the related item (e.g., IsPublishedIn).")]
    #[serde(rename = "relationType")]
    #[avro_schema(rename = "relationType")]
    pub relation_type: OptionalString,

    #[schema_docs(description = "The titles of the related item.")]
    #[serde(deserialize_with = "optional_vec_or_singular", default)]
    pub titles: Option<Vec<RelatedItemTitle>>,

    #[schema_docs(description = "The volume of the related item.")]
    #[serde(deserialize_with = "optional_string_or_integer", default)]
    pub volume: Option<String>
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedItemContributor {
    #[schema_docs(description = "The type of contributor to the related item.")]
    #[serde(rename = "contributorType")]
    #[avro_schema(rename = "contributorType")]
    pub contributor_type: OptionalString,

    #[schema_docs(description = "The family name of the contributor.")]
    #[serde(rename = "familyName")]
    #[avro_schema(rename = "familyName")]
    pub family_name: OptionalString,

    #[schema_docs(description = "The given name of the contributor.")]
    #[serde(rename = "givenName")]
    #[avro_schema(rename = "givenName")]
    pub given_name: OptionalString,

    #[schema_docs(description = "The full name of the contributor.")]
    pub name: OptionalString,

    #[schema_docs(description = "The type of name.")]
    #[serde(rename = "nameType")]
    #[avro_schema(rename = "nameType")]
    pub name_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedItemCreator {
    #[schema_docs(description = "The family name of the creator.")]
    #[serde(rename = "familyName")]
    #[avro_schema(rename = "familyName")]
    pub family_name: OptionalString,

    #[schema_docs(description = "The given name of the creator.")]
    #[serde(rename = "givenName")]
    #[avro_schema(rename = "givenName")]
    pub given_name: OptionalString,

    #[schema_docs(description = "The full name of the creator.")]
    pub name: OptionalString,

    #[schema_docs(description = "The type of name.")]
    #[serde(rename = "nameType")]
    #[avro_schema(rename = "nameType")]
    pub name_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedItemIdentifier {
    #[schema_docs(description = "The identifier value.")]
    #[serde(rename = "relatedItemIdentifier")]
    #[avro_schema(rename = "relatedItemIdentifier")]
    pub related_item_identifier: OptionalString,

    #[schema_docs(description = "The type of identifier (e.g., DOI, ISSN).")]
    #[serde(rename = "relatedItemIdentifierType")]
    #[avro_schema(rename = "relatedItemIdentifierType")]
    pub related_item_identifier_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedItemTitle {
    #[schema_docs(description = "The language of the title.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The title text.")]
    pub title: OptionalString,

    #[schema_docs(description = "The type of title (e.g., TranslatedTitle).")]
    #[serde(rename = "titleType")]
    #[avro_schema(rename = "titleType")]
    pub title_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Rights {
    #[schema_docs(description = "The language of the rights text.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The text stating the rights (e.g., \"CC-BY 4.0\").")]
    pub rights: OptionalString,

    #[schema_docs(description = "A short identifier for the rights (e.g., \"cc-by-4.0\").")]
    #[serde(rename = "rightsIdentifier")]
    #[avro_schema(rename = "rightsIdentifier")]
    pub rights_identifier: OptionalString,

    #[schema_docs(description = "The name of the scheme for the rights identifier (e.g., SPDX).")]
    #[serde(rename = "rightsIdentifierScheme")]
    #[avro_schema(rename = "rightsIdentifierScheme")]
    pub rights_identifier_scheme: OptionalString,

    #[schema_docs(description = "The URI of the license/rights statement.")]
    #[serde(rename = "rightsUri")]
    #[avro_schema(rename = "rightsUri")]
    pub rights_uri: OptionalString,

    #[schema_docs(description = "The URI of the rights identifier scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    #[schema_docs(description = "A subject classification code.")]
    #[serde(rename = "classificationCode")]
    #[avro_schema(rename = "classificationCode")]
    pub classification_code: OptionalString,

    #[schema_docs(description = "The language of the subject.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The URI of the subject scheme.")]
    #[serde(rename = "schemeUri")]
    #[avro_schema(rename = "schemeUri")]
    pub scheme_uri: OptionalString,

    #[schema_docs(description = "The subject or keyword text.")]
    pub subject: OptionalString,

    #[schema_docs(description = "The name of the subject scheme/classification.")]
    #[serde(rename = "subjectScheme")]
    #[avro_schema(rename = "subjectScheme")]
    pub subject_scheme: OptionalString,

    #[schema_docs(description = "The URI of the specific subject value.")]
    #[serde(rename = "valueUri")]
    #[avro_schema(rename = "valueUri")]
    pub value_uri: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    #[schema_docs(description = "The language of the title.")]
    pub lang: OptionalString,

    #[schema_docs(description = "The title text.")]
    pub title: OptionalString,

    #[schema_docs(description = "The type of title (e.g., AlternativeTitle, Subtitle, TranslatedTitle).")]
    #[serde(rename = "titleType")]
    #[avro_schema(rename = "titleType")]
    pub title_type: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Types {
    #[schema_docs(description = "The metadata formatted in BibTeX for citation.")]
    pub bibtex: OptionalString,

    #[schema_docs(description = "The metadata formatted in CiteProc JSON for citation.")]
    pub citeproc: OptionalString,

    #[schema_docs(description = "A description of the resource (free-text).")]
    #[serde(rename = "resourceType")]
    #[avro_schema(rename = "resourceType")]
    pub resource_type: OptionalString,

    #[schema_docs(description = "The general resource type from the controlled list (e.g., Dataset, Software, Image).")]
    #[serde(rename = "resourceTypeGeneral")]
    #[avro_schema(rename = "resourceTypeGeneral")]
    pub resource_type_general: OptionalString,

    #[schema_docs(description = "The metadata formatted in RIS for reference managers.")]
    pub ris: OptionalString,

    #[schema_docs(description = "The metadata formatted as Schema.org JSON-LD.")]
    #[serde(rename = "schemaOrg")]
    #[avro_schema(rename = "schemaOrg")]
    pub schema_org: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ViewOverTime {
    #[schema_docs(description = "The total number of views for the specific period.")]
    pub total: Option<i64>,

    #[schema_docs(description = "The year and month of the view count.")]
    #[serde(rename = "yearMonth")]
    #[avro_schema(rename = "yearMonth")]
    pub year_month: OptionalString,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    #[schema_docs(description = "Relationship to resources that cite this DOI.")]
    #[avro_schema(namespace = "citations")]
    pub citations: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to the DataCite Client (Repository) account that manages this DOI.")]
    #[avro_schema(namespace = "client")]
    pub client: Option<RelationshipSingle>,

    #[schema_docs(description = "Relationship to media objects (e.g., text/plain).")]
    #[avro_schema(namespace = "media")]
    pub media: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to resources that this DOI is a part of (e.g., a collection).")]
    #[serde(rename = "partOf")]
    #[avro_schema(rename = "partOf")]
    #[avro_schema(namespace = "part_of")]
    pub part_of: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to resources that are parts of this DOI.")]
    #[avro_schema(namespace = "parts")]
    pub parts: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to the DataCite Provider (Member) responsible for the Client.")]
    #[avro_schema(namespace = "provider")]
    pub provider: Option<RelationshipSingle>,

    #[schema_docs(description = "Relationship to resources referenced by this DOI.")]
    #[avro_schema(namespace = "references")]
    pub references: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to the resource this DOI is a version of.")]
    #[serde(rename = "versionOf")]
    #[avro_schema(rename = "versionOf")]
    #[avro_schema(namespace = "version_of")]
    pub version_of: Option<RelationshipList>,

    #[schema_docs(description = "Relationship to other versions of this resource.")]
    #[avro_schema(namespace = "versions")]
    pub versions: Option<RelationshipList>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipList {
    #[schema_docs(description = "The array of relationship objects.")]
    #[serde(deserialize_with = "empty_vec_is_none", default)]
    pub data: Option<Vec<RelationshipData>>
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipSingle {
    #[schema_docs(description = "The relationship object.")]
    pub data: Option<RelationshipData>,
}

#[derive(AvroSchema, SchemaDocs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipData {
    #[schema_docs(description = "The ID of the related resource.")]
    pub id: OptionalString,

    #[schema_docs(description = "The type of the entity (e.g., \"dois\", \"clients\", etc.).")]
    #[serde(rename = "type")]
    #[avro_schema(rename = "type")]
    pub r#type: OptionalString,
}

impl Relationships {
    fn clean(mut self) -> Self {

        if let Some(ref c) = self.client {
            match c.data.as_ref() {
                Some(c) if c.is_empty() => {
                    self.client = None;
                }
                Some(_) => {},
                _ => {
                    self.client = None;
                }
            }
        }

        if let Some(ref d) = self.citations {
            if d.is_empty() {
                self.media = None;
            }
        }

        if let Some(ref d) = self.media {
            if d.is_empty() {
                self.media = None;
            }
        }

        if let Some(ref d) = self.part_of {
            if d.is_empty() {
                self.part_of = None;
            }
        }

        if let Some(ref d) = self.parts {
            if d.is_empty() {
                self.parts = None;
            }
        }

        if let Some(ref d) = self.references {
            if d.is_empty() {
                self.references = None;
            }
        }

        if let Some(ref d) = self.version_of {
            if d.is_empty() {
                self.version_of = None;
            }
        }

        if let Some(ref d) = self.versions {
            if d.is_empty() {
                self.versions = None;
            }
        }

        self
    }
}

impl RelationshipList {
    fn is_empty(&self) -> bool {
        self.data.is_none()
    }
}

impl RelationshipData {
    fn is_empty(&self) -> bool {
        self.id.is_none() && self.r#type.is_none()
    }
}

// ===

/// Some affiliations are only a string value or a singular instance of an affiliation
/// rather than a vector, this fixes for those situations.
fn deserialize_affiliations<'de, D>(deserializer: D) -> Result<Option<Vec<Affiliation>>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::Array(array) => {
            let values : Vec<_> = array.into_iter()
                .filter(|v| !v.is_null())
                .collect();

            if values.is_empty() {
                return Ok(None);
            }

            // The contents of the array are all objects, assume they support `Affiliation` data type
            // and convert the JSON value into those.
            let all_objects = values.iter().filter(|v| v.is_object()).count() == values.len();
            if all_objects {
                let affiliations : Vec<Affiliation> = serde_json::from_value(Value::Array(values)).map_err(serde::de::Error::custom)?;
                return Ok(Some(affiliations));
            }

            // Contents are all strings. Map them across to simple affiliations....
            let all_strings = values.iter().filter(|v| v.is_string()).count() == values.len();
            if all_strings {
                let affiliations : Vec<String> = serde_json::from_value(Value::Array(values)).map_err(serde::de::Error::custom)?;
                let mapped_affiliations = affiliations.into_iter().map(|a| Affiliation {
                    affiliation_identifier: None.into(),
                    affiliation_identifier_scheme: None.into(),
                    name: Some(a).into(),
                    scheme_uri: None.into()
                }).collect();
                return Ok(Some(mapped_affiliations));
            }

            Err("unsupported data type for affiliations listing").map_err(serde::de::Error::custom)
        },
        Value::Object(object) => {
            let affiliation : Affiliation = serde_json::from_value(Value::Object(object)).map_err(serde::de::Error::custom)?;
            Ok(Some(vec![affiliation]))
        },
        Value::Null => Ok(None),
        _ => Err("unsupported data type for affiliations listing").map_err(serde::de::Error::custom),
    }
}

/// Some publishers are only a string value rather than a full publisher instance.
/// This deserializer accounts for those situations.
fn deserialize_publisher<'de, D>(deserializer: D) -> Result<Option<Publisher>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::Object(object) => {
            let publisher : Publisher = serde_json::from_value(Value::Object(object)).map_err(serde::de::Error::custom)?;
            Ok(Some(publisher))
        },
        Value::String(publisher) => {
            Ok(Some(Publisher {
                name: Some(publisher).into(),
                lang: None.into(),
                publisher_identifier: None.into(),
                publisher_identifier_scheme: None.into(),
                scheme_uri: None.into()
            }))
        }
        Value::Null => Ok(None),
        _ => Err("unsupported data type for publisher").map_err(serde::de::Error::custom),
    }
}

/// Some publishers are only a string value rather than a full publisher instance.
/// This deserializer accounts for those situations.
fn deserialize_geolocation_polygons<'de, D>(deserializer: D) -> Result<Option<Vec<GeoLocationPolygon>>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::Object(object) => {
            match serde_json::from_value(Value::Object(object)) {
                Ok(polygon) => Ok(Some(vec![polygon])),
                Err(_) => Ok(None)
            }
        },
        Value::Array(array) => {
            if array.is_empty() {
                return Ok(None);
            }
            match array.get(0) {
                Some(Value::Array(_)) => {
                    let result : Vec<Vec<GeoLocationPolygon>> = match serde_json::from_value(Value::Array(array)) {
                        Ok(r) => r,
                        Err(_) => return Ok(None)
                    };
                    let flattened = result.into_iter().flatten().collect();
                    Ok(Some(flattened))
                },
                Some(Value::Object(_)) => {
                    let result = match serde_json::from_value(Value::Array(array)) {
                        Ok(r) => r,
                        Err(_) => return Ok(None)
                    };
                    Ok(Some(result))
                },
                _ => Ok(None)
            }
        }
        Value::Null => Ok(None),
        _ => Ok(None)
    }
}

/// The `awardUri` field can sometimes contain a `false` boolean value rather than a string (for some reason).
/// This helper function cleans that up and treats it as a simple `None` result value.
fn deserialize_optional_string_ignore_bool<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::String(s) if !s.is_empty() => Ok(Some(s)),
        Value::String(_) => Ok(None),
        Value::Bool(false) => Ok(None),
        Value::Null => Ok(None),
        _ => Err("unsupported data type for optional string (ignoring false bool)").map_err(serde::de::Error::custom),
    }
}

fn deserialize_optional_string_or_number_ignore_array<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>
{
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::String(s) if !s.is_empty() => Ok(Some(s)),
        Value::String(_) => Ok(None),
        Value::Number(num) => Ok(Some(num.to_string())),
        Value::Array(array) if array.len() == 1 => {
            match array.into_iter().next() {
                Some(Value::String(s)) => {
                    if s.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(s))
                    }
                },
                Some(Value::Number(num)) => Ok(Some(num.to_string())),
                _ => Ok(None)
            }
        },
        Value::Array(_) => Ok(None),
        Value::Null => Ok(None),
        _ => Err("unsupported data type for optional string (ignoring arrays)").map_err(serde::de::Error::custom),
    }
}

fn deserialize_optional_string_bool_or_integer<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{

    struct DeserializeStringBoolOrIntVisitor;

    impl<'de> serde::de::Visitor<'de> for DeserializeStringBoolOrIntVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(format!("{}", v)))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(format!("{}", v)))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.is_empty() || v == "" {
                Ok(None)
            } else {
                Ok(Some(v.to_owned()))
            }
        }

        fn visit_bool<E>(self, _v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(DeserializeStringBoolOrIntVisitor)
}