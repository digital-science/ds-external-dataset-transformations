use serde::{Serialize, Deserialize};
use avro_schema::*;
use schema_docs::*;

use super::shared::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ActivitiesSummary {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "educations"))]
    #[schema_docs(description="The Education section records the educational institution(s) where the researcher has studied. \
    It uses the same fields as the Employment section: department-name and role-title for specific information about \
    the researcher’s field(s) of study and the degree(s) awarded; the common date tags to identify when the \
    researcher was affiliated with this organization; and the organization tags to identify the educational \
    institution. We recommend using the highest level organization when adding an education item (i.e. not \
    listing the organization at the department level) and always including the disambiguated organization \
    information.")]
    #[avro_schema(namespace="Educations")]
    educations: Option<Educations>,

    #[serde(rename(deserialize = "employments"))]
    #[schema_docs(description="The Employment section records where the researcher is or has been employed. \
    It uses the same fields as the Education section: department-name and role-title for specific information \
    about the department the researcher worked in and their role; the common date tags to identify when the \
    researcher was affiliated with this organization; and the organization tags to identify the educational \
    institution. We recommend using the highest level organization when adding an employment item (i.e. not \
    listing the organization at the department level) and always including the disambiguated organization \
    information.")]
    #[avro_schema(namespace="Employments")]
    employments: Option<Employments>,

    #[serde(rename(deserialize = "fundings"))]
    #[schema_docs(description="The Funding section records monetary awards the researcher has received to \
    support their work. As with works and peer review, funding items that share a common external \
    identifier are grouped together under a single activities:group tag, with the display index indicating \
    their order within the group.")]
    #[avro_schema(namespace="Fundings")]
    fundings: Option<Fundings>,

    #[serde(rename(deserialize = "peer-reviews"))]
    #[schema_docs(description="The Peer Review section of the record captures the researcher’s review and \
    service activity. As with funding and works, peer review items are grouped together using \
    the group tag. The group can be a journal, publisher, or other organization and is assigned \
    when the peer review item is added to ORCID. Peer review items can only be added by the API; \
    the user is not able to enter new items in this section.")]
    #[avro_schema(namespace="PeerReviews")]
    peer_reviews: Option<PeerReviews>,

    #[serde(rename(deserialize = "works"))]
    #[schema_docs(description="The works section records the researcher’s professional outputs. As with \
    funding and peer review, works are grouped together by matching external identifiers \
    using the activites:group tag.")]
    #[avro_schema(namespace="Works")]
    works: Option<Works>,

    // V3
    #[serde(rename(deserialize = "invited-positions"))]
    #[schema_docs(description="This section is used for paid or unpaid positions such as visiting \
    lectureships, emeritus professorships, and other invited positions")]
    #[avro_schema(namespace="InvitedPositions")]
    invited_positions: Option<InvitedPositions>,

    #[serde(rename(deserialize = "memberships"))]
    #[schema_docs(description="This is where information about the researcher’s membership of an \
    association or society — paid or gratis — is recorded.")]
    #[avro_schema(namespace="Memberships")]
    memberships: Option<Memberships>,

    #[serde(rename(deserialize = "qualifications"))]
    #[schema_docs(description="In addition to educational qualifications (which are recorded in the \
    Education section), information about other types of qualifications can also be added to ORCID \
    records. This section is where participation in professional and vocational courses, accreditation, \
    certification, and other training is recorded.")]
    #[avro_schema(namespace="Qualifications")]
    qualifications: Option<Qualifications>,

    #[serde(rename(deserialize = "services"))]
    #[schema_docs(description="Any form of service provided by the researcher as part of their professional \
    activities is recorded here. This includes&nbsp;serving on a society or editorial board, \
    review service, extension work, and more.")]
    #[avro_schema(namespace="Services")]
    services: Option<Services>,

    #[serde(rename(deserialize = "research-resources"))]
    #[schema_docs(description="The Research Resources section is for recording the specialist resources \
    that researchers use for their research, and which typically require a specific proposal \
    process or credential to access. This includes infrastructure (such as facilities and \
    buildings), equipment, service, and special collections (digital and physical) used for \
    research purposes.")]
    #[avro_schema(namespace="ResearchResources")]
    research_resources: Option<ResearchResources>
}

// === Education === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Educations {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    groups: Vec<EducationGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct EducationGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "education-summary"))]
    #[avro_schema(namespace="EducationSummary")]
    records: Vec<AffiliationSummary>
}

// === Employment === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Employments {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    groups: Vec<EmploymentGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct EmploymentGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "employment-summary"))]
    #[avro_schema(namespace="EmploymentSummary")]
    records: Vec<AffiliationSummary>
}

// === Funding === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Fundings {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "group"))]
    groups: Vec<FundingGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct FundingGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "funding-summary"))]
    #[avro_schema(namespace="FundingSummary")]
    records: Vec<FundingSummary>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct FundingSummary {

    #[serde(rename(deserialize = "type"))]
    r#type: String,
    #[schema_docs(description="The type of funding awarded, This field is selected from a list \
    containing the following values: Award, Contract, Grant, Salary-award. \
    Award: Peer-reviewed funding providing direct research costs through competitions.\
    Contract: Works commissioned by external public agencies or industry building on research \
    expertise and aimed at deliverables. Include research contracts awarded by federal agencies \
    for both direct and indirect costs, and honoraria.\
    Grant: Peer-reviewed funding providing direct research costs through competitions \
    Salary award: A competitive, peer-reviewed award that is paid as salary to the awardee/faculty \
    member")]
    #[serde(rename(deserialize = "title"))]
    #[schema_docs(description="The title of the funding award")]
    #[avro_schema(namespace="FundingTitle")]
    title: FundingTitle,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    //#[serde(rename(deserialize = "url"))]
    //url: Option<Cow<'a, str>>,

    #[serde(rename(deserialize = "start-date"))]
    #[schema_docs(description="The date the funding began")]
    #[avro_schema(namespace="StartDate")]
    start_date: Option<FuzzyDate>,
    #[serde(rename(deserialize = "end-date"))]
    #[schema_docs(description="The date the funding ended or will end")]
    #[avro_schema(namespace="EndDate")]
    end_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "organization"))]
    #[schema_docs(description="Information about the organization that awarded the funding")]
    organization: Organization,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when the element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct FundingTitle {
    #[serde(rename(deserialize = "title"))]
    #[schema_docs(description="The title of the funding award")]
    title: String,
    #[serde(rename(deserialize = "translated-title"))]
    #[schema_docs(description="The title the funding appears under \
    in another language, the langauge of the translated title is recorded as an attribute")]
    translated_title: Option<TranslatedTitle>,
}

// === Invited Positions === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct InvitedPositions {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    records: Vec<InvitedPositionGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct InvitedPositionGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "invited-position-summary"))]
    #[avro_schema(namespace="InvitedPositionSummary")]
    records: Vec<AffiliationSummary>
}

// === Memberships === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Memberships {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    groups: Vec<MembershipGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct MembershipGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "membership-summary"))]
    #[avro_schema(namespace="MembershipSummary")]
    records: Vec<AffiliationSummary>
}

// === Works === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Works {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "group"))]
    groups: Vec<WorkGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct WorkGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "work-summary"))]
    #[avro_schema(namespace="WorkSummary")]
    records: Vec<WorkSummary>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct WorkSummary {

    #[serde(rename(deserialize = "type"))]
    #[schema_docs(description="Required. The type of work. This field must be \
    selected from https://members.orcid.org/api/resources/work-types")]
    r#type: Option<String>,

    #[serde(rename(deserialize = "title"))]
    #[schema_docs(description="The title of the publication or group under which \
	the work was published.\
	 * If a journal, include the journal title of the work.\
	 * If a book chapter, use the book title.\
	 * If a translation or a manual, use the series title.\
	 * If a dictionary entry, use the dictionary title.\
	 * If a conference poster, abstract or paper, use the conference name.")]
    title: Option<WorkTitle>,

    #[serde(rename(deserialize = "journal-title"))]
    #[schema_docs(description="The name of a larger collection the work belongs to, \
    such as a journal for journal articles or a book for book chapters")]
    journal_title: Option<String>,

    #[serde(rename(deserialize = "publication-date"))]
    #[schema_docs(description="The date the work was completed")]
    #[avro_schema(namespace="PublicationDate")]
    publication_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(rename(deserialize = "url"))]
    #[schema_docs(description="A url linking to the work")]
    url: Option<String>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when the element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// === Peer Reviews === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct PeerReviews {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "group"))]
    groups: Vec<PeerReviewActivityGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct PeerReviewActivityGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "peer-review-group"))]
    #[avro_schema(namespace="PeerReviewGroup")]
    groups: Vec<PeerReviewGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct PeerReviewGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "peer-review-summary"))]
    #[avro_schema(namespace="PeerReviewSummary")]
    records: Vec<PeerReviewSummary>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct PeerReviewSummary {

    #[serde(rename(deserialize = "review-group-id"))]
    #[schema_docs(description="Identifier for the group that this review should be a part of for \
    aggregation purposes. The Group ID should usually be a valid ISSN.")]
    review_group_id: Option<String>,
    #[serde(rename(deserialize = "reviewer-role"))]
    #[schema_docs(description="The role played by a person in their contribution to a review. This field \
    is selected from a list containing the following values: chair, editor, member, organizer, reviewer")]
    reviewer_role: Option<String>,
    #[serde(rename(deserialize = "review-type"))]
    #[schema_docs(description="The kind of review applied to the subject type reviewed. This field is \
    selected from a list containing the following values: evaluation, review")]
    review_type: Option<String>,
    #[serde(rename(deserialize = "review-url"))]
    #[schema_docs(description="A link to a representation of the review on the web. This URL refers to \
    the review itself, NOT to the item that was reviewed.")]
    review_url: Option<String>,

    #[serde(rename(deserialize = "completion-date"))]
    #[avro_schema(namespace="CompletionDate")]
    completion_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "convening-organization"))]
    #[avro_schema(namespace="ConveningOrganization")]
    #[schema_docs(description=" Information about the organization convening the review (journal publisher, \
    conference organizer, funding agency, etc.). The organization identifiers such as RINGGOLD, GRID, ROR, \
    FUNDREF or LEI identifier are not required.")]
    convening_organization: Option<Organization>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when the element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

// === Qualifications === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Qualifications {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    groups: Vec<QualificationsGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct QualificationsGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "qualification-summary"))]
    #[avro_schema(namespace="Records")]
    records: Vec<AffiliationSummary>
}

// === Services === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Services {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "affiliation-group"))]
    groups: Vec<ServicesGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ServicesGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(rename(deserialize = "service-summary"))]
    #[avro_schema(namespace="Records")]
    records: Vec<AffiliationSummary>
}

// === Research Resources === //

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResources {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(default, rename(deserialize = "group"))]
    groups: Vec<ResearchResourceGroup>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResourceGroup {

    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,

    #[serde(default, rename(deserialize = "research-resource-summary"))]
    #[avro_schema(namespace="Records")]
    records: Vec<ResearchResourceSummary>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResourceSummary {

    #[serde(rename(deserialize = "proposal"))]
    proposal: Option<ResearchResourceProposal>,

    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when the element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResourceProposal {

    title: Option<ResearchResourceTitle>,
    #[serde(rename(deserialize = "start-date"))]
    #[schema_docs(description="The date the access started")]
    #[avro_schema(namespace="StartDate")]
    start_date: Option<FuzzyDate>,
    #[serde(rename(deserialize = "end-date"))]
    #[schema_docs(description="The date the access ended or will end")]
    #[avro_schema(namespace="EndDate")]
    end_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "hosts"))]
    hosts: Option<ResearchResourceHosts>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,
    #[serde(rename(deserialize = "url"))]
    #[schema_docs(description="A link to the proposal")]
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResourceTitle {
    #[schema_docs(description="The name of the resource")]
    title: Option<String>,
    #[serde(rename(deserialize = "translated-title"))]
    #[schema_docs(description="The title the proposal appears under in another language,\
    the language of the translated title is recorded as an attribute")]
    translated_title: Option<TranslatedTitle>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct ResearchResourceHosts {
    #[schema_docs(description="The organization that receives and processes resource proposals \
    or requests. Proposal host may or may not be the same as resource host.")]
    #[serde(rename(deserialize = "organization"))]
    organizations: Vec<Organization>
}

// Affiliation Summary
// ===
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct AffiliationSummary {

    #[serde(rename(deserialize = "department-name"))]
    #[schema_docs(description="The department or subdivision of the organization the affiliation is \
    associated with")]
    department_name: Option<String>,

    #[serde(rename(deserialize = "role-title"))]
    #[schema_docs(description="The title given for the position, or the degree earned")]
    role_title: Option<String>,

    #[serde(rename(deserialize = "start-date"))]
    #[schema_docs(description="The date the affiliation began")]
    #[avro_schema(namespace="StartDate")]
    start_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "end-date"))]
    #[schema_docs(description="The date the affiliation ended or will end")]
    #[avro_schema(namespace="EndDate")]
    end_date: Option<FuzzyDate>,

    #[serde(rename(deserialize = "organization"))]
    #[schema_docs(description="Information about the organization the affiliation was with. A ROR, \
    or Crossref Funder Registry organization identifier is required in 3.0+")]
    organization: Option<Organization>,

    #[serde(rename(deserialize = "url"))]
    #[schema_docs(description="A URL to a resource about the affiliation")]
    url: Option<String>,

    #[serde(rename(deserialize = "external-ids"))]
    external_ids: Option<ExternalIDs>,


    source: Option<Source>,
    #[serde(rename(deserialize = "last-modified-date"))]
    #[schema_docs(description = "The date time when the element was last modified.")]
    last_modified_date: Option<String>,
    #[serde(rename(deserialize = "created-date"))]
    #[schema_docs(description = "The date time when the element was created.")]
    created_date: Option<String>,

    #[serde(rename(deserialize = "@visibility"))]
    visibility: Option<String>,
    #[serde(rename(deserialize = "@display-index"))]
    display_index: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct Organization {
    name: String,
    #[serde(rename(deserialize = "address"))]
    #[schema_docs(description="Information about the physical location of the organization the affiliation \
    is with. A city and country are required")]
    address: OrganizationAddress,
    #[serde(rename(deserialize = "disambiguated-organization"))]
    disambiguated_organization: Option<DisambiguatedOrganization>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct OrganizationAddress {
    // Optional because the 2026 snapshot contains empty `<common:address/>` elements.
    city: Option<String>,
    region: Option<String>,
    country: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(AvroSchema, SchemaDocs)]
pub struct DisambiguatedOrganization {
    #[serde(rename(deserialize = "disambiguated-organization-identifier"))]
    identifier: String,
    #[serde(rename(deserialize = "disambiguation-source"))]
    source: String
}
