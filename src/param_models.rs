//! # `param_models` Module
//! 
//! This module defines the parameter models used for constructing API requests to various endpoints
//! of the US Congress API. It includes enums for standardized parameter values and structs for
//! endpoint-specific parameters, facilitating type-safe and structured API interactions.
//! 
//! ## Enums
//! 
//! - **`FormatType`**: Specifies the response format (`json` or `xml`).
//! - **`SortType`**: Defines sorting options (e.g., `updateDateAsc`, `updateDateDesc`).
//! - **`BillType`**: Categorizes different types of bills (e.g., `Hr`, `S`, `Hjres`).
//! - **`AmendmentType`**: Categorizes amendment types (e.g., `Hamdt`, `Samdt`).
//! - **`ChamberType`**: Distinguishes between legislative chambers (`House`, `Senate`, `Joint`).
//! - **`CommunicationType`**: Defines types of committee communications (e.g., `Ec`, `Pm`).
//! - **`LawType`**: Differentiates between public and private laws (`Pub`, `Priv`).
//! 
//! ## Parameter Structs
//! 
//! Each API endpoint has a corresponding parameter struct that allows filtering, pagination, and sorting:
//! 
//! - **Bill Endpoints**:
//!   - `BillListParams`
//!   - `BillByCongressParams`
//!   - `BillByTypeParams`
//!   - `BillDetailsParams`
//!   - `BillActionsParams`
//!   - `BillAmendmentsParams`
//!   - `BillCommitteesParams`
//!   - `BillCosponsorsParams`
//! 
//! - **Amendment Endpoints**:
//!   - `AmendmentListParams`
//!   - `AmendmentByCongressParams`
//!   - `AmendmentByTypeParams`
//!   - `AmendmentDetailsParams`
//!   - `AmendmentActionsParams`
//!   - `AmendmentCosponsorsParams`
//!   - `AmendmentAmendmentsParams`
//!   - `AmendmentTextParams`
//! 
//! - **Law Endpoints**:
//!   - `LawParams`
//! 
//! - **Member Endpoints**:
//!   - `MemberListParams`
//!   - `MemberByStateParams`
//!   - `MemberByCongressParams`
//!   - `MemberByCongressStateDistrictParams`
//!   - `MemberDetailsParams`
//!   - `SponsorshipListParams`
//!   - `CosponsorshipListParams`
//! 
//! - **Committee Endpoints**:
//!   - `CommitteeListParams`
//!   - `CommitteeByChamberParams`
//!   - `CommitteeByCongressParams`
//!   - `CommitteeByCongressChamberParams`
//!   - `CommitteeDetailsParams`
//!   - `CommitteeBillsParams`
//!   - `CommitteeReportsParams`
//!   - `CommitteeNominationsParams`
//!   - `CommitteeHouseCommunicationParams`
//!   - `CommitteeSenateCommunicationParams`
//! 
//! - **Nomination Endpoints**:
//!   - `NominationListParams`
//!   - `NominationByCongressParams`
//!   - `NominationDetailsParams`
//!   - `NomineesParams`
//!   - `NominationActionsParams`
//!   - `NominationCommitteesParams`
//!   - `NominationHearingsParams`
//! 
//! - **Treaty Endpoints**:
//!   - `TreatyListParams`
//!   - `TreatyByCongressParams`
//!   - `TreatyDetailsParams`
//!   - `TreatyPartitionedParams`
//!   - `TreatyCommitteesParams`
//!   - `TreatyActionsParams`
//! 
//! - **Summaries Endpoints**:
//!   - `SummariesListParams`
//!   - `SummariesByCongressParams`
//!   - `SummariesByTypeParams`
//! 
//! - **Congress Endpoints**:
//!   - `CongressListParams`
//!   - `CongressDetailsParams`
//!   - `CongressCurrentParams`
//! 
//! Each parameter struct typically includes fields such as `format`, `offset`, `limit`, `from_date_time`, `to_date_time`, and `sort` to control the API request behavior.
//! 
//! ## Example
//! 
//! ```rust
//! use cdg_api::param_models::{BillListParams, FormatType, SortType};
//! 
//! fn main() {
//!     let params = BillListParams {
//!         format: Some(FormatType::Json),
//!         limit: Some(10),
//!         from_date_time: Some("2023-01-01".to_string()),
//!         to_date_time: Some("2023-12-31".to_string()),
//!         sort: Some(SortType::UpdateDateDesc),
//!         ..BillListParams::default()
//!     };
//!     // Use `params` with an endpoint constructor
//! }
//! ```

use serde::{Deserialize, Serialize};

// =========================================
// Enums for API Endpoint Parameters Values
// =========================================

/// Enum representing the response formats available for API endpoints.
///
/// This enum is used to specify the desired format of the API response,
/// allowing consumers to choose between JSON and XML formats.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    /// JSON format.
    #[default]
    Json,

    /// XML format.
    Xml,
}

impl FormatType {
    /// Converts the `FormatType` variant to its corresponding query parameter string.
    ///
    /// # Returns
    ///
    /// A `String` representing the query parameter (e.g., `"format=json"`).
    pub fn to_query_param(&self) -> String {
        match self {
            FormatType::Json => "format=json".to_string(),
            FormatType::Xml => "format=xml".to_string(),
        }
    }

    /// Converts the `FormatType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the format type (e.g., `"json"` or `"xml"`).
    pub fn to_string(&self) -> String {
        match self {
            FormatType::Json => "json".to_string(),
            FormatType::Xml => "xml".to_string(),
        }
    }
}

/// Enum representing the sorting options available for API endpoints.
///
/// This enum allows consumers to sort responses based on update dates
/// in ascending or descending order.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SortType {
    /// SortType by update date in ascending order.
    UpdateDateAsc,

    /// SortType by update date in descending order.
    UpdateDateDesc,
}

impl SortType {
    /// Converts the `SortType` variant to its corresponding query parameter string.
    ///
    /// # Returns
    ///
    /// A `String` representing the sort query parameter
    /// (e.g., `"sort=updateDate+asc"`).
    pub fn to_query_param(&self) -> String {
        match self {
            SortType::UpdateDateAsc => "sort=updateDate+asc".to_string(),
            SortType::UpdateDateDesc => "sort=updateDate+desc".to_string(),
        }
    }

    /// Converts the `SortType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the sort order (`"asc"` or `"desc"`).
    pub fn to_string(&self) -> String {
        match self {
            SortType::UpdateDateAsc => "asc".to_string(),
            SortType::UpdateDateDesc => "desc".to_string(),
        }
    }
}

/// Enum representing various bill types in legislative processes.
///
/// This enum categorizes bills based on their origin and nature within
/// the legislative system.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum BillType {
    /// House Resolution (`hr`).
    #[default]
    Hr, // House Resolution

    /// Senate Bill (`s`).
    S, // Senate

    /// House Joint Resolution (`hjres`).
    Hjres, // House Joint Resolution

    /// Senate Joint Resolution (`sjres`).
    Sjres, // Senate Joint Resolution

    /// House Concurrent Resolution (`hconres`).
    Hconres, // House Concurrent Resolution

    /// Senate Concurrent Resolution (`sconres`).
    Sconres, // Senate Concurrent Resolution

    /// House Simple Resolution (`hres`).
    Hres, // House Simple Resolution

    /// Senate Simple Resolution (`sres`).
    Sres, // Senate Simple Resolution
}

impl BillType {
    /// Converts the `BillType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the bill type (e.g., `"hr"`, `"s"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            BillType::Hr => "hr".to_string(),
            BillType::S => "s".to_string(),
            BillType::Hjres => "hjres".to_string(),
            BillType::Sjres => "sjres".to_string(),
            BillType::Hconres => "hconres".to_string(),
            BillType::Sconres => "sconres".to_string(),
            BillType::Hres => "hres".to_string(),
            BillType::Sres => "sres".to_string(),
        }
    }

    /// Converts a `&str` to the corresponding `BillType` variant.
    pub fn from_str(s: &str) -> Option<BillType> {
        match s.to_lowercase().as_str() {
            "hr" => Some(BillType::Hr),
            "s" => Some(BillType::S),
            "hjres" => Some(BillType::Hjres),
            "sjres" => Some(BillType::Sjres),
            "hconres" => Some(BillType::Hconres),
            "sconres" => Some(BillType::Sconres),
            "hres" => Some(BillType::Hres),
            "sres" => Some(BillType::Sres),
            _ => None,
        }
    }
}

/// Enum representing the types of amendments in legislative processes.
///
/// This enum categorizes amendments based on their origin within
/// the legislative chambers.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AmendmentType {
    /// House Amendment (`hamdt`).
    Hamdt, // House Amendment

    /// Senate Amendment (`samdt`).
    Samdt, // Senate Amendment

    /// Senate Unnumbered Amendment (`suamdt`).
    Suamdt, // Senate Unnumbered Amendment
}

impl AmendmentType {
    /// Converts the `AmendmentType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the amendment type (e.g., `"hamdt"`, `"samdt"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            AmendmentType::Hamdt => "hamdt".to_string(),
            AmendmentType::Samdt => "samdt".to_string(),
            AmendmentType::Suamdt => "suamdt".to_string(),
        }
    }
}

/// Enum representing the chambers of Congress.
///
/// This enum distinguishes between the House, Senate, and Joint committees
/// within the legislative body.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChamberType {
    /// House chamber.
    House,

    /// Senate chamber.
    Senate,

    /// Joint chamber (both House and Senate).
    Joint,
}

impl ChamberType {
    /// Converts the `ChamberType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the chamber type (e.g., `"house"`, `"senate"`, `"joint"`).
    pub fn to_string(&self) -> String {
        match self {
            ChamberType::House => "house".to_string(),
            ChamberType::Senate => "senate".to_string(),
            ChamberType::Joint => "joint".to_string(),
        }
    }
}

/// Enum representing different types of communications handled by committees.
///
/// This enum categorizes the various communication types such as executive
/// communications, presidential messages, petitions, etc.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CommunicationType {
    /// Executive Communication (`ec`).
    Ec, // Executive Communication

    /// Message from the President (`ml`).
    Ml, // Message from the President

    /// Presidential Message (`pm`).
    Pm, // Presidential Message

    /// Petition (`pt`).
    Pt, // Petition
}

impl CommunicationType {
    /// Converts the `CommunicationType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the communication type (e.g., `"ec"`, `"ml"`, etc.).
    pub fn to_string(&self) -> String {
        match self {
            CommunicationType::Ec => "ec".to_string(),
            CommunicationType::Ml => "ml".to_string(),
            CommunicationType::Pm => "pm".to_string(),
            CommunicationType::Pt => "pt".to_string(),
        }
    }
}

/// Enum representing different types of laws.
///
/// This enum distinguishes between public and private laws.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LawType {
    /// Public Law (`pub`).
    Pub,  // Public Law

    /// Private Law (`priv`).
    Priv, // Private Law
}

impl LawType {
    /// Converts the `LawType` variant to its lowercase string representation.
    ///
    /// # Returns
    ///
    /// A `String` representing the law type (`"pub"` or `"priv"`).
    pub fn to_string(&self) -> String {
        match self {
            LawType::Pub => "pub".to_string(),
            LawType::Priv => "priv".to_string(),
        }
    }
}

// ================================
// Endpoint-Specific Parameter Structs
// ================================

/// Parameters for the `BillList` endpoint.
///
/// These parameters allow filtering and pagination when listing bills.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `BillByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving bills
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `BillByType` endpoint.
///
/// These parameters allow filtering and pagination when retrieving bills
/// of a specific type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering bills.
    pub from_date_time: Option<String>,

    /// End date-time for filtering bills.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `BillDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `BillActions` endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `BillAmendments` endpoint.
///
/// These parameters allow pagination when listing amendments of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillAmendmentsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `BillCommittees` endpoint.
///
/// These parameters allow pagination when retrieving committees associated with a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `BillCosponsors` endpoint.
///
/// These parameters allow pagination when retrieving cosponsors of a specific bill.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillCosponsorsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

// ================================
// Amendment Endpoints Parameters
// ================================

/// Parameters for the `AmendmentList` endpoint.
///
/// These parameters allow filtering and pagination when listing amendments.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `AmendmentByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving amendments
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `AmendmentByType` endpoint.
///
/// These parameters allow filtering and pagination when retrieving amendments
/// of a specific type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering amendments.
    pub from_date_time: Option<String>,

    /// End date-time for filtering amendments.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `AmendmentDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `AmendmentActions` endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `AmendmentCosponsors` endpoint.
///
/// These parameters allow pagination when retrieving cosponsors of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentCosponsorsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `AmendmentAmendments` endpoint.
///
/// These parameters allow pagination when listing amendments of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentAmendmentsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `AmendmentText` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving the text of a specific amendment.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AmendmentTextParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

// ================================
// Law Endpoints Parameters
// ================================

/// Parameters for the `LawList` endpoint.
///
/// These parameters allow filtering and pagination when listing laws.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LawParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}


// ================================
// Member Endpoints Parameters
// ================================

/// Parameters for the `MemberList` endpoint.
///
/// These parameters allow filtering and pagination when listing members of Congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering members.
    pub from_date_time: Option<String>,

    /// End date-time for filtering members.
    pub to_date_time: Option<String>,

    /// 
    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the `MemberByState` endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// representing a specific state.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByStateParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the `MemberByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// associated with a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the `MemberByCongressStateDistrict` endpoint.
///
/// These parameters allow filtering and pagination when retrieving members
/// by congress, state, and district.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberByCongressStateDistrictParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Filter to include only current members.
    pub current_member: Option<bool>,
}

/// Parameters for the `MemberDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MemberDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `SponsorshipList` endpoint.
///
/// These parameters allow pagination when listing sponsorships of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SponsorshipListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CosponsorshipList` endpoint.
///
/// These parameters allow pagination when listing cosponsorships of a specific member.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CosponsorshipListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

// ================================
// Committee Endpoints Parameters
// ================================

/// Parameters for the `CommitteeList` endpoint.
///
/// These parameters allow filtering and pagination when listing committees.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the `CommitteeByChamber` endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// by chamber (House, Senate, or Joint).
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the `CommitteeByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// associated with a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the `CommitteeByCongressChamber` endpoint.
///
/// These parameters allow filtering and pagination when retrieving committees
/// by both congress and chamber.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeByCongressChamberParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering committees.
    pub from_date_time: Option<String>,

    /// End date-time for filtering committees.
    pub to_date_time: Option<String>,
}

/// Parameters for the `CommitteeDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `CommitteeBills` endpoint.
///
/// These parameters allow pagination when listing bills under a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeBillsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CommitteeReports` endpoint.
///
/// These parameters allow pagination when retrieving reports from a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeReportsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CommitteeNominations` endpoint.
///
/// These parameters allow pagination when listing nominations handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeNominationsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CommitteeHouseCommunication` endpoint.
///
/// These parameters allow pagination when retrieving house communications handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeHouseCommunicationParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CommitteeSenateCommunication` endpoint.
///
/// These parameters allow pagination when retrieving senate communications handled by a specific committee.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommitteeSenateCommunicationParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

// ================================
// Nomination Endpoints Parameters
// ================================

/// Parameters for the `NominationList` endpoint.
///
/// These parameters allow filtering and pagination when listing nominations.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering nominations.
    pub from_date_time: Option<String>,

    /// End date-time for filtering nominations.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `NominationByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving nominations
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering nominations.
    pub from_date_time: Option<String>,

    /// End date-time for filtering nominations.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `NominationDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `Nominees` endpoint.
///
/// These parameters allow pagination when listing nominees of a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NomineesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `NominationActions` endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `NominationCommittees` endpoint.
///
/// These parameters allow pagination when retrieving committees involved in a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `NominationHearings` endpoint.
///
/// These parameters allow pagination when listing hearings related to a specific nomination.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NominationHearingsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

// ================================
// Treaty Endpoints Parameters
// ================================

/// Parameters for the `TreatyList` endpoint.
///
/// These parameters allow filtering and pagination when listing treaties.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering treaties.
    pub from_date_time: Option<String>,

    /// End date-time for filtering treaties.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `TreatyByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving treaties
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering treaties.
    pub from_date_time: Option<String>,

    /// End date-time for filtering treaties.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `TreatyDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `TreatyPartitioned` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving partitioned information about a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyPartitionedParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `TreatyCommittees` endpoint.
///
/// These parameters allow pagination when retrieving committees associated with a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyCommitteesParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `TreatyActions` endpoint.
///
/// These parameters allow pagination when fetching actions taken on a specific treaty.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TreatyActionsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

// ----------------
// Summaries Endpoints Structs
// ----------------

/// Parameters for the `SummariesList` endpoint.
///
/// These parameters allow filtering and pagination when listing summaries.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering summaries.
    pub from_date_time: Option<String>,

    /// End date-time for filtering summaries.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `SummariesByCongress` endpoint.
///
/// These parameters allow filtering and pagination when retrieving summaries
/// for a specific congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesByCongressParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

    /// Start date-time for filtering summaries.
    pub from_date_time: Option<String>,

    /// End date-time for filtering summaries.
    pub to_date_time: Option<String>,

    /// Sorting option for the results.
    pub sort: Option<SortType>,
}

/// Parameters for the `SummariesByType` endpoint.
///
/// These parameters allow filtering and pagination when retrieving summaries
/// of a specific bill type within a congress.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SummariesByTypeParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,

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

/// Parameters for the `CongressList` endpoint.
///
/// These parameters allow filtering and pagination when listing congress sessions.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressListParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}

/// Parameters for the `CongressDetails` endpoint.
///
/// These parameters allow specifying the desired response format
/// when retrieving details of a specific congress session.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressDetailsParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,
}

/// Parameters for the `CongressCurrent` endpoint.
///
/// These parameters allow filtering and pagination when retrieving information
/// about the current congress session.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CongressCurrentParams {
    /// Desired response format (JSON or XML).
    pub format: Option<FormatType>,

    /// Number of items to skip before starting to collect the result set.
    pub offset: Option<i32>,

    /// Maximum number of items to return.
    pub limit: Option<i32>,
}
