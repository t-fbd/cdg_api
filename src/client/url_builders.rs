//! # [`url_builders`] Module
//!
//! The [`url_builders`] module constructs complete URLs for API requests to the US Congress API. It leverages
//! the [`ApiParams`] trait to convert parameter structs into query strings and integrates them with endpoint
//! paths using the [`Endpoints`] enum.
//!
//! ## Traits
//!
//! - **[`ApiParams`]**: Defines a method to convert parameters into a query string.
//!
//! ## Implementations
//!
//! [`ApiParams`] is implemented for various parameter structs (e.g., [`BillListParams`], [`AmendmentListParams`]),
//! enabling each to generate its corresponding query string.
//!
//! The [`Display`] trait is implemented for [`Endpoints`], allowing seamless URL construction by combining
//! endpoint paths with their query parameters.
//!
//! ## Key Functions
//!
//! - **[`generate_url`]**: Combines the base URL, endpoint, query parameters, and API key to form the full request URL.
//!
//! ## Example
//!
//! ```rust
//! use cdg_api::param_models::BillListParams;
//! use cdg_api::cdg_types::{FormatType, SortType};
//! use cdg_api::endpoints::Endpoints;
//! use cdg_api::url_builders::generate_url;
//!
//! fn main() {
//!     let params = BillListParams::default()
//!         .format(FormatType::Json)
//!         .limit(10)
//!         //Use format: YYYY-MM-DDT00:00:00Z
//!         .from_date_time("2023-01-01T00:00:00Z".to_string())
//!         .to_date_time("2023-12-31T00:00:00Z".to_string())
//!         .sort(SortType::UpdateDateDesc);
//!
//!     let endpoint = Endpoints::BillList(params);
//!     let api_key = "YOUR_API_KEY";
//!     let url = generate_url(endpoint, api_key);
//!
//!     println!("URL: {}", url);
//!     // Output: https://api.congress.gov/v3/bill/?format=json&limit=10&fromDateTime=2023-01-01&toDateTime=2023-12-31&sort=updateDateDesc&api_key=YOUR_API_KEY
//! }
//! ```
//!
//! ## Summary
//!
//! - **Type Safety**: Ensures correct URL formation through trait implementations and enums.
//! - **Extensibility**: Easily add new endpoints by implementing [`ApiParams`] for new parameter structs.
//! - **Convenience**: Simplifies URL construction using Rust's formatting capabilities.

use crate::endpoints::Endpoints;
use crate::param_models::*;
use std::fmt::Display;

/// Called by the api client to generate the complete URL for the request.
/// If the endpoint contains a '?' character, the URL is generated with '&'
/// as the separator for the query parameters.
///
/// Otherwise, the URL is generated with '?' as the separator for the query parameters.
/// The API key is appended to the URL as a query parameter.
///
/// This works due to the [`Display`] implementation for the [`Endpoints`] enum, as well as
/// the [`Display`] implementation for the [`ApiParam`] structs. These both convert the existing
/// data into a query string.
pub fn generate_url(endpoint: Endpoints, api_key: &str) -> String {
    if endpoint.to_string().contains("?") {
        format!("{}{}&api_key={}", crate::BASE_URL, endpoint, api_key)
    } else {
        format!("{}{}?api_key={}", crate::BASE_URL, endpoint, api_key)
    }
}

/// Implementation of the [`Display`] trait for the [`Endpoints`] enum.
///
/// This, in conjunction with the [`Display`] implementation for the
/// [`ApiParam`] structs, allows for easy conversion of [`Endpoints`] to
/// a URL string.
///
/// For example:
///
/// If we have an endpoint like `Endpoints::BillList(params)`, where `params`
/// is a [`BillListParams`] struct with [`format`] set to `FormatType::Json` and `limit`
/// set to [`10`], the URL string would look like:
/// `bill/?format=json&limit=10`
impl std::fmt::Display for Endpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // NOTE: A '?' is appended to the params string via the `Display`
        // implementation for the [`ApiParam`] structs.
        match self {
            Endpoints::Generic(endpoint, params) => {
                write!(f, "{}/{}", endpoint, params.to_query_string())
            }
            // ================================
            // Bill Endpoints
            // ================================
            Endpoints::BillList(params) => write!(f, "bill/?{}", params.to_query_string()),
            Endpoints::BillByCongress(congress, params) => {
                write!(f, "bill/{}{}", congress, params.to_query_string())
            }
            Endpoints::BillByType(congress, bill_type, params) => {
                write!(
                    f,
                    "bill/{}/{}{}",
                    congress,
                    bill_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::BillDetails(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillActions(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/actions{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillAmendments(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/amendments{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillCommittees(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/committees{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillCosponsors(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/cosponsors{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillRelated(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/relatedbills{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillSubjects(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/subjects{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillSummaries(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/summary{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillText(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/text{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }
            Endpoints::BillTitles(congress, bill_type, bill_number, params) => {
                write!(
                    f,
                    "bill/{}/{}/{}/titles{}",
                    congress,
                    bill_type.to_string(),
                    bill_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Law Endpoints
            // ================================
            Endpoints::LawByType(congress, law_type, params) => {
                write!(
                    f,
                    "law/{}/{}{}",
                    congress,
                    law_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::LawByCongress(congress, params) => {
                write!(f, "law/{}{}", congress, params.to_query_string())
            }
            Endpoints::LawDetails(congress, law_type, law_number, params) => {
                write!(
                    f,
                    "law/{}/{}/{}{}",
                    congress,
                    law_type.to_string(),
                    law_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Amendment Endpoints
            // ================================
            Endpoints::AmendmentList(params) => write!(f, "amendment{}", params.to_query_string()),
            Endpoints::AmendmentByCongress(congress, params) => {
                write!(f, "amendment/{}{}", congress, params.to_query_string())
            }
            Endpoints::AmendmentByType(congress, amendment_type, params) => {
                write!(
                    f,
                    "amendment/{}/{}{}",
                    congress,
                    amendment_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::AmendmentDetails(congress, amendment_type, amendment_number, params) => {
                write!(
                    f,
                    "amendment/{}/{}/{}{}",
                    congress,
                    amendment_type.to_string(),
                    amendment_number,
                    params.to_query_string()
                )
            }
            Endpoints::AmendmentActions(congress, amendment_type, amendment_number, params) => {
                write!(
                    f,
                    "amendment/{}/{}/{}/actions{}",
                    congress,
                    amendment_type.to_string(),
                    amendment_number,
                    params.to_query_string()
                )
            }
            Endpoints::AmendmentCosponsors(congress, amendment_type, amendment_number, params) => {
                write!(
                    f,
                    "amendment/{}/{}/{}/cosponsors{}",
                    congress,
                    amendment_type.to_string(),
                    amendment_number,
                    params.to_query_string()
                )
            }
            Endpoints::AmendmentAmendments(congress, amendment_type, amendment_number, params) => {
                write!(
                    f,
                    "amendment/{}/{}/{}/amendments{}",
                    congress,
                    amendment_type.to_string(),
                    amendment_number,
                    params.to_query_string()
                )
            }
            Endpoints::AmendmentText(congress, amendment_type, amendment_number, params) => {
                write!(
                    f,
                    "amendment/{}/{}/{}/text{}",
                    congress,
                    amendment_type.to_string(),
                    amendment_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Summaries Endpoints
            // ================================
            Endpoints::SummariesList(params) => write!(f, "summary{}", params.to_query_string()),
            Endpoints::SummariesByCongress(congress, params) => {
                write!(f, "summary/{}{}", congress, params.to_query_string())
            }
            Endpoints::SummariesByType(congress, bill_type, params) => {
                write!(
                    f,
                    "summary/{}/{}{}",
                    congress,
                    bill_type.to_string(),
                    params.to_query_string()
                )
            }

            // ================================
            // Congress Endpoints
            // ================================
            Endpoints::CongressList(params) => write!(f, "congress{}", params.to_query_string()),
            Endpoints::CongressDetails(congress, params) => {
                write!(f, "congress/{}{}", congress, params.to_query_string())
            }
            Endpoints::CongressCurrent(params) => {
                write!(f, "congress/current{}", params.to_query_string())
            }

            // ================================
            // Member Endpoints
            // ================================
            Endpoints::MemberList(params) => write!(f, "member{}", params.to_query_string()),
            Endpoints::MemberByCongress(congress, params) => {
                write!(
                    f,
                    "member/congress/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::MemberByState(state_code, params) => {
                write!(f, "member/{}{}", state_code, params.to_query_string())
            }
            Endpoints::MemberByStateDistrict(state_code, district, params) => {
                write!(
                    f,
                    "member/{}/{}{}",
                    state_code,
                    district,
                    params.to_query_string()
                )
            }
            Endpoints::MemberByCongressStateDistrict(congress, state_code, district, params) => {
                write!(
                    f,
                    "member/congress/{}/{}/{}{}",
                    congress,
                    state_code,
                    district,
                    params.to_query_string()
                )
            }
            Endpoints::MemberDetails(bio_guide_id, params) => {
                write!(f, "member/{}{}", bio_guide_id, params.to_query_string())
            }
            Endpoints::SponsorshipList(bio_guide_id, params) => {
                write!(
                    f,
                    "member/{}/sponsored-legislation{}",
                    bio_guide_id,
                    params.to_query_string()
                )
            }
            Endpoints::CosponsorshipList(bio_guide_id, params) => {
                write!(
                    f,
                    "member/{}/cosponsored-legislation{}",
                    bio_guide_id,
                    params.to_query_string()
                )
            }

            // ================================
            // Committee Endpoints
            // ================================
            Endpoints::CommitteeList(params) => write!(f, "committee?{}", params.to_query_string()),
            Endpoints::CommitteeByChamber(chamber, params) => {
                write!(
                    f,
                    "committee/chamber/{}{}",
                    chamber.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeByCongress(congress, params) => {
                write!(f, "committee/{}{}", congress, params.to_query_string())
            }
            Endpoints::CommitteeByCongressChamber(congress, chamber, params) => {
                write!(
                    f,
                    "committee/{}/{}{}",
                    congress,
                    chamber.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeDetails(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeBills(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}/bills{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeReports(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}/reports{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeNominations(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}/nominations{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeHouseCommunication(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}/house-communication{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeSenateCommunication(chamber, committee_code, params) => {
                write!(
                    f,
                    "committee/{}/{}/senate-communication{}",
                    chamber.to_string(),
                    committee_code,
                    params.to_query_string()
                )
            }

            // ================================
            // Committee Report Endpoints
            // ================================
            Endpoints::CommitteeReportList(params) => {
                write!(f, "committee-report{}", params.to_query_string())
            }
            Endpoints::CommitteeReportByCongress(congress, params) => {
                write!(
                    f,
                    "committee-report/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeReportByType(congress, report_type, params) => {
                write!(
                    f,
                    "committee-report/{}/{}{}",
                    congress,
                    report_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeReportDetails(congress, report_type, report_number, params) => {
                write!(
                    f,
                    "committee-report/{}/{}/{}{}",
                    congress,
                    report_type.to_string(),
                    report_number,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeReportText(congress, report_type, report_number, params) => {
                write!(
                    f,
                    "committee-report/{}/{}/{}/text{}",
                    congress,
                    report_type.to_string(),
                    report_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Committee Print Endpoints
            // ================================
            Endpoints::CommitteePrintList(params) => {
                write!(f, "committee-print{}", params.to_query_string())
            }
            Endpoints::CommitteePrintByCongress(congress, params) => {
                write!(
                    f,
                    "committee-print/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteePrintByCongressChamber(congress, chamber, params) => {
                write!(
                    f,
                    "committee-print/{}/{}{}",
                    congress,
                    chamber.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::CommitteePrintByJacketNumber(congress, jacket_number, params) => {
                write!(
                    f,
                    "committee-print/{}/{}{}",
                    congress,
                    jacket_number,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteePrintText(congress, chamber, jacket_number, params) => {
                write!(
                    f,
                    "committee-print/{}/{}/{}/text{}",
                    congress,
                    chamber.to_string(),
                    jacket_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Committee Meeting Endpoints
            // ================================
            Endpoints::CommitteeMeetingList(params) => {
                write!(f, "committee-meeting{}", params.to_query_string())
            }
            Endpoints::CommitteeMeetingByCongress(congress, params) => {
                write!(
                    f,
                    "committee-meeting/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeMeetingByChamber(congress, chamber, params) => {
                write!(
                    f,
                    "committee-meeting/{}/{}{}",
                    congress,
                    chamber.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::CommitteeMeetingByEvent(congress, chamber, event_id, params) => {
                write!(
                    f,
                    "committee-meeting/{}/{}/{}/{}",
                    congress,
                    chamber.to_string(),
                    event_id,
                    params.to_query_string()
                )
            }

            // ================================
            // Hearing Endpoints
            // ================================
            Endpoints::HearingList(params) => write!(f, "hearing{}", params.to_query_string()),
            Endpoints::HearingByCongress(congress, params) => {
                write!(f, "hearing/{}{}", congress, params.to_query_string())
            }
            Endpoints::HearingByChamber(congress, chamber, params) => {
                write!(
                    f,
                    "hearing/{}/{}{}",
                    congress,
                    chamber.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::HearingByJacketNumber(congress, chamber, jacket_number, params) => {
                write!(
                    f,
                    "hearing/{}/{}/{}{}",
                    congress,
                    chamber.to_string(),
                    jacket_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Congressional Record Endpoints
            // ================================
            Endpoints::CongressionalRecordList(params) => {
                write!(f, "congressional-record{}", params.to_query_string())
            }

            // ================================
            // Daily Congressional Record Endpoints
            // ================================
            Endpoints::DailyCongressionalRecordList(params) => {
                write!(f, "daily-congressional-record{}", params.to_query_string())
            }
            Endpoints::DailyCongressionalRecordVolume(volume, params) => {
                write!(
                    f,
                    "daily-congressional-record/{}{}",
                    volume,
                    params.to_query_string()
                )
            }
            Endpoints::DailyCongressionalRecordVolumeIssue(volume, issue, params) => {
                write!(
                    f,
                    "daily-congressional-record/{}/{}{}",
                    volume,
                    issue,
                    params.to_query_string()
                )
            }
            Endpoints::DailyCongressionalRecordArticles(volume, issue, params) => {
                write!(
                    f,
                    "daily-congressional-record/{}/{}{}",
                    volume,
                    issue,
                    params.to_query_string()
                )
            }

            // ====================================
            // Bound Congressional Record Endpoints
            // ====================================
            Endpoints::BoundCongressionalRecordList(params) => {
                write!(f, "bound-congressional-record{}", params.to_query_string())
            }
            Endpoints::BoundCongressionalRecordByYear(year, params) => {
                write!(
                    f,
                    "bound-congressional-record/{}{}",
                    year,
                    params.to_query_string()
                )
            }
            Endpoints::BoundCongressionalRecordByYearMonth(year, month, params) => {
                write!(
                    f,
                    "bound-congressional-record/{}/{}{}",
                    year,
                    month,
                    params.to_query_string()
                )
            }
            Endpoints::BoundCongressionalRecordByYearMonthDay(year, month, day, params) => {
                write!(
                    f,
                    "bound-congressional-record/{}/{}/{}{}",
                    year,
                    month,
                    day,
                    params.to_query_string()
                )
            }

            // ======================================================
            // House Communication and Senate Communication Endpoints
            // ======================================================
            Endpoints::HouseCommunicationList(params) => {
                write!(f, "house-communication{}", params.to_query_string())
            }
            Endpoints::HouseCommunicationByCongress(congress, params) => {
                write!(
                    f,
                    "house-communication/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::HouseCommunicationByType(congress, communication_type, params) => {
                write!(
                    f,
                    "house-communication/{}/{}{}",
                    congress,
                    communication_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::HouseCommunicationDetails(
                congress,
                communication_type,
                communication_number,
                params,
            ) => {
                write!(
                    f,
                    "house-communication/{}/{}/{}{}",
                    congress,
                    communication_type.to_string(),
                    communication_number,
                    params.to_query_string()
                )
            }

            Endpoints::SenateCommunicationList(params) => {
                write!(f, "senate-communication{}", params.to_query_string())
            }
            Endpoints::SenateCommunicationByCongress(congress, params) => {
                write!(
                    f,
                    "senate-communication/{}{}",
                    congress,
                    params.to_query_string()
                )
            }
            Endpoints::SenateCommunicationByType(congress, communication_type, params) => {
                write!(
                    f,
                    "senate-communication/{}/{}{}",
                    congress,
                    communication_type.to_string(),
                    params.to_query_string()
                )
            }
            Endpoints::SenateCommunicationDetails(
                congress,
                communication_type,
                communication_number,
                params,
            ) => {
                write!(
                    f,
                    "senate-communication/{}/{}/{}{}",
                    congress,
                    communication_type.to_string(),
                    communication_number,
                    params.to_query_string()
                )
            }

            // ================================
            // House Requirement Endpoints
            // ================================
            Endpoints::HouseRequirementList(params) => {
                write!(f, "house-requirement{}", params.to_query_string())
            }
            Endpoints::HouseRequirementDetails(requirement_number, params) => {
                write!(
                    f,
                    "house-requirement/{}{}",
                    requirement_number,
                    params.to_query_string()
                )
            }
            Endpoints::HouseRequirementMatching(requirement_number, params) => {
                write!(
                    f,
                    "house-requirement/{}/matching{}",
                    requirement_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Nomination Endpoints
            // ================================
            Endpoints::NominationList(params) => {
                write!(f, "nomination{}", params.to_query_string())
            }
            Endpoints::NominationByCongress(congress, params) => {
                write!(f, "nomination/{}{}", congress, params.to_query_string())
            }
            Endpoints::NominationDetails(congress, nomination_number, params) => {
                write!(
                    f,
                    "nomination/{}/{}{}",
                    congress,
                    nomination_number,
                    params.to_query_string()
                )
            }
            Endpoints::Nominees(congress, nomination_number, ordinal, params) => {
                write!(
                    f,
                    "nomination/{}/{}/{}{}",
                    congress,
                    nomination_number,
                    ordinal,
                    params.to_query_string()
                )
            }
            Endpoints::NominationActions(congress, nomination_number, params) => {
                write!(
                    f,
                    "nomination/{}/{}/actions{}",
                    congress,
                    nomination_number,
                    params.to_query_string()
                )
            }
            Endpoints::NominationCommittees(congress, nomination_number, params) => {
                write!(
                    f,
                    "nomination/{}/{}/committees{}",
                    congress,
                    nomination_number,
                    params.to_query_string()
                )
            }
            Endpoints::NominationHearings(congress, nomination_number, params) => {
                write!(
                    f,
                    "nomination/{}/{}/hearings{}",
                    congress,
                    nomination_number,
                    params.to_query_string()
                )
            }

            // ================================
            // Treaty Endpoints
            // ================================
            Endpoints::TreatyList(params) => write!(f, "treaty{}", params.to_query_string()),
            Endpoints::TreatyByCongress(congress, params) => {
                write!(f, "treaty/{}{}", congress, params.to_query_string())
            }
            Endpoints::TreatyDetails(congress, treaty_number, params) => {
                write!(
                    f,
                    "treaty/{}/{}{}",
                    congress,
                    treaty_number,
                    params.to_query_string()
                )
            }
            Endpoints::TreatyPartitioned(congress, treaty_number, treaty_suffix, params) => {
                write!(
                    f,
                    "treaty/{}/{}/{}{}",
                    congress,
                    treaty_number,
                    treaty_suffix,
                    params.to_query_string()
                )
            }
            Endpoints::TreatyCommittees(congress, treaty_number, params) => {
                write!(
                    f,
                    "treaty/{}/{}/committees{}",
                    congress,
                    treaty_number,
                    params.to_query_string()
                )
            }
            Endpoints::TreatyActions(congress, treaty_number, params) => {
                write!(
                    f,
                    "treaty/{}/{}/actions{}",
                    congress,
                    treaty_number,
                    params.to_query_string()
                )
            }
            Endpoints::TreatyActionsBySuffix(congress, treaty_number, treaty_suffix, params) => {
                write!(
                    f,
                    "treaty/{}/{}/{}/actions{}",
                    congress,
                    treaty_number,
                    treaty_suffix,
                    params.to_query_string()
                )
            }
        }
    }
}

// ================================
// API Parameters

/// Trait representing Parameters for API Endpoints.
pub trait ApiParams {
    /// Converts the parameters to a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] representing the query parameters for the API endpoint.
    ///
    /// An enpoint, lets say for listing bills, would have a query string like:
    /// `?format=json&limit=10`
    /// if the parameters are set as:
    /// ```rust
    /// use cdg_api::param_models::BillListParams;
    /// use cdg_api::cdg_types::FormatType;
    ///
    /// let params = BillListParams {
    ///    format: Some(FormatType::Json),
    ///    limit: Some(10),
    ///    ..Default::default()
    /// };
    fn to_query_string(&self) -> String;
}

/// Display implementation for [`ApiParams`].
///
/// This implementation allows [`ApiParams`] to be displayed as a query string when
/// using the `format!` macro or similar.
impl Display for dyn ApiParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.to_query_string())
    }
}

// ================================
// Bill Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`BillListParams`].
///
/// Converts [`BillListParams`] into a query string suitable for the [`BillList`] endpoint.
impl ApiParams for BillListParams {
    /// Converts the [`BillListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing bills.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillByCongressParams`].
///
/// Converts [`BillByCongressParams`] into a query string suitable for the [`BillByCongress`] endpoint.
impl ApiParams for BillByCongressParams {
    /// Converts the [`BillByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving bills by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillByTypeParams`].
///
/// Converts [`BillByTypeParams`] into a query string suitable for the [`BillByType`] endpoint.
impl ApiParams for BillByTypeParams {
    /// Converts the [`BillByTypeParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving bills by type.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillDetailsParams`].
///
/// Converts [`BillDetailsParams`] into a query string suitable for the [`BillDetails`] endpoint.
impl ApiParams for BillDetailsParams {
    /// Converts the [`BillDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for bill details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`BillActionsParams`].
///
/// Converts [`BillActionsParams`] into a query string suitable for the [`BillActions`] endpoint.
impl ApiParams for BillActionsParams {
    /// Converts the [`BillActionsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill actions.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillAmendmentsParams`].
///
/// Converts [`BillAmendmentsParams`] into a query string suitable for the [`BillAmendments`] endpoint.
impl ApiParams for BillAmendmentsParams {
    /// Converts the [`BillAmendmentsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill amendments.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillCommitteesParams`].
///
/// Converts [`BillCommitteesParams`] into a query string suitable for the [`BillCommittees`] endpoint.
impl ApiParams for BillCommitteesParams {
    /// Converts the [`BillCommitteesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill committees.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillCosponsorsParams`].
///
/// Converts [`BillCosponsorsParams`] into a query string suitable for the [`BillCosponsors`] endpoint.
impl ApiParams for BillCosponsorsParams {
    /// Converts the [`BillCosponsorsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill cosponsors.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Amendment Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`AmendmentListParams`].
///
/// Converts [`AmendmentListParams`] into a query string suitable for the [`AmendmentList`] endpoint.
impl ApiParams for AmendmentListParams {
    /// Converts the [`AmendmentListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing amendments.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentByCongressParams`].
///
/// Converts [`AmendmentByCongressParams`] into a query string suitable for the [`AmendmentByCongress`] endpoint.
impl ApiParams for AmendmentByCongressParams {
    /// Converts the [`AmendmentByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving amendments by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentByTypeParams`].
///
/// Converts [`AmendmentByTypeParams`] into a query string suitable for the [`AmendmentByType`] endpoint.
impl ApiParams for AmendmentByTypeParams {
    /// Converts the [`AmendmentByTypeParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving amendments by type.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentDetailsParams`].
///
/// Converts [`AmendmentDetailsParams`] into a query string suitable for the [`AmendmentDetails`] endpoint.
impl ApiParams for AmendmentDetailsParams {
    /// Converts the [`AmendmentDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for amendment details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentActionsParams`].
///
/// Converts [`AmendmentActionsParams`] into a query string suitable for the [`AmendmentActions`] endpoint.
impl ApiParams for AmendmentActionsParams {
    /// Converts the [`AmendmentActionsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for amendment actions.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentCosponsorsParams`].
///
/// Converts [`AmendmentCosponsorsParams`] into a query string suitable for the [`AmendmentCosponsors`] endpoint.
impl ApiParams for AmendmentCosponsorsParams {
    /// Converts the [`AmendmentCosponsorsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for amendment cosponsors.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentAmendmentsParams`].
///
/// Converts [`AmendmentAmendmentsParams`] into a query string suitable for the [`AmendmentAmendments`] endpoint.
impl ApiParams for AmendmentAmendmentsParams {
    /// Converts the [`AmendmentAmendmentsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for amendment amendments.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`AmendmentTextParams`].
///
/// Converts [`AmendmentTextParams`] into a query string suitable for the [`AmendmentText`] endpoint.
impl ApiParams for AmendmentTextParams {
    /// Converts the [`AmendmentTextParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for amendment text.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

// ================================
// Law Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`LawListParams`].
///
/// Converts [`LawListParams`] into a query string suitable for the [`LawList`] endpoint.
impl ApiParams for LawParams {
    /// Converts the [`LawListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing laws.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Member Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`MemberListParams`].
///
/// Converts [`MemberListParams`] into a query string suitable for the [`MemberList`] endpoint.
impl ApiParams for MemberListParams {
    /// Converts the [`MemberListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing members.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(current_member) = &self.current_member {
            query_params.push(format!("currentMember={}", current_member));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`MemberByCongressParams`].
///
/// Converts [`MemberByCongressParams`] into a query string suitable for the [`MemberByCongress`] endpoint.
impl ApiParams for MemberByCongressParams {
    /// Converts the [`MemberByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving members by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(current_member) = &self.current_member {
            query_params.push(format!("currentMember={}", current_member));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`MemberByStateParams`].
///
/// Converts [`MemberByStateParams`] into a query string suitable for the [`MemberByState`] endpoint.
impl ApiParams for MemberByStateParams {
    /// Converts the [`MemberByStateParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving members by state.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(current_member) = &self.current_member {
            query_params.push(format!("currentMember={}", current_member));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`MemberByCongressStateDistrictParams`].
///
/// Converts [`MemberByCongressStateDistrictParams`] into a query string suitable for the [`MemberByCongressStateDistrict`] endpoint.
impl ApiParams for MemberByCongressStateDistrictParams {
    /// Converts the [`MemberByCongressStateDistrictParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving members by congress, state, and district.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(current_member) = &self.current_member {
            query_params.push(format!("currentMember={}", current_member));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`MemberDetailsParams`].
///
/// Converts [`MemberDetailsParams`] into a query string suitable for the [`MemberDetails`] endpoint.
impl ApiParams for MemberDetailsParams {
    /// Converts the [`MemberDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for member details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`SponsorshipListParams`].
///
/// Converts [`SponsorshipListParams`] into a query string suitable for the [`SponsorshipList`] endpoint.
impl ApiParams for SponsorshipListParams {
    /// Converts the [`SponsorshipListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for sponsorship lists.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CosponsorshipListParams`].
///
/// Converts [`CosponsorshipListParams`] into a query string suitable for the [`CosponsorshipList`] endpoint.
impl ApiParams for CosponsorshipListParams {
    /// Converts the [`CosponsorshipListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for cosponsorship lists.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Committee Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CommitteeListParams`].
///
/// Converts [`CommitteeListParams`] into a query string suitable for the [`CommitteeList`] endpoint.
impl ApiParams for CommitteeListParams {
    /// Converts the [`CommitteeListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing committees.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeByChamberParams`].
///
/// Converts [`CommitteeByChamberParams`] into a query string suitable for the [`CommitteeByChamber`] endpoint.
impl ApiParams for CommitteeByChamberParams {
    /// Converts the [`CommitteeByChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving committees by chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeByCongressParams`].
///
/// Converts [`CommitteeByCongressParams`] into a query string suitable for the [`CommitteeByCongress`] endpoint.
impl ApiParams for CommitteeByCongressParams {
    /// Converts the [`CommitteeByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving committees by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeByCongressChamberParams`].
///
/// Converts [`CommitteeByCongressChamberParams`] into a query string suitable for the [`CommitteeByCongressChamber`] endpoint.
impl ApiParams for CommitteeByCongressChamberParams {
    /// Converts the [`CommitteeByCongressChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving committees by congress and chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeDetailsParams`].
///
/// Converts [`CommitteeDetailsParams`] into a query string suitable for the [`CommitteeDetails`] endpoint.
impl ApiParams for CommitteeDetailsParams {
    /// Converts the [`CommitteeDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for committee details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format.to_query_param()
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeBillsParams`].
///
/// Converts [`CommitteeBillsParams`] into a query string suitable for the [`CommitteeBills`] endpoint.
impl ApiParams for CommitteeBillsParams {
    /// Converts the [`CommitteeBillsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee bills.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeReportsParams`].
///
/// Converts [`CommitteeReportsParams`] into a query string suitable for the [`CommitteeReports`] endpoint.
impl ApiParams for CommitteeReportsParams {
    /// Converts the [`CommitteeReportsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee reports.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeNominationsParams`].
///
/// Converts [`CommitteeNominationsParams`] into a query string suitable for the [`CommitteeNominations`] endpoint.
impl ApiParams for CommitteeNominationsParams {
    /// Converts the [`CommitteeNominationsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee nominations.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeHouseCommunicationParams`].
///
/// Converts [`CommitteeHouseCommunicationParams`] into a query string suitable for the [`CommitteeHouseCommunication`] endpoint.
impl ApiParams for CommitteeHouseCommunicationParams {
    /// Converts the [`CommitteeHouseCommunicationParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee house communications.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeSenateCommunicationParams`].
///
/// Converts [`CommitteeSenateCommunicationParams`] into a query string suitable for the [`CommitteeSenateCommunication`] endpoint.
impl ApiParams for CommitteeSenateCommunicationParams {
    /// Converts the [`CommitteeSenateCommunicationParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee senate communications.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Nomination Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`NominationListParams`].
///
/// Converts [`NominationListParams`] into a query string suitable for the [`NominationList`] endpoint.
impl ApiParams for NominationListParams {
    /// Converts the [`NominationListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing nominations.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`NominationByCongressParams`].
///
/// Converts [`NominationByCongressParams`] into a query string suitable for the [`NominationByCongress`] endpoint.
impl ApiParams for NominationByCongressParams {
    /// Converts the [`NominationByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving nominations by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`NominationDetailsParams`].
///
/// Converts [`NominationDetailsParams`] into a query string suitable for the [`NominationDetails`] endpoint.
impl ApiParams for NominationDetailsParams {
    /// Converts the [`NominationDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for nomination details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`NomineesParams`].
///
/// Converts [`NomineesParams`] into a query string suitable for the [`Nominees`] endpoint.
impl ApiParams for NomineesParams {
    /// Converts the [`NomineesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for nominees.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`NominationActionsParams`].
///
/// Converts [`NominationActionsParams`] into a query string suitable for the [`NominationActions`] endpoint.
impl ApiParams for NominationActionsParams {
    /// Converts the [`NominationActionsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for nomination actions.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`NominationCommitteesParams`].
///
/// Converts [`NominationCommitteesParams`] into a query string suitable for the [`NominationCommittees`] endpoint.
impl ApiParams for NominationCommitteesParams {
    /// Converts the [`NominationCommitteesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for nomination committees.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`NominationHearingsParams`].
///
/// Converts [`NominationHearingsParams`] into a query string suitable for the [`NominationHearings`] endpoint.
impl ApiParams for NominationHearingsParams {
    /// Converts the [`NominationHearingsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for nomination hearings.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Treaty Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`TreatyListParams`].
///
/// Converts [`TreatyListParams`] into a query string suitable for the [`TreatyList`] endpoint.
impl ApiParams for TreatyListParams {
    /// Converts the [`TreatyListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing treaties.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        query_params.join("&");
        query_params.insert(0, "?".to_string());
        query_params.join("")
    }
}

/// Implementation of [`ApiParams`] for [`TreatyByCongressParams`].
///
/// Converts [`TreatyByCongressParams`] into a query string suitable for the [`TreatyByCongress`] endpoint.
impl ApiParams for TreatyByCongressParams {
    /// Converts the [`TreatyByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving treaties by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(sort) = &self.sort {
            query_params.push(sort.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`TreatyDetailsParams`].
///
/// Converts [`TreatyDetailsParams`] into a query string suitable for the [`TreatyDetails`] endpoint.
impl ApiParams for TreatyDetailsParams {
    /// Converts the [`TreatyDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for treaty details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format.to_query_param()
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`TreatyPartitionedParams`].
///
/// Converts [`TreatyPartitionedParams`] into a query string suitable for the [`TreatyPartitioned`] endpoint.
impl ApiParams for TreatyPartitionedParams {
    /// Converts the [`TreatyPartitionedParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for partitioned treaties.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`TreatyCommitteesParams`].
///
/// Converts [`TreatyCommitteesParams`] into a query string suitable for the [`TreatyCommittees`] endpoint.
impl ApiParams for TreatyCommitteesParams {
    /// Converts the [`TreatyCommitteesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for treaty committees.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`TreatyActionsParams`].
///
/// Converts [`TreatyActionsParams`] into a query string suitable for the [`TreatyActions`] endpoint.
impl ApiParams for TreatyActionsParams {
    /// Converts the [`TreatyActionsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for treaty actions.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = &self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = &self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`SummariesListParams`].
///
/// Converts [`SummariesListParams`] into a query string suitable for the [`SummariesList`] endpoint.
impl ApiParams for SummariesListParams {
    /// Converts the [`SummariesListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for summaries lists.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`SummariesByCongressParams`].
///
/// Converts [`SummariesByCongressParams`] into a query string suitable for the [`SummariesByCongress`]
impl ApiParams for SummariesByCongressParams {
    /// Converts the [`SummariesByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for summaries by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`SummariesByCongressChamberParams`].
///
/// Converts [`SummariesByCongressChamberParams`] into a query string suitable for the
impl ApiParams for SummariesByTypeParams {
    /// Converts the [`SummariesByCongressChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for summaries by congress and chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CongressListParams`].
///
/// Converts [`CongressListParams`] into a query string suitable for the [`CongressList`] endpoint.
impl ApiParams for CongressListParams {
    /// Converts the [`CongressListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing congresses.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CongressDetailsParams`].
///
/// Converts [`CongressDetailsParams`] into a query string suitable for the [`CongressDetails`]
/// endpoint.
impl ApiParams for CongressDetailsParams {
    /// Converts the [`CongressDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for congress details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`CongressCurrentParams`].
///
/// Converts [`CongressCurrentParams`] into a query string suitable for the [`CongressCurrent`]
/// endpoint.
impl ApiParams for CongressCurrentParams {
    /// Converts the [`CongressCurrentParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for the current congress.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`BillRelatedParams`].
///
/// Converts [`BillRelatedParams`] into a query string suitable for the [`BillRelated`] endpoint.
impl ApiParams for BillRelatedParams {
    /// Converts the [`BillRelatedParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill related bills.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillSubjectsParams`].
///
/// Converts [`BillSubjectsParams`] into a query string suitable for the [`BillSubjects`] endpoint.
impl ApiParams for BillSubjectsParams {
    /// Converts the [`BillSubjectsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill subjects.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillSummariesParams`].
///
/// Converts [`BillSummariesParams`] into a query string suitable for the [`BillSummaries`] endpoint.
impl ApiParams for BillSummariesParams {
    /// Converts the [`BillSummariesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill summaries.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`BillTextParams`].
///
/// Converts [`BillTextParams`] into a query string suitable for the [`BillText`] endpoint.
impl ApiParams for BillTextParams {
    /// Converts the [`BillTextParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill text.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`BillTitlesParams`].
///
/// Converts [`BillTitlesParams`] into a query string suitable for the [`BillTitles`] endpoint.
impl ApiParams for BillTitlesParams {
    /// Converts the [`BillTitlesParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bill titles.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Member Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`MemberByStateDistrictParams`].
///
/// Converts [`MemberByStateDistrictParams`] into a query string suitable for the [`MemberByStateDistrict`] endpoint.
impl ApiParams for MemberByStateDistrictParams {
    /// Converts the [`MemberByStateDistrictParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for retrieving members by state and district.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(current_member) = self.current_member {
            query_params.push(format!("currentMember={}", current_member));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Committee Report Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CommitteeReportListParams`].
///
/// Converts [`CommitteeReportListParams`] into a query string suitable for the [`CommitteeReportList`] endpoint.
impl ApiParams for CommitteeReportListParams {
    /// Converts the [`CommitteeReportListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing committee reports.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(conference) = self.conference {
            query_params.push(format!("conference={}", conference));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeReportByCongressParams`].
///
/// Converts [`CommitteeReportByCongressParams`] into a query string suitable for the [`CommitteeReportByCongress`] endpoint.
impl ApiParams for CommitteeReportByCongressParams {
    /// Converts the [`CommitteeReportByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee reports by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(conference) = self.conference {
            query_params.push(format!("conference={}", conference));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeReportByTypeParams`].
///
/// Converts [`CommitteeReportByTypeParams`] into a query string suitable for the [`CommitteeReportByType`] endpoint.
impl ApiParams for CommitteeReportByTypeParams {
    /// Converts the [`CommitteeReportByTypeParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee reports by type.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(conference) = self.conference {
            query_params.push(format!("conference={}", conference));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeReportDetailsParams`].
///
/// Converts [`CommitteeReportDetailsParams`] into a query string suitable for the [`CommitteeReportDetails`] endpoint.
impl ApiParams for CommitteeReportDetailsParams {
    /// Converts the [`CommitteeReportDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for committee report details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeReportTextParams`].
///
/// Converts [`CommitteeReportTextParams`] into a query string suitable for the [`CommitteeReportText`] endpoint.
impl ApiParams for CommitteeReportTextParams {
    /// Converts the [`CommitteeReportTextParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee report text.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Committee Print Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CommitteePrintListParams`].
///
/// Converts [`CommitteePrintListParams`] into a query string suitable for the [`CommitteePrintList`] endpoint.
impl ApiParams for CommitteePrintListParams {
    /// Converts the [`CommitteePrintListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing committee prints.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteePrintByCongressParams`].
///
/// Converts [`CommitteePrintByCongressParams`] into a query string suitable for the [`CommitteePrintByCongress`] endpoint.
impl ApiParams for CommitteePrintByCongressParams {
    /// Converts the [`CommitteePrintByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee prints by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteePrintByCongressChamberParams`].
///
/// Converts [`CommitteePrintByCongressChamberParams`] into a query string suitable for the [`CommitteePrintByCongressChamber`] endpoint.
impl ApiParams for CommitteePrintByCongressChamberParams {
    /// Converts the [`CommitteePrintByCongressChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee prints by congress and chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteePrintByJacketNumberParams`].
///
/// Converts [`CommitteePrintByJacketNumberParams`] into a query string suitable for the [`CommitteePrintByJacketNumber`] endpoint.
impl ApiParams for CommitteePrintByJacketNumberParams {
    /// Converts the [`CommitteePrintByJacketNumberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for committee print by jacket number.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?format={}", format.to_string())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`CommitteePrintTextParams`].
///
/// Converts [`CommitteePrintTextParams`] into a query string suitable for the [`CommitteePrintText`] endpoint.
impl ApiParams for CommitteePrintDetailsParams {
    /// Converts the [`CommitteePrintTextParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee print text.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Committee Meeting Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CommitteeMeetingListParams`].
///
/// Converts [`CommitteeMeetingListParams`] into a query string suitable for the [`CommitteeMeetingList`] endpoint.
impl ApiParams for CommitteeMeetingListParams {
    /// Converts the [`CommitteeMeetingListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing committee meetings.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeMeetingByCongressParams`].
///
/// Converts [`CommitteeMeetingByCongressParams`] into a query string suitable for the [`CommitteeMeetingByCongress`] endpoint.
impl ApiParams for CommitteeMeetingByCongressParams {
    /// Converts the [`CommitteeMeetingByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee meetings by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeMeetingByChamberParams`].
///
/// Converts [`CommitteeMeetingByChamberParams`] into a query string suitable for the [`CommitteeMeetingByChamber`] endpoint.
impl ApiParams for CommitteeMeetingByChamberParams {
    /// Converts the [`CommitteeMeetingByChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for committee meetings by chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommitteeMeetingByEventParams`].
///
/// Converts [`CommitteeMeetingByEventParams`] into a query string suitable for the [`CommitteeMeetingByEvent`] endpoint.
impl ApiParams for CommitteeMeetingByEventParams {
    /// Converts the [`CommitteeMeetingByEventParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for committee meeting by event.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?format={}", format.to_string())
        } else {
            "".to_string()
        }
    }
}

// ================================
// Hearing Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`HearingListParams`].
///
/// Converts [`HearingListParams`] into a query string suitable for the [`HearingList`] endpoint.
impl ApiParams for HearingListParams {
    /// Converts the [`HearingListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing hearings.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`HearingByCongressParams`].
///
/// Converts [`HearingByCongressParams`] into a query string suitable for the [`HearingByCongress`] endpoint.
impl ApiParams for HearingByCongressParams {
    /// Converts the [`HearingByCongressParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for hearings by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`HearingByChamberParams`].
///
/// Converts [`HearingByChamberParams`] into a query string suitable for the [`HearingByChamber`] endpoint.
impl ApiParams for HearingByChamberParams {
    /// Converts the [`HearingByChamberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for hearings by chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`HearingByJacketNumberParams`].
///
/// Converts [`HearingByJacketNumberParams`] into a query string suitable for the [`HearingByJacketNumber`] endpoint.
impl ApiParams for HearingByJacketNumberParams {
    /// Converts the [`HearingByJacketNumberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for hearing by jacket number.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?format={}", format.to_string())
        } else {
            "".to_string()
        }
    }
}

// ================================
// Congressional Record Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CongressionalRecordListParams`].
///
/// Converts [`CongressionalRecordListParams`] into a query string suitable for the [`CongressionalRecordList`] endpoint.
impl ApiParams for CongressionalRecordListParams {
    /// Converts the [`CongressionalRecordListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing congressional records.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(year) = self.year {
            query_params.push(format!("year={}", year));
        }

        if let Some(month) = self.month {
            query_params.push(format!("month={}", month));
        }

        if let Some(day) = self.day {
            query_params.push(format!("day={}", day));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Daily Congressional Record Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`DailyCongressionalRecordListParams`].
///
/// Converts [`DailyCongressionalRecordListParams`] into a query string suitable for the [`DailyCongressionalRecordList`] endpoint.
impl ApiParams for DailyCongressionalRecordListParams {
    /// Converts the [`DailyCongressionalRecordListParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for listing daily congressional records.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`DailyCongressionalVolumeNumberParams`].
///
/// Converts [`DailyCongressionalVolumeNumberParams`] into a query string suitable for the [`DailyCongressionalRecordVolume`] endpoint.
impl ApiParams for DailyCongressionalVolumeNumberParams {
    /// Converts the [`DailyCongressionalVolumeNumberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for daily congressional records by volume number.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`DailyCongressionalVolumeNumberIssueNumberParams`].
///
/// Converts [`DailyCongressionalVolumeNumberIssueNumberParams`] into a query string suitable for the [`DailyCongressionalRecordVolumeIssue`] and [`DailyCongressionalRecordArticles`] endpoints.
impl ApiParams for DailyCongressionalVolumeNumberIssueNumberParams {
    /// Converts the [`DailyCongressionalVolumeNumberIssueNumberParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for daily congressional records by volume and issue number.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Bound Congressional Record Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`BoundCongressionalRecordParams`].
///
/// Converts [`BoundCongressionalRecordParams`] into a query string suitable for the [`BoundCongressionalRecord`] endpoints.
impl ApiParams for BoundCongressionalRecordParams {
    /// Converts the [`BoundCongressionalRecordParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for bound congressional records.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

// ================================
// Communication Params to Query String
// ================================

/// Implementation of [`ApiParams`] for [`CommunicationParams`].
///
/// Converts [`CommunicationParams`] into a query string suitable for the House and Senate Communication endpoints.
impl ApiParams for CommunicationParams {
    /// Converts the [`CommunicationParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for communications.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format!("format={}", format.to_string()));
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`CommunicationDetailsParams`].
///
/// Converts [`CommunicationDetailsParams`] into a query string suitable for the House and Senate
/// Communication Details endpoints.
impl ApiParams for CommunicationDetailsParams {
    /// Converts the [`CommunicationDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for communication details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?format={}", format.to_string())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of [`ApiParams`] for [`RequirementParams`].
///
/// Converts [`RequirementParams`] into a query string suitable for the House Requirement
/// endpoints.
impl ApiParams for RequirementParams {
    /// Converts the [`RequirementParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for requirements.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of [`ApiParams`] for [`RequirementDetailsParams`].
///
/// Converts [`RequirementDetailsParams`] into a query string suitable for the House
/// Requirement Details endpoints.
impl ApiParams for RequirementDetailsParams {
    /// Converts the [`RequirementDetailsParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameter for requirement details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?format={}", format.to_string())
        } else {
            "".to_string()
        }
    }
}

impl ApiParams for GenericParams {
    /// Converts the [`GenericParams`] into a query string.
    ///
    /// # Returns
    ///
    /// A [`String`] containing the query parameters for generic endpoints.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        if let Some(offset) = self.offset {
            query_params.push(format!("offset={}", offset));
        }

        if let Some(limit) = self.limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(from_date_time) = &self.from_date_time {
            query_params.push(format!("fromDateTime={}", from_date_time));
        }

        if let Some(to_date_time) = &self.to_date_time {
            query_params.push(format!("toDateTime={}", to_date_time));
        }

        if let Some(conference) = self.conference {
            query_params.push(format!("conference={}", conference));
        }

        if let Some(year) = self.year {
            query_params.push(format!("year={}", year));
        }

        if let Some(month) = self.month {
            query_params.push(format!("month={}", month));
        }

        if let Some(day) = self.day {
            query_params.push(format!("day={}", day));
        }

        if let Some(sort) = &self.sort {
            query_params.push(format!("sort={}", sort.to_string()));
        }

        "?".to_string() + &query_params.join("&")
    }
}
