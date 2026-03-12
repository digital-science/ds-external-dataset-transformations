# DataCite Dataset Generator

Parses archived [DataCite](https://datacite.org/) DOI metadata records, transforms them into structured Avro, and supports an external process that writes the output to Google Cloud Storage. The source data follows the [DataCite Metadata Schema](https://schema.datacite.org/) and is available as public archives from locations such as:

- <https://datafiles.datacite.org/datafiles/public-2025>

## Source Data

The archived DataCite files contain JSON records conforming to the DataCite REST API format. Each record wraps a DOI with its full metadata payload, including:

- **Attributes** — core metadata fields such as titles, creators, contributors, descriptions, subjects, dates, publication year, DOI state, rights, funding references, related identifiers, related items, geo-locations, resource types, and usage metrics (citation/download/view counts over time).
- **Relationships** — links to related API entities (citations, references, parts, versions, client, provider, media).

## Schema

The Rust schema (defined in `src/model.rs`) models the complete DataCite record structure using Serde for JSON deserialization and derives an Avro schema via `AvroSchema`. Key types include:

| Struct | Description |
|---|---|
| `DataCiteRecord` | Top-level record: `id`, `type`, `attributes`, and `relationships`. |
| `Attributes` | All metadata fields for a DOI (titles, creators, contributors, dates, descriptions, subjects, rights, related identifiers, geo-locations, types, funding references, usage metrics, etc.). |
| `Creator` / `Contributor` | People or organizations associated with the resource, including affiliations and name identifiers (e.g., ORCID). |
| `RelatedIdentifier` | Links to related resources with relationship types (e.g., Cites, IsPartOf). |
| `RelatedItem` | Extended information about related resources (journal articles, books, etc.). |
| `FundingReference` | Grant/funding metadata (funder name, award number, funder ID). |
| `GeoLocation` | Spatial metadata — points, bounding boxes, and polygons. |
| `Publisher` | Publisher name and identifier. |
| `Relationships` | JSON API–style relationships to other DataCite entities (citations, references, parts, versions, client, provider). |

## Custom Deserializers

The source data contains many inconsistencies (mixed types, null-filled arrays, scalars instead of arrays, etc.). The codebase includes custom Serde deserializers in `src/utils.rs` and `src/datacite.rs` to handle these gracefully:

- `empty_vec_is_none` — collapses empty arrays to `None`.
- `empty_vec_is_none_filter_nulls` — removes null entries from arrays before collapsing.
- `empty_vec_is_none_string_or_integer` — coerces mixed string/integer array elements to strings.
- `optional_string_or_integer` — accepts either a string or integer and returns a string.
- `optional_coerce_f64` — coerces strings, integers, or floats to `f64`.
- `optional_vec_or_singular` — accepts a single object or an array of objects.
- `optional_bool_or_string` — accepts a boolean or a string representation of a boolean.
- `deserialize_affiliations` — handles affiliations provided as strings, objects, or mixed arrays.
- `deserialize_publisher` — handles publishers provided as a plain string or a full object.
- `deserialize_geolocation_polygons` — normalizes nested/flat polygon array structures.

