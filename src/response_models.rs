//! # `response_modes` Module
//! 
//! This module defines the response models used for parsing API responses from various endpoints
//! of the US Congress API. It includes a combination of enums, structs, and traits to handle
//! different types of responses in a type-safe and structured manner.
//! 
//! ## Traits
//! 
//! - **`PrimaryResponse`**: A marker trait implemented by all primary response types.
//! 
//! ## Enums
//! 
//! - **`GenericResponseModel`**: An untagged enum that can represent multiple response types.
//! 
//! ## Structs
//! 
//! - **`GenericResponse`**: A catch-all response model for handling diverse response structures.
//! - **Response Models**: Specific structs for each API endpoint, such as:
//!   - `AmendmentsResponse`, `BillDetailsResponse`, `MemberDetailsResponse`
//!   - `NominationDetailsResponse`, `TreatyDetailsResponse`, `HearingDetailsResponse`
//!   - ... and many others.
//! - **Supporting Models**: Structs representing components of responses, like `AmendmentSummary`,
//!   `MemberSummary`, `BillSummary`, `Committee`, `LatestAction`, etc.
//! 
//! ## Example
//! 
//! ```rust
//! use cdg_api::response_models::{BillsResponse, GenericResponseModel};
//! 
//! fn handle_response(response: BillsResponse) {
//!     for bill in response.bills {
//!         println!("Bill Number: {}", bill.number);
//!         if let Some(action) = bill.latest_action {
//!             println!("Latest Action: {}", action.text);
//!         }
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait PrimaryResponse {}

macro_rules! impl_primary_response {
    ($($t:ty),*) => {
        $(impl PrimaryResponse for $t {})*
    };
}

impl_primary_response!(
    GenericResponse,
    AmendmentsResponse,
    AmendmentDetailsResponse,
    AmendmentActionsResponse,
    AmendmentCosponsorsResponse,
    AmendmentAmendmentsResponse,
    AmendmentTextVersionsResponse,
    BillsResponse,
    BillDetailsResponse,
    BillActionsResponse,
    BillAmendmentsResponse,
    BillCommitteesResponse,
    BillCosponsorsResponse,
    RelatedBillsResponse,
    BillSubjectsResponse,
    BillSummariesResponse,
    BillTextVersionsResponse,
    BillTitlesResponse,
    SummariesResponse,
    LawsResponse,
    LawDetailsResponse,
    CongressesResponse,
    CongressDetailsResponse,
    CongressionalRecordResponse,
    DailyCongressionalRecordResponse,
    ArticlesResponse,
    MembersResponse,
    MemberDetailsResponse,
    NominationsResponse,
    NominationDetailsResponse,
    TreatiesResponse,
    TreatyDetailsResponse,
    HearingsResponse,
    HearingDetailsResponse,
    HouseCommunicationDetailsResponse,
    HouseCommunicationsResponse,
    CommitteesResponse,
    CommitteeDetailsResponse,
    CommitteeReportsResponse
);

/// Dynamic response model that can represent a variety of response types.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GenericResponseModel {
    AmendmentsResponse(AmendmentsResponse),
    AmendmentDetailsResponse(AmendmentDetailsResponse),
    AmendmentActionsResponse(AmendmentActionsResponse),
    AmendmentCosponsorsResponse(AmendmentCosponsorsResponse),
    AmendmentAmendmentsResponse(AmendmentAmendmentsResponse),
    AmendmentTextVersionsResponse(AmendmentTextVersionsResponse),
    BillsResponse(BillsResponse),
    BillDetailsResponse(BillDetailsResponse),
    BillActionsResponse(BillActionsResponse),
    BillAmendmentsResponse(BillAmendmentsResponse),
    BillCommitteesResponse(BillCommitteesResponse),
    BillCosponsorsResponse(BillCosponsorsResponse),
    RelatedBillsResponse(RelatedBillsResponse),
    BillSubjectsResponse(BillSubjectsResponse),
    BillSummariesResponse(BillSummariesResponse),
    BillTextVersionsResponse(BillTextVersionsResponse),
    BillTitlesResponse(BillTitlesResponse),
    SummariesResponse(SummariesResponse),
    LawsResponse(LawsResponse),
    LawDetailsResponse(LawDetailsResponse),
    CongressesResponse(CongressesResponse),
    CongressDetailsResponse(CongressDetailsResponse),
    CongressionalRecordResponse(CongressionalRecordResponse),
    DailyCongressionalRecordResponse(DailyCongressionalRecordResponse),
    ArticlesResponse(ArticlesResponse),
    MembersResponse(MembersResponse),
    MemberDetailsResponse(MemberDetailsResponse),
    NominationsResponse(NominationsResponse),
    NominationDetailsResponse(NominationDetailsResponse),
    TreatiesResponse(TreatiesResponse),
    TreatyDetailsResponse(TreatyDetailsResponse),
    LatestAction(LatestAction),
    ResourceReference(ResourceReference),
    CosponsorsReference(CosponsorsReference),
    MemberSummary(MemberSummary),
    AmendedBill(AmendedBill),
    AmendmentDetails(AmendmentDetails),
    AmendmentAction(AmendmentAction),
    RecordedVote(RecordedVote),
    SourceSystem(SourceSystem),
    AmendmentCosponsor(AmendmentCosponsor),
    TextVersion(TextVersion),
    TextFormat(TextFormat),
    CboCostEstimate(CboCostEstimate),
    CommitteeReport(CommitteeReport),
    LawReference(LawReference),
    PolicyArea(PolicyArea),
    RelationshipDetail(RelationshipDetail),
    LegislativeSubject(LegislativeSubject),
    BillSummaryItem(BillSummaryItem),
    BillTitle(BillTitle),
    SummaryItem(SummaryItem),
    CongressSummary(CongressSummary),
    Session(Session),
    Results(Results),
    Issue(Issue),
    Links(Links),
    Section(Section),
    Pdf(Pdf),
    PdfItem(PdfItem),
    DailyIssue(DailyIssue),
    FullIssue(FullIssue),
    EntireIssue(EntireIssue),
    EntireIssueItem(EntireIssueItem),
    Sections(Sections),
    SectionItem(SectionItem),
    SectionText(SectionText),
    SectionTextItem(SectionTextItem),
    Articles(Articles),
    BillAction(BillAction),
    Committee(Committee),
    CommitteeActivity(CommitteeActivity),
    BillCosponsor(BillCosponsor),
    RelatedBill(RelatedBill),
    BillReference(BillReference),
    LawSummary(LawSummary),
    Number(u32),
    Array(Vec<GenericResponseModel>),
}

/// Generic response model that is essentially a catch-all for endpoints that don't have a specific
/// response model, or when the response model is unknown.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GenericResponse {
    // Fields from GenericResponse itself
    #[serde(skip_serializing_if = "Option::is_none")]
    pub congress: Option<GenericResponseModel>,
    #[serde(rename = "latestAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_action: Option<LatestAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub amendment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amendment: Option<AmendmentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<GenericResponseModel>,
    #[serde(rename = "amendedBill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amended_bill: Option<AmendedBill>,
    #[serde(rename = "amendmentsToAmendment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amendments_to_amendment: Option<ResourceReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chamber: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosponsors: Option<GenericResponseModel>,
    #[serde(rename = "proposedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsors: Option<Vec<MemberSummary>>,
    #[serde(rename = "submittedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amendments: Option<Vec<AmendmentSummary>>,
    #[serde(rename = "textVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<TextVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<BillSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill: Option<BillDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committees: Option<Vec<Committee>>,
    #[serde(rename = "relatedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_bills: Option<Vec<RelatedBill>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Subjects>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<GenericResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<BillTitle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub congresses: Option<Vec<CongressSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Results>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dailyCongressionalRecord")]
    pub daily_congressional_record: Option<GenericResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles: Option<Vec<Article>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MemberDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominations: Option<Vec<NominationItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nomination: Option<NominationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treaties: Option<Vec<TreatyItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treaty: Option<TreatyDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl GenericResponse {
    /// Serializes the generic response model to a JSON string.
    /// If `pretty` is true, the JSON will be pretty-printed.
    pub fn serialize_generic_response(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(&self)
        } else {
            serde_json::to_string(&self)
        }
    }

    /// Attempts to parse the generic response model as a specific response model.
    pub fn parse_generic_response<T: PrimaryResponse + serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let json = serde_json::to_string(&self)?;
        let response: T = serde_json::from_str(&json)?;
        Ok(response)
    }
}

/// Response model for the `/amendment` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary of an amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentDetailsResponse {
    pub amendment: AmendmentDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Detailed information about a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Reference to a resource with a count and URL.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ResourceReference {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Reference to cosponsors with counts and URL.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CosponsorsReference {
    pub count: u32,
    #[serde(rename = "countIncludingWithdrawnCosponsors")]
    pub count_including_withdrawn_cosponsors: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Summary information about a member of Congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Information about the bill that is being amended.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/actions` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentActionsResponse {
    pub actions: Vec<AmendmentAction>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an action taken on an amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Information about a recorded vote.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RecordedVote {
    pub chamber: String,
    pub congress: u32,
    pub date: String,
    #[serde(rename = "rollNumber")]
    pub roll_number: u32,
    #[serde(rename = "sessionNumber")]
    pub session_number: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Information about the source system of the action.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SourceSystem {
    pub code: u32,
    pub name: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/cosponsors` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentCosponsorsResponse {
    pub cosponsors: Vec<AmendmentCosponsor>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a cosponsor of an amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/amendments` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentAmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/amendment/{congress}/{amendmentType}/{amendmentNumber}/text` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentTextVersionsResponse {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<TextVersion>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a text version of an amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TextVersion {
    pub date: String,
    pub formats: Vec<TextFormat>,
    #[serde(rename = "type")]
    pub text_type: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a format of the text version.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TextFormat {
    #[serde(rename = "type")]
    pub format_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillsResponse {
    pub bills: Vec<BillSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the latest action taken on a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LatestAction {
    #[serde(rename = "actionDate")]
    pub action_date: String,
    pub text: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillDetailsResponse {
    pub bill: BillDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Detailed information about a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillDetails {
    pub actions: ResourceReference,
    pub amendments: Option<ResourceReference>,
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a Congressional Budget Office cost estimate.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CboCostEstimate {
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub title: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a committee report reference.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReport {
    pub citation: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a law reference.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LawReference {
    pub number: String,
    #[serde(rename = "type")]
    pub law_type: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the policy area of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PolicyArea {
    pub name: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/actions` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillActionsResponse {
    pub actions: Vec<BillAction>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an action taken on a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/amendments` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillAmendmentsResponse {
    pub amendments: Vec<AmendmentSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/committees` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCommitteesResponse {
    pub committees: Vec<Committee>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a committee associated with a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Committee {
    pub activities: Option<Vec<CommitteeActivity>>,
    pub chamber: String,
    pub name: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    #[serde(rename = "type")]
    pub committee_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an activity of a committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeActivity {
    pub date: Option<String>,
    pub name: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/cosponsors` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCosponsorsResponse {
    pub cosponsors: Vec<BillCosponsor>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a cosponsor of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/relatedbills` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RelatedBillsResponse {
    #[serde(rename = "relatedBills")]
    pub related_bills: Vec<RelatedBill>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a related bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the details of a relationship between bills.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RelationshipDetail {
    #[serde(rename = "identifiedBy")]
    pub identified_by: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/subjects` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillSubjectsResponse {
    pub subjects: Subjects,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the subjects of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Subjects {
    #[serde(rename = "legislativeSubjects")]
    pub legislative_subjects: Vec<LegislativeSubject>,
    #[serde(rename = "policyArea")]
    pub policy_area: PolicyArea,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a legislative subject.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LegislativeSubject {
    pub name: String,
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/summaries` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillSummariesResponse {
    pub summaries: Vec<BillSummaryItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/text` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillTextVersionsResponse {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<TextVersion>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/bill/{congress}/{billType}/{billNumber}/titles` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillTitlesResponse {
    pub titles: Vec<BillTitle>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a title of a bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/summaries` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesResponse {
    pub summaries: Vec<SummaryItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary item in the summaries response.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a reference to a bill within a summary.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/law/{congress}` and `/law/{congress}/{lawType}` endpoints.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LawsResponse {
    pub bills: Vec<LawSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary of a law (bill that became a law).
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/law/{congress}/{lawType}/{lawNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LawDetailsResponse {
    pub bill: BillDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/congress` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressesResponse {
    pub congresses: Vec<CongressSummary>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a summary of a congress session.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressSummary {
    #[serde(rename = "endYear")]
    pub end_year: String,
    pub name: String,
    pub sessions: Vec<Session>,
    #[serde(rename = "startYear")]
    pub start_year: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a session within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Session {
    pub chamber: String,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub number: u32,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "type")]
    pub session_type: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/congress/{congress}` and `/congress/current` endpoints.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressDetailsResponse {
    pub congress: CongressDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Detailed information about a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/congressional-record` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressionalRecordResponse {
    pub results: Results,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for Congressional Record issues.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Results {
    pub issues: Vec<Issue>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual Congressional Record issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Issue {
    pub congress: u32,
    pub issue: u32,
    pub links: Links,
    #[serde(rename = "publishDate")]
    pub publish_date: String,
    pub session: u32,
    pub volume: u32,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for links to the individual sections of the issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Links {
    pub digest: Option<Section>,
    pub remarks: Option<Section>,
    pub house: Option<Section>,
    pub senate: Option<Section>,
    #[serde(rename = "fullRecord")]
    pub full_record: Option<Section>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a section of the Congressional Record issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section {
    pub label: String,
    pub ordinal: u32,
    pub pdf: Pdf,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for the PDF text format for the section.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Pdf {
    pub items: Vec<PdfItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual PDF text format for the section.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PdfItem {
    pub part: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/daily-congressional-record` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DailyCongressionalRecordResponse {
    #[serde(rename = "dailyCongressionalRecord")]
    pub daily_congressional_record: Vec<DailyIssue>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual daily Congressional Record issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DailyIssue {
    #[serde(rename = "issueNumber")]
    pub issue_number: String,
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
    pub full_issue: Option<FullIssue>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for full issue, sections, and articles.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FullIssue {
    #[serde(rename = "entireIssue")]
    pub entire_issue: Option<EntireIssue>,
    pub sections: Option<Sections>,
    pub articles: Option<Articles>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the entire issue items.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EntireIssue {
    pub items: Vec<EntireIssueItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an entire issue item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EntireIssueItem {
    pub part: u32,
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for sections in the issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sections {
    pub items: Vec<SectionItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a section item of the daily Congressional Record issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SectionItem {
    pub name: String,
    #[serde(rename = "startPage")]
    pub start_page: String,
    #[serde(rename = "endPage")]
    pub end_page: String,
    pub text: SectionText,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for section text items.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SectionText {
    pub items: Vec<SectionTextItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a section text item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SectionTextItem {
    pub part: u32,
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for articles in the issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Articles {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/daily-congressional-record/{volumeNumber}/{issueNumber}/articles` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual article in a section.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Article {
    pub title: String,
    #[serde(rename = "startPage")]
    pub start_page: String,
    #[serde(rename = "endPage")]
    pub end_page: String,
    pub text: ArticleText,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Container for article text items.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ArticleText {
    pub items: Vec<ArticleTextItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an article text item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ArticleTextItem {
    #[serde(rename = "type")]
    pub document_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/member` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MembersResponse {
    pub members: Vec<Member>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual member's entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a member's terms of service.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Terms {
    pub item: Vec<Term>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual term of service for a member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Term {
    pub chamber: String,
    #[serde(rename = "startYear")]
    pub start_year: u32,
    #[serde(rename = "endYear")]
    pub end_year: Option<u32>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the member's current official portrait.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Depiction {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub attribution: Option<String>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/member/{bioguideId}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberDetailsResponse {
    pub member: MemberDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents detailed information about an individual member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a member's term of service in a Congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a member's contact information.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AddressInformation {
    #[serde(rename = "officeAddress")]
    pub office_address: String,
    pub city: String,
    pub district: String,
    #[serde(rename = "zipCode")]
    pub zip_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a leadership position held by the member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LeadershipPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    pub congress: u32,
    pub current: bool,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a reference to a member's sponsored or cosponsored legislation.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LegislationReference {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/nomination` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationsResponse {
    pub nominations: Vec<NominationItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual nomination entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationItem {
    pub congress: u32,
    pub number: u32,
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,
    pub citation: String,
    pub description: Option<String>,
    #[serde(rename = "receivedDate")]
    pub received_date: String,
    #[serde(rename = "nominationType")]
    pub nomination_type: NominationType,
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,
    pub url: String,
    pub organization: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents the type of nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationType {
    #[serde(rename = "isCivilian")]
    pub is_civilian: Option<bool>,
    #[serde(rename = "isMilitary")]
    pub is_military: Option<bool>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/nomination/{congress}/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationDetailsResponse {
    pub nomination: NominationDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents detailed information about a nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual nominee position within a nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a reference to committees associated with the nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteesReference {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a reference to actions taken on the nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ActionsReference {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a reference to printed hearings associated with the nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingsReference {
    pub count: u32,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/treaty` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatiesResponse {
    pub treaties: Vec<TreatyItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual treaty entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents parts of a treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyParts {
    pub count: u32,
    pub urls: Vec<String>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/treaty/{congress}/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyDetailsResponse {
    pub treaty: TreatyDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents detailed information about a treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a country or party associated with the treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CountryParty {
    pub name: String,
    #[serde(rename = "oldNumber")]
    pub old_number: Option<String>,
    #[serde(rename = "oldNumberDisplayName")]
    pub old_number_display_name: Option<String>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an index term associated with the treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct IndexTerm {
    pub name: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an executive report associated with the treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RelatedDoc {
    pub name: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/hearing` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingsResponse {
    pub hearings: Vec<HearingItem>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents an individual hearing entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingItem {
    #[serde(rename = "jacketNumber")]
    pub jacket_number: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub chamber: String,
    pub congress: u32,
    pub number: u32,
    pub part: Option<String>,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/hearing/{congress}/{chamber}/{jacketNumber}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingDetailsResponse {
    pub hearing: HearingDetails,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents detailed information about a hearing.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingDetails {
    #[serde(rename = "jacketNumber")]
    pub jacket_number: String,
    #[serde(rename = "libraryOfCongressIdentifier")]
    pub library_of_congress_identifier: String,
    pub number: u32,
    pub part: Option<String>,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub congress: u32,
    pub title: String,
    pub citation: String,
    pub chamber: String,
    pub committees: Vec<HearingCommittee>,
    pub dates: Vec<HearingDate>,
    pub formats: Vec<HearingFormat>,
    #[serde(rename = "associatedMeeting")]
    pub associated_meeting: Option<AssociatedMeeting>,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a committee that held the hearing.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingCommittee {
    pub name: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a date when the hearing was held.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingDate {
    pub date: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents a hearing transcript format.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingFormat {
    #[serde(rename = "type")]
    pub format_type: String,
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Represents meeting information associated with the hearing.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AssociatedMeeting {
    #[serde(rename = "eventID")]
    pub event_id: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(flatten)]
    pub unknown: Option<Value>,
}

/// Response model for the `/house-communication` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseCommunicationsResponse {
    pub house_communications: Vec<HouseCommunicationItem>,
}

/// Represents an individual House communication entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseCommunicationItem {
    pub chamber: String,
    pub number: u32,
    #[serde(rename = "communicationType")]
    pub communication_type: CommunicationType,
    #[serde(rename = "congressNumber")]
    pub congress_number: u32,
    pub url: String,
}

/// Represents the type of communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommunicationType {
    pub code: String,
    pub name: String,
}

/// Response model for the `/house-communication/{congress}/{type}/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseCommunicationDetailsResponse {
    #[serde(rename = "house-communication")]
    pub house_communication: HouseCommunicationDetails,
}

/// Represents detailed information about a House communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseCommunicationDetails {
    pub chamber: String,
    pub number: u32,
    #[serde(rename = "communicationType")]
    pub communication_type: CommunicationType,
    pub congress: u32,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub abstract_text: String,
    #[serde(rename = "congressionalRecordDate")]
    pub congressional_record_date: String,
    #[serde(rename = "sessionNumber")]
    pub session_number: u32,
    #[serde(rename = "isRulemaking")]
    pub is_rulemaking: String,
    pub committees: Vec<CommunicationCommittee>,
    #[serde(rename = "matchingRequirements")]
    pub matching_requirements: Vec<MatchingRequirement>,
    #[serde(rename = "houseDocument")]
    pub house_document: Vec<HouseDocument>,
}

/// Represents a committee associated with the communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommunicationCommittee {
    pub name: String,
    #[serde(rename = "referralDate")]
    pub referral_date: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
}

/// Represents a matching requirement associated with the communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MatchingRequirement {
    pub number: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "reportNature")]
    pub report_nature: String,
    #[serde(rename = "submittingAgency")]
    pub submitting_agency: String,
    #[serde(rename = "submittingOfficial")]
    pub submitting_official: String,
    #[serde(rename = "legalAuthority")]
    pub legal_authority: String,
}

/// Represents a House document associated with the communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseDocument {
    pub citation: String,
    pub title: String,
}

/// Response model for the `/house-requirement` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseRequirementsResponse {
    pub house_requirements: Vec<HouseRequirementItem>,
}

/// Represents an individual House requirement entry.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseRequirementItem {
    pub number: u32,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub url: String,
}

/// Response model for the `/house-requirement/{number}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseRequirementDetailsResponse {
    #[serde(rename = "houseRequirement")]
    pub house_requirement: HouseRequirementDetails,
}

/// Represents detailed information about a House requirement.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HouseRequirementDetails {
    pub number: u32,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "parentAgency")]
    pub parent_agency: String,
    pub frequency: String,
    pub nature: String,
    #[serde(rename = "legalAuthority")]
    pub legal_authority: String,
    #[serde(rename = "activeRecord")]
    pub active_record: bool,
    #[serde(rename = "submittingAgency")]
    pub submitting_agency: String,
    #[serde(rename = "submittingOfficial")]
    pub submitting_official: String,
    #[serde(rename = "matchingCommunications")]
    pub matching_communications: MatchingCommunications,
}

/// Represents matching communications associated with a House requirement.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MatchingCommunications {
    pub count: u32,
    pub url: String,
    pub items: Vec<MatchingCommunicationItem>,
}

/// Represents a House communication matching a House requirement.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MatchingCommunicationItem {
    pub chamber: String,
    pub number: u32,
    #[serde(rename = "communicationType")]
    pub communication_type: CommunicationType,
    pub congress: u32,
    pub url: String,
}

/// Response model for the `/committee` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteesResponse {
    pub committees: Vec<CommitteeItem>,
}

/// Represents an individual committee or subcommittee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeItem {
    pub url: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    pub name: String,
    pub chamber: String,
    #[serde(rename = "committeeTypeCode")]
    pub committee_type_code: String,
    pub parent: Option<ParentCommittee>,
    pub subcommittees: Option<Vec<SubcommitteeItem>>,
}

/// Represents a parent committee for a subcommittee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ParentCommittee {
    pub url: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    pub name: String,
}

/// Represents an individual subcommittee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SubcommitteeItem {
    pub url: String,
    #[serde(rename = "systemCode")]
    pub system_code: String,
    pub name: String,
}

/// Response model for the `/committee/{systemCode}` endpoint.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeDetailsResponse {
    pub committee: CommitteeDetails,
}

/// Represents detailed information about a committee or subcommittee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeDetails {
    #[serde(rename = "systemCode")]
    pub system_code: String,
    pub name: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "isCurrent")]
    pub is_current: bool,
    pub parent: Option<ParentCommittee>,
    pub subcommittees: Option<Vec<SubcommitteeItem>>,
    pub reports: Option<CommitteeReports>,
    pub communications: Option<CommitteeCommunications>,
    pub bills: Option<CommitteeBills>,
    pub nominations: Option<CommitteeNominations>,
    pub history: Option<Vec<CommitteeHistoryItem>>,
    pub committee_type: String,
}

/// Represents reports issued by a committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReports {
    pub url: String,
    pub count: u32,
}

/// Represents communications associated with a committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeCommunications {
    pub url: String,
    pub count: u32,
}

/// Represents bills associated with a committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeBills {
    pub url: String,
    pub count: u32,
}

/// Represents nominations associated with a Senate committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeNominations {
    pub url: String,
    pub count: u32,
}

/// Represents the history of a committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeHistoryItem {
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "officialName")]
    pub official_name: String,
    #[serde(rename = "libraryOfCongressName")]
    pub library_of_congress_name: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "committeeTypeCode")]
    pub committee_type_code: String,
    #[serde(rename = "establishingAuthority")]
    pub establishing_authority: Option<String>,
    #[serde(rename = "locLinkedDataId")]
    pub loc_linked_data_id: Option<String>,
    #[serde(rename = "superintendentDocumentNumber")]
    pub superintendent_document_number: Option<String>,
    #[serde(rename = "naraId")]
    pub nara_id: Option<String>,
}

/// Represents a response containing a list of committee meetings.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingsResponse {
    #[serde(rename = "committeeMeetings")]
    pub committee_meetings: Vec<CommitteeMeetingItem>,
}

/// Represents an individual committee meeting item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingItem {
    #[serde(rename = "eventId")]
    pub event_id: u32,
    pub url: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub congress: u32,
    pub chamber: String,
}

/// Represents detailed information about a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingDetailsResponse {
    #[serde(rename = "eventId")]
    pub event_id: u32,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub congress: u32,
    #[serde(rename = "type")]
    pub meeting_type: String,
    pub title: String,
    #[serde(rename = "meetingStatus")]
    pub meeting_status: String,
    pub date: String,
    pub chamber: String,
    pub committees: Vec<CommitteeItem>,
    pub location: Option<MeetingLocation>,
    pub videos: Option<Vec<VideoItem>>,
    pub witnesses: Option<Vec<WitnessItem>>,
    #[serde(rename = "witnessDocuments")]
    pub witness_documents: Option<Vec<WitnessDocumentItem>>,
    #[serde(rename = "meetingDocuments")]
    pub meeting_documents: Option<Vec<MeetingDocumentItem>>,
    #[serde(rename = "hearingTranscript")]
    pub hearing_transcript: Option<HearingTranscript>,
    #[serde(rename = "relatedItems")]
    pub related_items: Option<RelatedItems>,
}

/// Represents the location of a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MeetingLocation {
    pub room: Option<String>,
    pub building: Option<String>,
    pub address: Option<String>,
}

/// Represents a video related to a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VideoItem {
    pub name: String,
    pub url: String,
}

/// Represents a witness associated with a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WitnessItem {
    pub name: String,
    pub position: String,
    pub organization: String,
}

/// Represents a document provided by a witness.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WitnessDocumentItem {
    #[serde(rename = "documentType")]
    pub document_type: String,
    pub format: String,
    pub url: String,
}

/// Represents a document related to a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MeetingDocumentItem {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "documentType")]
    pub document_type: String,
    pub url: String,
    pub format: String,
}

/// Represents a hearing transcript associated with a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingTranscript {
    #[serde(rename = "jacketNumber")]
    pub jacket_number: String,
    pub url: String,
}

/// Represents items related to a committee meeting, such as bills, treaties, or nominations.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RelatedItems {
    pub bills: Option<Vec<BillItem>>,
    pub treaties: Option<Vec<TreatyItem>>,
    pub nominations: Option<Vec<NominationItem>>,
}

/// Represents a bill or resolution associated with a committee meeting.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillItem {
    #[serde(rename = "billType")]
    pub bill_type: String,
    pub number: u32,
    pub congress: u32,
    pub url: String,
}

/// Represents a response containing a list of committee prints.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintsResponse {
    #[serde(rename = "committeePrints")]
    pub committee_prints: Vec<CommitteePrintItem>,
}

/// Represents an individual committee print item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintItem {
    #[serde(rename = "jacketNumber")]
    pub jacket_number: String,
    pub url: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub congress: u32,
    pub chamber: String,
}

/// Represents detailed information about a committee print.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintDetailsResponse {
    #[serde(rename = "jacketNumber")]
    pub jacket_number: String,
    pub citation: Option<String>,
    pub congress: u32,
    pub number: Option<u32>,
    pub title: String,
    pub chamber: String,
    pub committees: Vec<CommitteeItem>,
    #[serde(rename = "associatedBills")]
    pub associated_bills: Option<Vec<AssociatedBillItem>>,
    pub text: Option<CommitteePrintText>,
}

/// Represents a bill associated with a committee print.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AssociatedBillItem {
    pub congress: u32,
    #[serde(rename = "type")]
    pub bill_type: String,
    pub number: u32,
    pub url: String,
}

/// Represents the text formats available for a committee print.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintText {
    pub count: u32,
    pub url: String,
}

/// Represents a text format of a committee print.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintTextItem {
    pub url: String,
    #[serde(rename = "type")]
    pub format_type: String,
}

/// Represents a response containing a list of committee reports.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportsResponse {
    #[serde(rename = "reports")]
    pub reports: Vec<CommitteeReportItem>,
}

/// Represents an individual committee report item.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportItem {
    pub citation: String,
    pub url: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    pub congress: u32,
    pub chamber: String,
    #[serde(rename = "type")]
    pub report_type: String,
    pub number: u32,
    pub part: Option<u32>,
}

/// Represents detailed information about a committee report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportDetailsResponse {
    pub committees: Vec<CommitteeItem>,
    pub congress: u32,
    pub chamber: String,
    #[serde(rename = "sessionNumber")]
    pub session_number: u32,
    pub citation: String,
    pub number: u32,
    pub part: Option<u32>,
    #[serde(rename = "type")]
    pub report_type: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "isConferenceReport")]
    pub is_conference_report: bool,
    pub title: String,
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    #[serde(rename = "reportType")]
    pub report_type_full: String,
    pub text: Option<CommitteeReportText>,
    #[serde(rename = "associatedTreaties")]
    pub associated_treaties: Option<Vec<AssociatedTreatyItem>>,
    #[serde(rename = "associatedBill")]
    pub associated_bills: Option<Vec<AssociatedBillItem>>,
}

/// Represents a treaty associated with an executive report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AssociatedTreatyItem {
    pub congress: u32,
    pub number: u32,
    pub part: Option<String>,
    pub url: String,
}

/// Represents the text formats available for a committee report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportText {
    pub count: u32,
    pub url: String,
}

/// Represents a text format of a committee report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportTextItem {
    pub url: String,
    #[serde(rename = "type")]
    pub format_type: String,
    #[serde(rename = "isErrata")]
    pub is_errata: String,
}
