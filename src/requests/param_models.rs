//! # `param_models` Module
//!
//! This module defines the parameter models used for constructing API requests to various endpoints
//! of the US Congress API. It includes structs for endpoint-specific parameters, facilitating
//! type-safe and structured API interactions.
//!
//! # `param_chains` module
//!
//! Simply holds a macro that allows each Param model to call `build` methods for user ease
//!
//! ## Example
//!
//! ```rust
//! use cdg_api::param_models::BillListParams;
//! use cdg_api::cdg_types::{FormatType, SortType};
//!
//! fn main() {
//!     let params = BillListParams::default()
//!         .format(FormatType::Json)
//!         .limit(10)
//!         .sort(SortType::UpdateDateDesc);
//!
//!     // Use `params` with an endpoint constructor
//! }
//! ```

use crate::cdg_types::*;
use serde::{Deserialize, Serialize};

// ================================
// Endpoint-Specific Parameter Structs
// ================================

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GenericParams {
    /// Desired response format (JSON or XML).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FormatType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_member: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chamber: Option<ChamberType>,
}

impl GenericParams {
    pub fn new(
        format: Option<FormatType>,
        offset: Option<u32>,
        limit: Option<u32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        sort: Option<SortType>,
        conference: Option<bool>,
        current_member: Option<bool>,
        year: Option<u32>,
        month: Option<u32>,
        day: Option<u32>,
        chamber: Option<ChamberType>,
    ) -> Self {
        Self {
            format,
            offset,
            limit,
            from_date_time,
            to_date_time,
            sort,
            conference,
            current_member,
            year,
            month,
            day,
            chamber,
        }
    }
}

/// Parameters for the [`BillList`] endpoint.
///
/// These parameters allow filtering and pagination when listing bills.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`BillByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving bills
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`BillByType`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving bills
/// of a specific type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`BillDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`BillActions`] endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillAmendments`] endpoint.
///
/// These parameters allow pagination when listing amendments of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillAmendmentsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillCommittees`] endpoint.
///
/// These parameters allow pagination when retrieving committees associated with a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillCosponsors`] endpoint.
///
/// These parameters allow pagination when retrieving cosponsors of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCosponsorsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillRelated`] endpoint.
///
/// These parameters allow pagination when listing related bills of a specific bill.
/// Related bills include companion bills, identical bills, etc.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillRelatedParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillSubjects`] endpoint.
///
/// These parameters allow pagination when listing legislative subjects associated with a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillSubjectsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering subjects.
    #[serde(rename = "fromDateTime")]
    pub from_date_time: Option<String>,

    /// End date-time for filtering subjects.
    #[serde(rename = "toDateTime")]
    pub to_date_time: Option<String>,
}

/// Parameters for the [`BillSummaries`] endpoint.
///
/// These parameters allow pagination when listing summaries of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillSummariesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillText`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving the text of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillTextParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`BillTitles`] endpoint.
///
/// These parameters allow pagination when listing titles of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillTitlesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering titles.
    #[serde(rename = "fromDateTime")]
    pub from_date_time: Option<String>,

    /// End date-time for filtering titles.
    #[serde(rename = "toDateTime")]
    pub to_date_time: Option<String>,
}

// ================================
// Law Endpoints Parameters
// ================================

/// Parameters for all law endpoints.
///
/// These parameters allow filtering and pagination when listing laws.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LawParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ================================
// Amendment Endpoints Parameters
// ================================

/// Parameters for the [`AmendmentList`] endpoint.
///
/// These parameters allow filtering and pagination when listing amendments.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`AmendmentByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving amendments
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`AmendmentByType`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving amendments
/// of a specific type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`AmendmentDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`AmendmentActions`] endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`AmendmentCosponsors`] endpoint.
///
/// These parameters allow pagination when retrieving cosponsors of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentCosponsorsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`AmendmentAmendments`] endpoint.
///
/// These parameters allow pagination when listing amendments of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentAmendmentsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`AmendmentText`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving the text of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentTextParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// ================================
// Member Endpoints Parameters
// ================================

/// Parameters for the [`MemberList`] endpoint.
///
/// These parameters allow filtering and pagination when listing members of Congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering members.
    pub from_date_time: Option<String>,

    /// End date-time for filtering members.
    pub to_date_time: Option<String>,

    ///
    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the [`MemberByState`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// representing a specific state.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByStateParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the [`MemberByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// associated with a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the [`MemberByStateDistrict`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// by state and district.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByStateDistrictParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the [`MemberByCongressStateDistrict`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// by congress, state, and district.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByCongressStateDistrictParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the [`MemberDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`SponsorshipList`] endpoint.
///
/// These parameters allow pagination when listing sponsorships of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SponsorshipListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CosponsorshipList`] endpoint.
///
/// These parameters allow pagination when listing cosponsorships of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CosponsorshipListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ================================
// Committee Endpoints Parameters
// ================================

/// Parameters for the [`CommitteeList`] endpoint.
///
/// These parameters allow filtering and pagination when listing committees.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeByChamber`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// by chamber (House, Senate, or Joint).
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// associated with a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeByCongressChamber`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// by both congress and chamber.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByCongressChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`CommitteeBills`] endpoint.
///
/// These parameters allow pagination when listing bills under a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeBillsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeReports`] endpoint.
///
/// These parameters allow pagination when retrieving reports from a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeNominations`] endpoint.
///
/// These parameters allow pagination when listing nominations handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeNominationsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeHouseCommunication`] endpoint.
///
/// These parameters allow pagination when retrieving house communications handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeHouseCommunicationParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeSenateCommunication`] endpoint.
///
/// These parameters allow pagination when retrieving senate communications handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeSenateCommunicationParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ================================
// Committee Report Endpoints Parameters
// ================================

/// Parameters for the [`ReportList`] endpoint.
///
/// These parameters allow filtering and pagination when listing committee reports.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    pub conference: Option<bool>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering reports.
    pub from_date_time: Option<String>,

    /// End date-time for filtering reports.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeReportByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee reports
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    pub conference: Option<bool>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering reports.
    pub from_date_time: Option<String>,

    /// End date-time for filtering reports.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeReportByType`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee reports
/// of a specific type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    pub conference: Option<bool>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering reports.
    pub from_date_time: Option<String>,

    /// End date-time for filtering reports.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteeReportDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific committee report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`CommitteeReportText`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving the text of a specific committee report.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportTextParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ====================================
// Committee Print Endpoints Parameters
// ====================================

/// Parameters for the [`CommitteePrintList`] endpoint.
///
/// These parameters allow filtering and pagination when listing committee prints.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering prints.
    pub from_date_time: Option<String>,

    /// End date-time for filtering prints.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteePrintByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee prints
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering prints.
    pub from_date_time: Option<String>,

    /// End date-time for filtering prints.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteePrintByCongressChamber`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee prints
/// by congress and chamber.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintByCongressChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering prints.
    pub from_date_time: Option<String>,

    /// End date-time for filtering prints.
    pub to_date_time: Option<String>,
}

/// Parameters for the [`CommitteePrintByJacketNumber`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving committee prints by jacket number.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintByJacketNumberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`CommitteePrintDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific committee print.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteePrintDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ======================================
// Committee Meeting Endpoints Parameters
// ======================================

/// Parameters for the [`CommitteeMeetingList`] endpoint.
///
/// These parameters allow filtering and pagination when listing committee meetings.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeMeetingByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee meetings
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeMeetingByChamber`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee meetings
/// by chamber (House, Senate, or Joint).
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingByChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CommitteeMeetingByEvent`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving committee meetings
/// by event.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeMeetingByEventParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// ================================
// Hearing Endpoints Parameters
// ================================

/// Parameters for the [`HearingList`] endpoint.
///
/// These parameters allow filtering and pagination when listing hearings.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`HearingByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving hearings
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`HearingByChamber`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving hearings
/// by chamber (House, Senate, or Joint).
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingByChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`HearingByJacketNumber`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving hearings by jacket number.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HearingByJacketNumberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// =========================================
// Congressional Record Endpoints Parameters
// =========================================

/// Parameters for the [`CongressionalRecordList`] endpoint.
///
/// These parameters allow filtering and pagination when listing congressional records.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressionalRecordListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Year of the congressional record.
    pub year: Option<u32>,

    /// Month of the congressional record.
    pub month: Option<u32>,

    /// Day of the congressional record.
    pub day: Option<u32>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ===============================================
// Daily Congressional Record Endpoints Parameters
// ===============================================

/// Parameters for the [`DailyCongressionalRecordList`] endpoint.
///
/// These parameters allow filtering and pagination when listing daily congressional records.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DailyCongressionalRecordListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`DailyCongressionalRecordVolume`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving daily congressional records by volume.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DailyCongressionalVolumeNumberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`DailyCongressionalRecordVolumeIssue`] endpoint and
/// the [`DailyCongressionalRecordArticles`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving daily congressional records by volume and issue.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DailyCongressionalVolumeNumberIssueNumberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ===============================================
// Bound Congressional Record Endpoints Parameters
// ===============================================

/// Parameters for the [`BoundCongressionalRecord`] endpoints.
///
/// These parameters allow specifying the desired response format
/// when retrieving bound congressional records.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BoundCongressionalRecordParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ======================================
// House Requirement Endpoints Parameters
// ======================================

/// Parameters for the [`HouseRequirementList`] endpoint.
///
/// These parameters allow filtering and pagination when listing house requirements.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RequirementParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`HouseRequirementByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving house requirements
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RequirementDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// ==================================================
// House and Senate Communication Endpoint Parameters
// ==================================================

/// Parameters for the much of the House and Senate Communication endpoints.
///
/// These parameters allow filtering and pagination when listing communications.
/// These parameters are used by the following endpoints:
/// - [`HouseCommunicationList`]
/// - [`HouseCommunicationByCongress`]
/// - [`HouseCommunicationByType`]
///
/// - [`SenateCommunicationList`]
/// - [`SenateCommunicationByCongress`]
/// - [`SenateCommunicationByType`]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommunicationParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`HouseCommunicationDetails`] and [`SenateCommunicationDetails`] endpoints.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific communication.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommunicationDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// ================================
// Nomination Endpoints Parameters
// ================================

/// Parameters for the [`NominationList`] endpoint.
///
/// These parameters allow filtering and pagination when listing nominations.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering nominations.
    pub from_date_time: Option<String>,

    /// End date-time for filtering nominations.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`NominationByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving nominations
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering nominations.
    pub from_date_time: Option<String>,

    /// End date-time for filtering nominations.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`NominationDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`Nominees`] endpoint.
///
/// These parameters allow pagination when listing nominees of a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NomineesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`NominationActions`] endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`NominationCommittees`] endpoint.
///
/// These parameters allow pagination when retrieving committees involved in a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`NominationHearings`] endpoint.
///
/// These parameters allow pagination when listing hearings related to a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationHearingsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ================================
// Treaty Endpoints Parameters
// ================================

/// Parameters for the [`TreatyList`] endpoint.
///
/// These parameters allow filtering and pagination when listing treaties.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering treaties.
    pub from_date_time: Option<String>,

    /// End date-time for filtering treaties.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`TreatyByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving treaties
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering treaties.
    pub from_date_time: Option<String>,

    /// End date-time for filtering treaties.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`TreatyDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`TreatyPartitioned`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving partitioned information about a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyPartitionedParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`TreatyCommittees`] endpoint.
///
/// These parameters allow pagination when retrieving committees associated with a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`TreatyActions`] endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

// ----------------
// Summaries Endpoints Structs
// ----------------

/// Parameters for the [`SummariesList`] endpoint.
///
/// These parameters allow filtering and pagination when listing summaries.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering summaries.
    pub from_date_time: Option<String>,

    /// End date-time for filtering summaries.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`SummariesByCongress`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving summaries
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering summaries.
    pub from_date_time: Option<String>,

    /// End date-time for filtering summaries.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the [`SummariesByType`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving summaries
/// of a specific bill type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,

    /// Start date-time for filtering summaries.
    pub from_date_time: Option<String>,

    /// End date-time for filtering summaries.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

// ----------------
// Congress Endpoints Structs
// ----------------

/// Parameters for the [`CongressList`] endpoint.
///
/// These parameters allow filtering and pagination when listing congress sessions.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Parameters for the [`CongressDetails`] endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific congress session.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the [`CongressCurrent`] endpoint.
///
/// These parameters allow filtering and pagination when retrieving information
/// about the current congress session.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressCurrentParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,

    /// Maximum number of items to return.
    pub limit: Option<u32>,
}

/// Macro for constructing the chainables for a Param
macro_rules! impl_builder {
    ($struct_name:ident, { $($field:ident : $type:ty),* $(,)? }) => {
        impl $struct_name {
            $(
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
        }
    };
}

// Generic Params
impl_builder!(GenericParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
    conference: bool,
    current_member: bool,
    year: u32,
    month: u32,
    day: u32,
    chamber: ChamberType,
});

// Bill Endpoints Parameters
impl_builder!(BillListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(BillByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(BillByTypeParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(BillDetailsParams, {
    format: FormatType,
});

impl_builder!(BillActionsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillAmendmentsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillCommitteesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillCosponsorsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillRelatedParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillSubjectsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(BillSummariesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillTextParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(BillTitlesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

// Law Endpoints Parameters
impl_builder!(LawParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Amendment Endpoints Parameters
impl_builder!(AmendmentListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(AmendmentByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(AmendmentByTypeParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(AmendmentDetailsParams, {
    format: FormatType,
});

impl_builder!(AmendmentActionsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(AmendmentCosponsorsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(AmendmentAmendmentsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(AmendmentTextParams, {
    format: FormatType,
});

// Member Endpoints Parameters
impl_builder!(MemberListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    current_member: bool,
});

impl_builder!(MemberByStateParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    current_member: bool,
});

impl_builder!(MemberByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    current_member: bool,
});

impl_builder!(MemberByStateDistrictParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    current_member: bool,
});

impl_builder!(MemberByCongressStateDistrictParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    current_member: bool,
});

impl_builder!(MemberDetailsParams, {
    format: FormatType,
});

impl_builder!(SponsorshipListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CosponsorshipListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Committee Endpoints Parameters
impl_builder!(CommitteeListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeByChamberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeByCongressChamberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeDetailsParams, {
    format: FormatType,
});

impl_builder!(CommitteeBillsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeReportsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeNominationsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeHouseCommunicationParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeSenateCommunicationParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Committee Report Endpoints Parameters
impl_builder!(CommitteeReportListParams, {
    format: FormatType,
    conference: bool,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeReportByCongressParams, {
    format: FormatType,
    conference: bool,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeReportByTypeParams, {
    format: FormatType,
    conference: bool,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteeReportDetailsParams, {
    format: FormatType,
});

impl_builder!(CommitteeReportTextParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Committee Print Endpoints Parameters
impl_builder!(CommitteePrintListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteePrintByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteePrintByCongressChamberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
});

impl_builder!(CommitteePrintByJacketNumberParams, {
    format: FormatType,
});

impl_builder!(CommitteePrintDetailsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Committee Meeting Endpoints Parameters
impl_builder!(CommitteeMeetingListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeMeetingByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeMeetingByChamberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommitteeMeetingByEventParams, {
    format: FormatType,
});

// Hearing Endpoints Parameters
impl_builder!(HearingListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(HearingByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(HearingByChamberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(HearingByJacketNumberParams, {
    format: FormatType,
});

// Congressional Record Endpoints Parameters
impl_builder!(CongressionalRecordListParams, {
    format: FormatType,
    year: u32,
    month: u32,
    day: u32,
    offset: u32,
    limit: u32,
});

// Daily Congressional Record Endpoints Parameters
impl_builder!(DailyCongressionalRecordListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(DailyCongressionalVolumeNumberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(DailyCongressionalVolumeNumberIssueNumberParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Bound Congressional Record Endpoints Parameters
impl_builder!(BoundCongressionalRecordParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// House Requirement Endpoints Parameters
impl_builder!(RequirementParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(RequirementDetailsParams, {
    format: FormatType,
});

// House and Senate Communication Endpoints Parameters
impl_builder!(CommunicationParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CommunicationDetailsParams, {
    format: FormatType,
});

// Nomination Endpoints Parameters
impl_builder!(NominationListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(NominationByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(NominationDetailsParams, {
    format: FormatType,
});

impl_builder!(NomineesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(NominationActionsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(NominationCommitteesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(NominationHearingsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Treaty Endpoints Parameters
impl_builder!(TreatyListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(TreatyByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(TreatyDetailsParams, {
    format: FormatType,
});

impl_builder!(TreatyPartitionedParams, {
    format: FormatType,
});

impl_builder!(TreatyCommitteesParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(TreatyActionsParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

// Summaries Endpoints Parameters
impl_builder!(SummariesListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(SummariesByCongressParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

impl_builder!(SummariesByTypeParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
    from_date_time: String,
    to_date_time: String,
    sort: SortType,
});

// Congress Endpoints Parameters
impl_builder!(CongressListParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});

impl_builder!(CongressDetailsParams, {
    format: FormatType,
});

impl_builder!(CongressCurrentParams, {
    format: FormatType,
    offset: u32,
    limit: u32,
});
