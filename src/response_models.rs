use serde::{Deserialize, Serialize};

pub trait PrimaryResponse {}

impl PrimaryResponse for AmendmentsResponse {}
impl PrimaryResponse for AmendmentDetailsResponse {}
impl PrimaryResponse for AmendmentActionsResponse {}
impl PrimaryResponse for AmendmentCosponsorsResponse {}
impl PrimaryResponse for AmendmentAmendmentsResponse {}
impl PrimaryResponse for AmendmentTextVersionsResponse {}
impl PrimaryResponse for BillsResponse {}
impl PrimaryResponse for BillDetailsResponse {}
impl PrimaryResponse for BillActionsResponse {}
impl PrimaryResponse for BillAmendmentsResponse {}
impl PrimaryResponse for BillCommitteesResponse {}
impl PrimaryResponse for BillCosponsorsResponse {}
impl PrimaryResponse for RelatedBillsResponse {}
impl PrimaryResponse for BillSubjectsResponse {}
impl PrimaryResponse for BillSummariesResponse {}
impl PrimaryResponse for BillTextVersionsResponse {}
impl PrimaryResponse for BillTitlesResponse {}
impl PrimaryResponse for SummariesResponse {}
impl PrimaryResponse for LawsResponse {}
impl PrimaryResponse for LawDetailsResponse {}
impl PrimaryResponse for CongressesResponse {}
impl PrimaryResponse for CongressDetailsResponse {}
impl PrimaryResponse for CongressionalRecordResponse {}
impl PrimaryResponse for DailyCongressionalRecordResponse {}
impl PrimaryResponse for ArticlesResponse {}
impl PrimaryResponse for MembersResponse {}
impl PrimaryResponse for MemberDetailsResponse {}
impl PrimaryResponse for NominationsResponse {}
impl PrimaryResponse for NominationDetailsResponse {}
impl PrimaryResponse for TreatiesResponse {}
impl PrimaryResponse for TreatyDetailsResponse {}

/// Response model for the `/amendment` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
}

/// Represents a summary of an amendment.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentSummary {
    pub congress: u32,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub number: String,
    pub purpose: Option<String>,
    #[serde(rename = "type")]
    pub amendment_type: String,
    pub url: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentDetailsResponse {
    pub amendment: AmendmentDetails,
}

/// Detailed information about a specific amendment.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentDetails {
    pub actions: ResourceReference,
    #[serde(rename = "amendedBill")]
    pub amended_bill: Option<AmendedBill>,
    #[serde(rename = "amendmentsToAmendment")]
    pub amendments_to_amendment: Option<ResourceReference>,
    pub chamber: Option<String>,
    pub congress: u32,
    pub cosponsors: Option<CosponsorsReference>,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub number: String,
    #[serde(rename = "proposedDate")]
    pub proposed_date: Option<String>,
    pub purpose: Option<String>,
    pub sponsors: Option<Vec<MemberSummary>>,
    #[serde(rename = "submittedDate")]
    pub submitted_date: Option<String>,
    #[serde(rename = "type")]
    pub amendment_type: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

/// Reference to a resource with a count and URL.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceReference {
    pub count: u32,
    pub url: String,
}

/// Reference to cosponsors with counts and URL.
#[derive(Debug, Serialize, Deserialize)]
pub struct CosponsorsReference {
    pub count: u32,
    #[serde(rename = "countIncludingWithdrawnCosponsors")]
    pub count_including_withdrawn_cosponsors: u32,
    pub url: String,
}

/// Summary information about a member of Congress.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberSummary {
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub url: String,
}

/// Information about the bill that is being amended.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendedBill {
    pub congress: u32,
    pub number: String,
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    pub url: String,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/actions` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentActionsResponse {
    pub actions: Vec<AmendmentAction>,
}

/// Represents an action taken on an amendment.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentAction {
    #[serde(rename = "actionDate")]
    pub action_date: String,
    #[serde(rename = "recordedVotes")]
    pub recorded_votes: Option<Vec<RecordedVote>>,
    #[serde(rename = "sourceSystem")]
    pub source_system: SourceSystem,
    pub text: String,
    #[serde(rename = "type")]
    pub action_type: String,
}

/// Information about a recorded vote.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordedVote {
    pub chamber: String,
    pub congress: u32,
    pub date: String,
    #[serde(rename = "rollNumber")]
    pub roll_number: u32,
    #[serde(rename = "sessionNumber")]
    pub session_number: u32,
    pub url: String,
}

/// Information about the source system of the action.
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSystem {
    pub code: u32,
    pub name: String,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/cosponsors` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentCosponsorsResponse {
    pub cosponsors: Vec<AmendmentCosponsor>,
}

/// Represents a cosponsor of an amendment.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentCosponsor {
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "isOriginalCosponsor")]
    pub is_original_cosponsor: bool,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub party: Option<String>,
    #[serde(rename = "sponsorshipDate")]
    pub sponsorship_date: String,
    pub state: Option<String>,
    pub url: String,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/amendments` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentAmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/text` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentTextVersionsResponse {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<TextVersion>,
}

/// Represents a text version of an amendment.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextVersion {
    pub date: String,
    pub formats: Vec<TextFormat>,
    #[serde(rename = "type")]
    pub text_type: String,
}

/// Represents a format of the text version.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextFormat {
    #[serde(rename = "type")]
    pub format_type: String,
    pub url: String,
}

/// Response model for the `/bill` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillsResponse {
    pub bills: Vec<BillSummary>,
}

/// Represents a summary of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillSummary {
    pub congress: u32,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub number: String,
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
    pub url: String,
}

/// Represents the latest action taken on a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestAction {
    #[serde(rename = "actionDate")]
    pub action_date: String,
    pub text: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillDetailsResponse {
    pub bill: BillDetails,
}

/// Detailed information about a specific bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillDetails {
    pub actions: ResourceReference,
    pub amendments: ResourceReference,
    #[serde(rename = "cboCostEstimates")]
    pub cbo_cost_estimates: Option<Vec<CboCostEstimate>>,
    #[serde(rename = "committeeReports")]
    pub committee_reports: Option<Vec<CommitteeReport>>,
    pub committees: ResourceReference,
    pub congress: u32,
    #[serde(rename = "constitutionalAuthorityStatementText")]
    pub constitutional_authority_statement_text: Option<String>,
    pub cosponsors: CosponsorsReference,
    #[serde(rename = "introducedDate")]
    pub introduced_date: String,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub laws: Option<Vec<LawReference>>,
    pub number: String,
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,
    #[serde(rename = "policyArea")]
    pub policy_area: Option<PolicyArea>,
    #[serde(rename = "relatedBills")]
    pub related_bills: ResourceReference,
    pub sponsors: Vec<MemberSummary>,
    pub subjects: ResourceReference,
    pub summaries: ResourceReference,
    #[serde(rename = "textVersions")]
    pub text_versions: ResourceReference,
    pub title: String,
    pub titles: ResourceReference,
    #[serde(rename = "type")]
    pub bill_type: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
}

/// Represents a Congressional Budget Office cost estimate.
#[derive(Debug, Serialize, Deserialize)]
pub struct CboCostEstimate {
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub title: String,
    pub url: String,
}

/// Represents a committee report reference.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeReport {
    pub citation: String,
    pub url: String,
}

/// Represents a law reference.
#[derive(Debug, Serialize, Deserialize)]
pub struct LawReference {
    pub number: String,
    #[serde(rename = "type")]
    pub law_type: String,
}

/// Represents the policy area of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyArea {
    pub name: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/actions` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillActionsResponse {
    pub actions: Vec<BillAction>,
}

/// Represents an action taken on a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillAction {
    #[serde(rename = "actionCode")]
    pub action_code: Option<String>,
    #[serde(rename = "actionDate")]
    pub action_date: String,
    #[serde(rename = "actionTime")]
    pub action_time: Option<String>,
    #[serde(rename = "sourceSystem")]
    pub source_system: SourceSystem,
    pub text: String,
    #[serde(rename = "type")]
    pub action_type: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/amendments` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillAmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/committees` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillCommitteesResponse {
    pub committees: Vec<Committee>,
}

/// Represents a committee associated with a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct Committee {
    pub activities: Option<Vec<CommitteeActivity>>,
    pub chamber: String,
    pub name: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    #[serde(rename = "type")]
    pub committee_type: String,
    pub url: String,
}

/// Represents an activity of a committee.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeActivity {
    pub date: Option<String>,
    pub name: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/cosponsors` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillCosponsorsResponse {
    pub cosponsors: Vec<BillCosponsor>,
}

/// Represents a cosponsor of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillCosponsor {
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,
    pub district: Option<u32>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "isOriginalCosponsor")]
    pub is_original_cosponsor: bool,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    pub party: Option<String>,
    #[serde(rename = "sponsorshipDate")]
    pub sponsorship_date: String,
    pub state: Option<String>,
    pub url: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/relatedbills` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedBillsResponse {
    #[serde(rename = "relatedBills")]
    pub related_bills: Vec<RelatedBill>,
}

/// Represents a related bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedBill {
    pub congress: u32,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub number: String,
    #[serde(rename = "relationshipDetails")]
    pub relationship_details: Vec<RelationshipDetail>,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    pub url: String,
}

/// Represents the details of a relationship between bills.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipDetail {
    #[serde(rename = "identifiedBy")]
    pub identified_by: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/subjects` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillSubjectsResponse {
    pub subjects: Subjects,
}

/// Represents the subjects of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct Subjects {
    #[serde(rename = "legislativeSubjects")]
    pub legislative_subjects: Vec<LegislativeSubject>,
    #[serde(rename = "policyArea")]
    pub policy_area: PolicyArea,
}

/// Represents a legislative subject.
#[derive(Debug, Serialize, Deserialize)]
pub struct LegislativeSubject {
    pub name: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/summaries` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillSummariesResponse {
    pub summaries: Vec<BillSummaryItem>,
}

/// Represents a summary of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillSummaryItem {
    #[serde(rename = "actionDate")]
    pub action_date: String,
    #[serde(rename = "actionDesc")]
    pub action_desc: String,
    pub text: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(rename = "versionCode")]
    pub version_code: String,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/text` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillTextVersionsResponse {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<TextVersion>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/titles` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillTitlesResponse {
    pub titles: Vec<BillTitle>,
}

/// Represents a title of a bill.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillTitle {
    pub title: String,
    #[serde(rename = "titleType")]
    pub title_type: String,
    #[serde(rename = "titleTypeCode")]
    pub title_type_code: Option<u32>,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(rename = "billTextVersionCode")]
    pub bill_text_version_code: Option<String>,
    #[serde(rename = "billTextVersionName")]
    pub bill_text_version_name: Option<String>,
    #[serde(rename = "chamberCode")]
    pub chamber_code: Option<String>,
    #[serde(rename = "chamberName")]
    pub chamber_name: Option<String>,
}

/// Response model for the `/summaries` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummariesResponse {
    pub summaries: Vec<SummaryItem>,
}

/// Represents a summary item in the summaries response.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryItem {
    #[serde(rename = "actionDate")]
    pub action_date: String,
    #[serde(rename = "actionDesc")]
    pub action_desc: String,
    pub bill: BillReference,
    #[serde(rename = "currentChamber")]
    pub current_chamber: String,
    #[serde(rename = "currentChamberCode")]
    pub current_chamber_code: String,
    #[serde(rename = "lastSummaryUpdateDate")]
    pub last_summary_update_date: Option<String>,
    pub text: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "versionCode")]
    pub version_code: String,
}

/// Represents a reference to a bill within a summary.
#[derive(Debug, Serialize, Deserialize)]
pub struct BillReference {
    pub congress: u32,
    pub number: String,
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
    pub url: String,
}

/// Response model for the `/law/{congress}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct LawsResponse {
    pub bills: Vec<LawSummary>,
}

/// Represents a summary of a law (bill that became a law).
#[derive(Debug, Serialize, Deserialize)]
pub struct LawSummary {
    pub congress: u32,
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    pub laws: Vec<LawReference>,
    pub number: String,
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,
    pub title: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
    pub url: String,
}

/// Response model for the `/law/{congress}/{lawType}/{lawNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct LawDetailsResponse {
    pub bill: BillDetails,
}

/// Response model for the `/congress` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct CongressesResponse {
    pub congresses: Vec<CongressSummary>,
}

/// Represents a summary of a congress session.
#[derive(Debug, Serialize, Deserialize)]
pub struct CongressSummary {
    #[serde(rename = "endYear")]
    pub end_year: String,
    pub name: String,
    pub sessions: Vec<Session>,
    #[serde(rename = "startYear")]
    pub start_year: String,
}

/// Represents a session within a congress.
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub chamber: String,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub number: u32,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "type")]
    pub session_type: String,
}

/// Response model for the `/congress/{congress}` and `/congress/current` endpoints.
#[derive(Debug, Serialize, Deserialize)]
pub struct CongressDetailsResponse {
    pub congress: CongressDetails,
}

/// Detailed information about a specific congress.
#[derive(Debug, Serialize, Deserialize)]
pub struct CongressDetails {
    #[serde(rename = "endYear")]
    pub end_year: String,
    pub name: String,
    pub number: u32,
    pub sessions: Vec<Session>,
    #[serde(rename = "startYear")]
    pub start_year: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    pub url: Option<String>,
}

/// Response model for the `/congressional-record` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct CongressionalRecordResponse {
    pub results: Results,
}

/// Container for Congressional Record issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    pub issues: Vec<Issue>,
}

/// Represents an individual Congressional Record issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub congress: u32,
    pub issue: u32,
    pub links: Links,
    #[serde(rename = "publishDate")]
    pub publish_date: String,
    pub session: u32,
    pub volume: u32,
}

/// Container for links to the individual sections of the issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub digest: Option<Section>,
    pub remarks: Option<Section>,
    pub house: Option<Section>,
    pub senate: Option<Section>,
    #[serde(rename = "fullRecord")]
    pub full_record: Option<Section>,
}

/// Represents a section of the Congressional Record issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub label: String,
    pub ordinal: u32,
    pub pdf: Pdf,
}

/// Container for the PDF text format for the section.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pdf {
    pub items: Vec<PdfItem>,
}

/// Represents an individual PDF text format for the section.
#[derive(Debug, Serialize, Deserialize)]
pub struct PdfItem {
    pub part: u32,
    pub url: String,
}

/// Response model for the `/daily-congressional-record` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyCongressionalRecordResponse {
    pub daily_congressional_record: DailyCongressionalRecord,
}

/// Container for daily Congressional Record issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyCongressionalRecord {
    pub issues: Vec<DailyIssue>,
}

/// Represents an individual daily Congressional Record issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyIssue {
    #[serde(rename = "issueNumber")]
    pub issue_number: u32,
    #[serde(rename = "volumeNumber")]
    pub volume_number: u32,
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    pub congress: u32,
    #[serde(rename = "sessionNumber")]
    pub session_number: u32,
    pub url: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "fullIssue")]
    pub full_issue: FullIssue,
}

/// Container for full issue, sections, and articles.
#[derive(Debug, Serialize, Deserialize)]
pub struct FullIssue {
    #[serde(rename = "entireIssue")]
    pub entire_issue: Option<EntireIssue>,
    pub sections: Option<Sections>,
    pub articles: Option<Articles>,
}

/// Represents the entire issue items.
#[derive(Debug, Serialize, Deserialize)]
pub struct EntireIssue {
    pub items: Vec<EntireIssueItem>,
}

/// Represents an entire issue item.
#[derive(Debug, Serialize, Deserialize)]
pub struct EntireIssueItem {
    pub part: u32,
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
}

/// Container for sections in the issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sections {
    pub items: Vec<SectionItem>,
}

/// Represents a section item of the daily Congressional Record issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionItem {
    pub name: String,
    #[serde(rename = "startPage")]
    pub start_page: String,
    #[serde(rename = "endPage")]
    pub end_page: String,
    pub text: SectionText,
}

/// Container for section text items.
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionText {
    pub items: Vec<SectionTextItem>,
}

/// Represents a section text item.
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionTextItem {
    pub part: u32,
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
}

/// Container for articles in the issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
    pub count: u32,
    pub url: String,
}

/// Response model for the `/daily-congressional-record/{volumeNumber}/{issueNumber}/articles` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
}

/// Represents an individual article in a section.
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    #[serde(rename = "startPage")]
    pub start_page: String,
    #[serde(rename = "endPage")]
    pub end_page: String,
    pub text: ArticleText,
}

/// Container for article text items.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleText {
    pub items: Vec<ArticleTextItem>,
}

/// Represents an article text item.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleTextItem {
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
}

/// Response model for the `/member` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<Member>,
}

/// Represents an individual member's entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,
    pub state: String,
    #[serde(rename = "partyName")]
    pub party_name: String,
    pub district: Option<u32>,
    pub name: String,
    pub terms: Terms,
    pub depiction: Option<Depiction>,
}

/// Represents a member's terms of service.
#[derive(Debug, Serialize, Deserialize)]
pub struct Terms {
    pub item: Vec<Term>,
}

/// Represents an individual term of service for a member.
#[derive(Debug, Serialize, Deserialize)]
pub struct Term {
    pub chamber: String,
    #[serde(rename = "startYear")]
    pub start_year: u32,
    #[serde(rename = "endYear")]
    pub end_year: Option<u32>,
}

/// Represents the member's current official portrait.
#[derive(Debug, Serialize, Deserialize)]
pub struct Depiction {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub attribution: String,
}

/// Response model for the `/member/{bioguideId}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDetailsResponse {
    pub member: MemberDetails,
}

/// Represents detailed information about an individual member.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberDetails {
    #[serde(rename = "currentMember")]
    pub current_member: bool,
    #[serde(rename = "birthYear")]
    pub birth_year: u32,
    #[serde(rename = "deathYear")]
    pub death_year: Option<u32>,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub depiction: Option<Depiction>,
    pub terms: Vec<MemberTerm>,
    #[serde(rename = "officialUrl")]
    pub official_url: Option<String>,
    #[serde(rename = "honorificName")]
    pub honorific_name: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "suffixName")]
    pub suffix_name: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    #[serde(rename = "directOrderName")]
    pub direct_order_name: String,
    #[serde(rename = "invertedOrderName")]
    pub inverted_order_name: String,
    #[serde(rename = "addressInformation")]
    pub address_information: Option<AddressInformation>,
    pub leadership: Option<Vec<LeadershipPosition>>,
    #[serde(rename = "sponsoredLegislation")]
    pub sponsored_legislation: LegislationReference,
    #[serde(rename = "cosponsoredLegislation")]
    pub cosponsored_legislation: LegislationReference,
    #[serde(rename = "updateDate")]
    pub update_date_member: String,
}

/// Represents a member's term of service in a Congress.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberTerm {
    #[serde(rename = "memberType")]
    pub member_type: String,
    pub congress: u32,
    pub chamber: String,
    #[serde(rename = "stateCode")]
    pub state_code: String,
    #[serde(rename = "stateName")]
    pub state_name: String,
    #[serde(rename = "partyName")]
    pub party_name: String,
    #[serde(rename = "partyCode")]
    pub party_code: String,
    #[serde(rename = "startYear")]
    pub start_year: u32,
    #[serde(rename = "endYear")]
    pub end_year: u32,
    pub district: Option<u32>,
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,
    pub party: String,
    pub state: String,
    #[serde(rename = "officialUrl")]
    pub official_url: Option<String>,
}

/// Represents a member's contact information.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInformation {
    #[serde(rename = "officeAddress")]
    pub office_address: String,
    pub city: String,
    pub district: String,
    #[serde(rename = "zipCode")]
    pub zip_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}

/// Represents a leadership position held by the member.
#[derive(Debug, Serialize, Deserialize)]
pub struct LeadershipPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    pub congress: u32,
    pub current: bool,
}

/// Represents a reference to a member's sponsored or cosponsored legislation.
#[derive(Debug, Serialize, Deserialize)]
pub struct LegislationReference {
    pub count: u32,
    pub url: String,
}

/// Response model for the `/nomination` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct NominationsResponse {
    pub nominations: Vec<NominationItem>,
}

/// Represents an individual nomination entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct NominationItem {
    pub congress: u32,
    pub number: u32,
    #[serde(rename = "partNumber")]
    pub part_number: Option<u32>,
    pub citation: String,
    pub description: String,
    #[serde(rename = "receivedDate")]
    pub received_date: String,
    #[serde(rename = "nominationType")]
    pub nomination_type: NominationType,
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,
    pub url: String,
    pub organization: String,
}

/// Represents the type of nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct NominationType {
    #[serde(rename = "isCivilian")]
    pub is_civilian: bool,
    #[serde(rename = "isMilitary")]
    pub is_military: bool,
}

/// Response model for the `/nomination/{congress}/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct NominationDetailsResponse {
    pub nomination: NominationDetails,
}

/// Represents detailed information about a nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct NominationDetails {
    pub congress: u32,
    pub number: u32,
    #[serde(rename = "partNumber")]
    pub part_number: Option<u32>,
    pub citation: String,
    #[serde(rename = "isPrivileged")]
    pub is_privileged: bool,
    #[serde(rename = "isList")]
    pub is_list: bool,
    #[serde(rename = "receivedDate")]
    pub received_date: String,
    pub description: String,
    #[serde(rename = "executiveCalendarNumber")]
    pub executive_calendar_number: Option<String>,
    #[serde(rename = "authorityDate")]
    pub authority_date: Option<String>,
    pub nominees: Vec<Nominee>,
    pub committees: Option<CommitteesReference>,
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,
    pub actions: Option<ActionsReference>,
    pub hearings: Option<HearingsReference>,
    #[serde(rename = "updateDate")]
    pub update_date: String,
}

/// Represents an individual nominee position within a nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct Nominee {
    pub ordinal: u32,
    #[serde(rename = "introText")]
    pub intro_text: Option<String>,
    pub organization: String,
    #[serde(rename = "positionTitle")]
    pub position_title: String,
    pub division: Option<String>,
    pub url: String,
    #[serde(rename = "nomineeCount")]
    pub nominee_count: u32,
}

/// Represents a reference to committees associated with the nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteesReference {
    pub count: u32,
    pub url: String,
}

/// Represents a reference to actions taken on the nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionsReference {
    pub count: u32,
    pub url: String,
}

/// Represents a reference to printed hearings associated with the nomination.
#[derive(Debug, Serialize, Deserialize)]
pub struct HearingsReference {
    pub count: u32,
    pub url: String,
}

/// Response model for the `/treaty` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreatiesResponse {
    pub treaties: Vec<TreatyItem>,
}

/// Represents an individual treaty entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreatyItem {
    #[serde(rename = "congressReceived")]
    pub congress_received: u32,
    #[serde(rename = "congressConsidered")]
    pub congress_considered: u32,
    pub number: u32,
    pub suffix: Option<String>,
    #[serde(rename = "transmittedDate")]
    pub transmitted_date: String,
    #[serde(rename = "resolutionText")]
    pub resolution_text: String,
    pub topic: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub parts: Option<TreatyParts>,
}

/// Represents parts of a treaty.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreatyParts {
    pub count: u32,
    pub urls: Vec<String>,
}

/// Response model for the `/treaty/{congress}/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreatyDetailsResponse {
    pub treaty: TreatyDetails,
}

/// Represents detailed information about a treaty.
#[derive(Debug, Serialize, Deserialize)]
pub struct TreatyDetails {
    #[serde(rename = "congressReceived")]
    pub congress_received: u32,
    #[serde(rename = "congressConsidered")]
    pub congress_considered: u32,
    pub number: u32,
    pub suffix: Option<String>,
    #[serde(rename = "transmittedDate")]
    pub transmitted_date: String,
    #[serde(rename = "inForceDate")]
    pub in_force_date: Option<String>,
    #[serde(rename = "resolutionText")]
    pub resolution_text: String,
    #[serde(rename = "countriesParties")]
    pub countries_parties: Vec<CountryParty>,
    #[serde(rename = "indexTerms")]
    pub index_terms: Vec<IndexTerm>,
    #[serde(rename = "relatedDocs")]
    pub related_docs: Vec<RelatedDoc>,
    pub topic: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub parts: Option<TreatyParts>,
}

/// Represents a country or party associated with the treaty.
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryParty {
    pub name: String,
    #[serde(rename = "oldNumber")]
    pub old_number: Option<String>,
    #[serde(rename = "oldNumberDisplayName")]
    pub old_number_display_name: Option<String>,
}

/// Represents an index term associated with the treaty.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexTerm {
    pub name: String,
}

/// Represents an executive report associated with the treaty.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedDoc {
    pub name: String,
    pub url: String,
}


