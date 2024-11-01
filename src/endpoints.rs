//! # [`endpoints`] Module
//! 
//! This module defines the [`Endpoints`] enum, which represents the various API endpoints
//! available in the US Congress API. Each variant encapsulates the necessary parameters
//! required to interact with a specific endpoint.
//! 
//! Additionally, the [`NewEndpoint`] trait provides constructor methods for creating
//! instances of each [`Endpoints`] variant in a standardized manner.
//! 
//! ## Example
//! 
//! ```rust
//! use cdg_api::endpoints::{Endpoints, NewEndpoint};
//! use cdg_api::cdg_types::FormatType;
//! use cdg_api::param_models::*;
//! 
//! fn main() {
//!     let params = BillListParams {
//!         format: Some(FormatType::Json),
//!         limit: Some(20),
//!         ..BillListParams::default()
//!     };
//!     let endpoint = Endpoints::new_bill_list(params);
//!     // Use the endpoint with CongressApiClient
//! }
//! ```

use serde::{Deserialize, Serialize};
use crate::{param_models::*, cdg_types::*};

/// Each variant of the [`Endpoints`] enum corresponds to a specific API endpoint,
/// encapsulating the necessary parameters required to interact with that endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Endpoints {
    /// Endpoint for manual API requests, where the user provides the entire endpoint.
    /// The base URL, and the API key are automatically appended to the provided endpoint.
    Generic(String, GenericParams),

    // ================================
    // Bill Endpoints
    // ================================

    /// Endpoint to list bills based on provided parameters.
    /// /bill
    BillList(BillListParams),

    /// Endpoint to retrieve bills by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillByCongressParams`]: Additional parameters for filtering bills.
    /// /bill/{congress}
    BillByCongress(u32, BillByCongressParams),

    /// Endpoint to get bills filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill (e.g., House, Senate).
    /// - [`BillByTypeParams`]: Additional parameters for filtering bills by type.
    /// /bill/{congress}/{billType}
    BillByType(u32, BillType, BillByTypeParams),

    /// Endpoint to retrieve detailed information about a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillDetailsParams`]: Additional parameters for bill details.
    /// /bill/{congress}/{billType}/{billNumber}
    BillDetails(u32, BillType, u32, BillDetailsParams),

    /// Endpoint to fetch actions taken on a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillActionsParams`]: Additional parameters for bill actions.
    /// /bill/{congress}/{billType}/{billNumber}/actions
    BillActions(u32, BillType, u32, BillActionsParams),

    /// Endpoint to list amendments of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillAmendmentsParams`]: Additional parameters for bill amendments.
    /// /bill/{congress}/{billType}/{billNumber}/amendments
    BillAmendments(u32, BillType, u32, BillAmendmentsParams),

    /// Endpoint to get committees associated with a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillCommitteesParams`]: Additional parameters for bill committees.
    /// /bill/{congress}/{billType}/{billNumber}/committees
    BillCommittees(u32, BillType, u32, BillCommitteesParams),

    /// Endpoint to retrieve cosponsors of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillCosponsorsParams`]: Additional parameters for bill cosponsors.
    /// /bill/{congress}/{billType}/{billNumber}/cosponsors
    BillCosponsors(u32, BillType, u32, BillCosponsorsParams),

    /// Endpoint to get related bills of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillRelatedParams`]: Additional parameters for related bills.
    /// /bill/{congress}/{billType}/{billNumber}/relatedbills
    BillRelated(u32, BillType, u32, BillRelatedParams),

    /// Endpoint to retrieve subjects of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillSubjectsParams`]: Additional parameters for bill subjects.
    /// /bill/{congress}/{billType}/{billNumber}/subjects
    BillSubjects(u32, BillType, u32, BillSubjectsParams),

    /// Endpoint to retrieve summaries of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillSummariesParams`]: Additional parameters for bill summaries.
    /// /bill/{congress}/{billType}/{billNumber}/summaries
    BillSummaries(u32, BillType, u32, BillSummariesParams),

    /// Endpoint to retrieve text of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillTextParams`]: Additional parameters for bill text.
    /// /bill/{congress}/{billType}/{billNumber}/text
    BillText(u32, BillType, u32, BillTextParams),
    
    /// Endpoint to retrieve titles of a specific bill.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`u32`]: The bill number.
    /// - [`BillTitlesParams`]: Additional parameters for bill titles.
    /// /bill/{congress}/{billType}/{billNumber}/titles
    BillTitles(u32, BillType, u32, BillTitlesParams),

    // ================================
    // Law Endpoints
    // ================================

    /// Endpoint to list laws based on provided parameters.
    /// /law/{congress}/{lawType}
    LawByType(u32, LawType, LawParams),

    /// Endpoint to retrieve laws by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`LawParams`]: Additional parameters for filtering laws.
    /// /law/{congress}
    LawByCongress(u32, LawParams),

    /// Endpoint to get detailed information about a specific law.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`LawType`]: The type of law.
    /// - [`u32`]: The law number.
    /// - [`LawDetailsParams`]: Additional parameters for law details.
    /// /law/{congress}/{lawType}/{lawNumber}
    LawDetails(u32, LawType, u32, LawParams),

    // ================================
    // Amendment Endpoints
    // ================================

    /// Endpoint to list amendments based on provided parameters.
    /// /amendment
    AmendmentList(AmendmentListParams),

    /// Endpoint to retrieve amendments by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentByCongressParams`]: Additional parameters for filtering amendments.
    /// /amendment/{congress}
    AmendmentByCongress(u32, AmendmentByCongressParams),

    /// Endpoint to get amendments filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`AmendmentByTypeParams`]: Additional parameters for filtering amendments by type.
    /// /amendment/{congress}/{amendmentType}
    AmendmentByType(u32, AmendmentType, AmendmentByTypeParams),

    /// Endpoint to retrieve detailed information about a specific amendment.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`u32`]: The amendment number.
    /// - [`AmendmentDetailsParams`]: Additional parameters for amendment details.
    /// /amendment/{congress}/{amendmentType}/{amendmentNumber}
    AmendmentDetails(u32, AmendmentType, u32, AmendmentDetailsParams),

    /// Endpoint to fetch actions taken on a specific amendment.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`String`]: The amendment number.
    /// - [`AmendmentActionsParams`]: Additional parameters for amendment actions.
    /// /amendment/{congress}/{amendmentType}/{amendmentNumber}/actions
    AmendmentActions(u32, AmendmentType, String, AmendmentActionsParams),

    /// Endpoint to retrieve cosponsors of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`String`]: The amendment number.
    /// - [`AmendmentCosponsorsParams`]: Additional parameters for amendment cosponsors.
    /// /amendment/{congress}/{amendmentType}/{amendmentNumber}/cosponsors
    AmendmentCosponsors(u32, AmendmentType, String, AmendmentCosponsorsParams),

    /// Endpoint to list amendments of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`String`]: The amendment number.
    /// - [`AmendmentAmendmentsParams`]: Additional parameters for amendment amendments.
    /// /amendment/{congress}/{amendmentType}/{amendmentNumber}/amendments
    AmendmentAmendments(u32, AmendmentType, String, AmendmentAmendmentsParams),

    /// Endpoint to retrieve the text of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`AmendmentType`]: The type of amendment.
    /// - [`String`]: The amendment number.
    /// - [`AmendmentTextParams`]: Additional parameters for amendment text.
    /// /amendment/{congress}/{amendmentType}/{amendmentNumber}/text
    AmendmentText(u32, AmendmentType, String, AmendmentTextParams),

    // ================================
    // Summaries Endpoints
    // ================================

    /// Endpoint to list summaries based on provided parameters.
    /// /summary
    SummariesList(SummariesListParams),

    /// Endpoint to retrieve summaries by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`SummariesByCongressParams`]: Additional parameters for filtering summaries.
    /// /summary/{congress}
    SummariesByCongress(u32, SummariesByCongressParams),

    /// Endpoint to get summaries filtered by bill type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`BillType`]: The type of bill.
    /// - [`SummariesByTypeParams`]: Additional parameters for filtering summaries by type.
    /// /summary/{congress}/{billType}
    SummariesByType(u32, BillType, SummariesByTypeParams),

    // ================================
    // Congress Endpoints
    // ================================

    /// Endpoint to list congress sessions based on provided parameters.
    /// /congress
    CongressList(CongressListParams),

    /// Endpoint to retrieve detailed information about a specific congress session.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CongressDetailsParams`]: Additional parameters for congress details.
    /// /congress/{congress}
    CongressDetails(u32, CongressDetailsParams),

    /// Endpoint to get information about the current congress session.
    ///
    /// # Parameters
    ///
    /// - [`CongressCurrentParams`]: Additional parameters for current congress.
    /// /congress/current
    CongressCurrent(CongressCurrentParams),

    // ================================
    // Member Endpoints
    // ================================

    /// Endpoint to list members of Congress based on provided parameters.
    /// /member
    MemberList(MemberListParams),

    /// Endpoint to retrieve members by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`MemberByCongressParams`]: Additional parameters for filtering members by congress.
    /// /member/congress/{congress}
    MemberByCongress(u32, MemberByCongressParams),

    /// Endpoint to get members representing a specific state.
    ///
    /// # Parameters
    ///
    /// - [`String`]: The state code (e.g., "NY" for New York).
    /// - [`MemberByStateParams`]: Additional parameters for filtering members by state.
    /// /member/{stateCode}
    MemberByState(String, MemberByStateParams),

    /// Endpoint to retrieve members by state and district.
    ///
    /// # Parameters
    ///
    /// - [`String`]: The state code.
    /// - [`u32`]: The district number.
    /// - [`MemberByStateDistrictParams`]: Additional parameters for filtering members by state and
    /// district.
    /// /member/{stateCode}/{district}
    MemberByStateDistrict(String, u32, MemberByStateDistrictParams),

    /// Endpoint to retrieve members by congress, state, and district.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The state code.
    /// - [`u32`]: The district number.
    /// - [`MemberByCongressStateDistrictParams`]: Additional parameters for filtering.
    /// /member/congress/{congress}/{stateCode}/{district}
    MemberByCongressStateDistrict(u32, String, u32, MemberByCongressStateDistrictParams),

    /// Endpoint to get detailed information about a specific member.
    ///
    /// # Parameters
    ///
    /// - [`String`]: The bio guide ID of the member.
    /// - [`MemberDetailsParams`]: Additional parameters for member details.
    /// /member/{bioGuideId}
    MemberDetails(String, MemberDetailsParams),

    /// Endpoint to list sponsorships of a specific member.
    ///
    /// # Parameters
    ///
    /// - [`String`]: The bio guide ID of the member.
    /// - [`SponsorshipListParams`]: Additional parameters for sponsorships.
    /// /member/{bioGuideId}/sponsored-legislation
    SponsorshipList(String, SponsorshipListParams),

    /// Endpoint to list cosponsorships of a specific member.
    ///
    /// # Parameters
    ///
    /// - [`String`]: The bio guide ID of the member.
    /// - [`CosponsorshipListParams`]: Additional parameters for cosponsorships.
    /// /member/{bioGuideId}/cosponsored-legislation
    CosponsorshipList(String, CosponsorshipListParams),

    // ================================
    // Committee Endpoints
    // ================================

    /// Endpoint to list committees based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommitteeListParams`]: Additional parameters for listing committees.
    /// /committee
    CommitteeList(CommitteeListParams),

    /// Endpoint to retrieve committees by chamber (House or Senate).
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type (House or Senate).
    /// - [`CommitteeByChamberParams`]: Additional parameters for filtering committees by chamber.
    /// /committee/{chamber}
    CommitteeByChamber(ChamberType, CommitteeByChamberParams),

    /// Endpoint to get committees associated with a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeByCongressParams`]: Additional parameters for filtering committees by congress.
    /// /committee/{congress}
    CommitteeByCongress(u32, CommitteeByCongressParams),

    /// Endpoint to retrieve committees by congress and chamber.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`CommitteeByCongressChamberParams`]: Additional parameters for filtering.
    /// /committee/{congress}/{chamber}
    CommitteeByCongressChamber(u32, ChamberType, CommitteeByCongressChamberParams),

    /// Endpoint to get detailed information about a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeDetailsParams`]: Additional parameters for committee details.
    /// /committee/{chamber}/{committeeCode}
    CommitteeDetails(ChamberType, String, CommitteeDetailsParams),

    /// Endpoint to list bills under a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeBillsParams`]: Additional parameters for committee bills.
    /// /committee/{chamber}/{committeeCode}/bills
    CommitteeBills(ChamberType, String, CommitteeBillsParams),

    /// Endpoint to retrieve reports from a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeReportsParams`]: Additional parameters for committee reports.
    /// /committee/{chamber}/{committeeCode}/reports
    CommitteeReports(ChamberType, String, CommitteeReportsParams),

    /// Endpoint to list nominations handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeNominationsParams`]: Additional parameters for committee nominations.
    /// /committee/{chamber}/{committeeCode}/nominations
    CommitteeNominations(ChamberType, String, CommitteeNominationsParams),

    /// Endpoint to retrieve house communications handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeHouseCommunicationParams`]: Additional parameters for house communications.
    /// /committee/{chamber}/{committeeCode}/house-communication
    CommitteeHouseCommunication(ChamberType, String, CommitteeHouseCommunicationParams),

    /// Endpoint to retrieve senate communications handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The committee code.
    /// - [`CommitteeSenateCommunicationParams`]: Additional parameters for senate communications.
    /// /committee/{chamber}/{committeeCode}/senate-communication
    CommitteeSenateCommunication(ChamberType, String, CommitteeSenateCommunicationParams),

    // ================================
    // Committee Report Endpoints
    // ================================
    
    /// Endpoint to list committee reports based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommitteeReportListParams`]: Additional parameters for listing committee reports.
    /// /committee-report
    CommitteeReportList(CommitteeReportListParams),

    /// Endpoint to retrieve committee reports by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeReportByCongressParams`]: Additional parameters for filtering committee reports.
    /// /committee-report/{congress}
    CommitteeReportByCongress(u32, CommitteeReportByCongressParams),

    /// Endpoint to get committee reports filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeReportType`]: The type of committee report.
    /// - [`CommitteeReportByTypeParams`]: Additional parameters for filtering committee reports by
    /// type.
    /// /committee-report/{congress}/{reportType}
    CommitteeReportByType(u32, CommitteeReportType, CommitteeReportByTypeParams),

    /// Endpoint to retrieve detailed information about a specific committee report.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeReportType`]: The type of committee report.
    /// - [`u32`]: The report number.
    /// - [`CommitteeReportDetailsParams`]: Additional parameters for committee report details.
    /// /committee-report/{congress}/{reportType}/{reportNumber}
    CommitteeReportDetails(u32, CommitteeReportType, u32, CommitteeReportDetailsParams),

    /// Endpoint to retrieve text of a specific committee report.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeReportType`]: The type of committee report.
    /// - [`u32`]: The report number.
    /// - [`CommitteeReportTextParams`]: Additional parameters for committee report text.
    /// /committee-report/{congress}/{reportType}/{reportNumber}/text
    CommitteeReportText(u32, CommitteeReportType, u32, CommitteeReportTextParams),

    // ================================
    // Committee Print Endpoints
    // ================================
    
    /// Endpoint to list committee prints based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommitteePrintListParams`]: Additional parameters for listing committee prints.
    /// /committee-print
    CommitteePrintList(CommitteePrintListParams),

    /// Endpoint to retrieve committee prints by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteePrintByCongressParams`]: Additional parameters for filtering committee prints.
    /// /committee-print/{congress}
    CommitteePrintByCongress(u32, CommitteePrintByCongressParams),

    /// Endpoint to get committee prints filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type. (house, senate, nochamber)
    /// - [`CommitteePrintByCongressChamberParams`]: Additional parameters for filtering committee
    /// prints by chamber.
    /// /committee-print/{congress}/{chamber}
    CommitteePrintByCongressChamber(u32, ChamberType, CommitteePrintByCongressChamberParams),

    /// Endpoint to retrieve detailed information about a specific committee print.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The jacket number for the committee print.
    CommitteePrintByJacketNumber(u32, u32, CommitteePrintByJacketNumberParams),

    /// Endpoint to retrieve text of a specific committee print.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`u32`]: The jacket number for the committee print.
    /// - [`CommitteePrintTextParams`]: Additional parameters for committee print text.
    CommitteePrintText(u32, String, u32, CommitteePrintDetailsParams),

    // ================================
    // Committee Meeting Endpoints
    // ================================
    
    /// Endpoint to list committee meetings based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommitteeMeetingListParams`]: Additional parameters for listing committee meetings.
    /// /committee-meeting
    CommitteeMeetingList(CommitteeMeetingListParams),

    /// Endpoint to retrieve committee meetings by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommitteeMeetingByCongressParams`]: Additional parameters for filtering committee
    /// meetings.
    /// /committee-meeting/{congress}
    CommitteeMeetingByCongress(u32, CommitteeMeetingByCongressParams),

    /// Endpoint to get committee meetings filtered by chamber within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`CommitteeMeetingByChamberParams`]: Additional parameters for filtering committee meetings
    /// by chamber.
    /// /committee-meeting/{congress}/{chamber}
    CommitteeMeetingByChamber(u32, ChamberType, CommitteeMeetingByChamberParams),

    /// Endpoint to retrieve detailed information about a specific committee meeting.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`String`]: The eventId for the committee meeting.
    /// - [`CommitteeMeetingDetailsParams`]: Additional parameters for committee meeting details.
    /// /committee-meeting/{congress}/{chamber}/{eventId}
    CommitteeMeetingByEvent(u32, ChamberType, String, CommitteeMeetingByEventParams),

    // ================================
    // Hearing Endpoints
    // ================================
    
    /// Endpoint to list hearings based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`HearingListParams`]: Additional parameters for listing hearings.
    /// /hearing
    HearingList(HearingListParams),

    /// Endpoint to retrieve hearings by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`HearingByCongressParams`]: Additional parameters for filtering hearings.
    /// /hearing/{congress}
    HearingByCongress(u32, HearingByCongressParams),

    /// Endpoint to get hearings filtered by chamber within a specific congress.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`HearingByChamberParams`]: Additional parameters for filtering hearings by chamber.
    /// /hearing/{congress}/{chamber}
    HearingByChamber(u32, ChamberType, HearingByChamberParams),

    /// Endpoint to retrieve detailed information about a specific hearing.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`ChamberType`]: The chamber type.
    /// - [`u32`]: The jacket number for the hearing.
    /// - [`HearingDetailsParams`]: Additional parameters for hearing details.
    /// /hearing/{congress}/{jacketNumber}
    HearingByJacketNumber(u32, ChamberType, u32, HearingByJacketNumberParams),

    // ================================
    // Congressional Record Endpoints
    // ================================
    
    /// Endpoint to list Congressional Records based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CongressionalRecordListParams`]: Additional parameters for listing Congressional Records.
    /// /congressional-record
    CongressionalRecordList(CongressionalRecordListParams),

    // ====================================
    // Daily Congressional Record Endpoints
    // ====================================
    
    /// Endpoint to list daily Congressional Records based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`DailyCongressionalRecordListParams`]: Additional parameters for listing daily
    /// /daily-congressional-record
    DailyCongressionalRecordList(DailyCongressionalRecordListParams),

    /// Endpoint to retrieve daily Congressional Records by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`DailyCongressionalRecordByCongressParams`]: Additional parameters for filtering daily
    /// /daily-congressional-record/{volumeNumber}
    DailyCongressionalRecordVolume(u32, DailyCongressionalVolumeNumberParams),

    /// Endpoint to get daily Congressional Records by volume and issue number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The volume number.
    /// - [`DailyCongressionalVolumeNumberIssueNumberParams`]: Additional parameters for filtering
    /// daily Congressional Records by volume and issue number.
    /// /daily-congressional-record/{volumeNumber}/{issueNumber}
    DailyCongressionalRecordVolumeIssue(u32, u32, DailyCongressionalVolumeNumberIssueNumberParams),

    /// Endpoint to get daily Congressional Records by volume, issue number, and article number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The volume number.
    /// - [`DailyCongressionalVolumeNumberIssueNumberParams`]: Additional parameters for filtering
    /// daily Congressional Records by volume, issue number, and article number.
    /// /daily-congressional-record/{volumeNumber}/{issueNumber}/article
    DailyCongressionalRecordArticles(u32, u32, DailyCongressionalVolumeNumberIssueNumberParams),

    // ================================
    // Bound Congressional Record Endpoints
    // ================================

    /// Endpoint to list bound Congressional Records based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`BoundCongressionalRecordListParams`]: Additional parameters for listing bound
    /// /bound-congressional-record
    BoundCongressionalRecordList(BoundCongressionalRecordParams),

    /// Endpoint to retrieve bound Congressional Records by a specific year.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The year.
    /// - [`BoundCongressionalRecordByYearParams`]: Additional parameters for filtering bound
    /// /bound-congressional-record/{year}
    BoundCongressionalRecordByYear(u32, BoundCongressionalRecordParams),

    /// Endpoint to get bound Congressional Records by year and month.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The year.
    /// - [`u32`]: The month.
    /// - [`BoundCongressionalRecordByYearMonthParams`]: Additional parameters for filtering bound
    /// /bound-congressional-record/{year}/{month}
    BoundCongressionalRecordByYearMonth(u32, u32, BoundCongressionalRecordParams),

    /// Endpoint to get bound Congressional Records by year, month, and day.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The year.
    /// - [`u32`]: The month.
    /// - [`u32`]: The day.
    /// - [`BoundCongressionalRecordByYearMonthDayParams`]: Additional parameters for filtering bound
    /// /bound-congressional-record/{year}/{month}/{day}
    BoundCongressionalRecordByYearMonthDay(u32, u32, u32, BoundCongressionalRecordParams),

    // ================================
    // House Communication Endpoints
    // ================================

    /// Endpoint to list house communications based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommunicationParams`]: Additional parameters for listing house communications.
    /// /house-communication
    HouseCommunicationList(CommunicationParams),

    /// Endpoint to retrieve house communications by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationParams`]: Additional parameters for filtering house
    /// /house-communication/{congress}
    HouseCommunicationByCongress(u32, CommunicationParams),

    /// Endpoint to get house communications filtered by congress and a specific communication
    /// type.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationType`]: The type of house communication.
    /// - [`CommunicationParams`]: Additional parameters for filtering house
    /// communications by type.
    /// /house-communication/{congress}/{communicationType}
    HouseCommunicationByType(u32, CommunicationType, CommunicationParams),

    /// Endpoint to retrieve detailed information about a specific house communication.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationType`]: The type of house communication.
    /// - [`u32`]: The communication number.
    /// - [`CommunicationDetailsParams`]: Additional parameters for house communication details.
    /// /house-communication/{congress}/{communicationType}/{communicationNumber}
    HouseCommunicationDetails(u32, CommunicationType, u32, CommunicationDetailsParams),

    // ================================
    // House Requirement Endpoints
    // ================================

    /// Endpoint to list house requirements based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`RequirementParams`]: Additional parameters for listing house requirements.
    /// /house-requirement
    HouseRequirementList(RequirementParams),

    /// Endpoint to retrieve house requirements by a specific requirement number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The requirement number.
    /// - [`RequirementDetailsParams`]: Additional parameters for filtering house requirements.
    /// /house-requirement/{requirementNumber}
    HouseRequirementDetails(u32, RequirementDetailsParams),

    /// Endpoint to get matching house requirements based on a specific requirement number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The requirement number.
    /// - [`RequirementParams`]: Additional parameters for matching house requirements.
    /// /house-requirement/{requirementNumber}/matching-communications
    HouseRequirementMatching(u32, RequirementParams),

    // ================================
    // Senate Communication Endpoints
    // ================================

    /// Endpoint to list house communications based on provided parameters.
    ///
    /// # Parameters
    ///
    /// - [`CommunicationParams`]: Additional parameters for listing house communications.
    /// /house-communication
    SenateCommunicationList(CommunicationParams),

    /// Endpoint to retrieve house communications by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationParams`]: Additional parameters for filtering house
    /// /house-communication/{congress}
    SenateCommunicationByCongress(u32, CommunicationParams),

    /// Endpoint to get house communications filtered by congress and a specific communication
    /// type.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationType`]: The type of house communication.
    /// - [`CommunicationParams`]: Additional parameters for filtering house
    /// communications by type.
    /// /house-communication/{congress}/{communicationType}
    SenateCommunicationByType(u32, CommunicationType, CommunicationParams),

    /// Endpoint to retrieve detailed information about a specific house communication.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`CommunicationType`]: The type of house communication.
    /// - [`u32`]: The communication number.
    /// - [`CommunicationDetailsParams`]: Additional parameters for house communication details.
    /// /house-communication/{congress}/{communicationType}/{communicationNumber}
    SenateCommunicationDetails(u32, CommunicationType, u32, CommunicationDetailsParams),

    // ================================
    // Nomination Endpoints
    // ================================

    /// Endpoint to list nominations based on provided parameters.
    NominationList(NominationListParams),

    /// Endpoint to retrieve nominations by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`NominationByCongressParams`]: Additional parameters for filtering nominations.
    NominationByCongress(u32, NominationByCongressParams),

    /// Endpoint to get detailed information about a specific nomination.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The nomination number.
    /// - [`NominationDetailsParams`]: Additional parameters for nomination details.
    NominationDetails(u32, String, NominationDetailsParams),

    /// Endpoint to list nominees of a specific nomination.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The nomination number.
    /// - [`u32`]: The ordinal number.
    /// - [`NomineesParams`]: Additional parameters for nominees.
    Nominees(u32, String, u32, NomineesParams),

    /// Endpoint to fetch actions taken on a specific nomination.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The nomination number.
    /// - [`NominationActionsParams`]: Additional parameters for nomination actions.
    NominationActions(u32, String, NominationActionsParams),

    /// Endpoint to retrieve committees involved in a specific nomination.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The nomination number.
    /// - [`NominationCommitteesParams`]: Additional parameters for nomination committees.
    NominationCommittees(u32, String, NominationCommitteesParams),

    /// Endpoint to list hearings related to a specific nomination.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`String`]: The nomination number.
    /// - [`NominationHearingsParams`]: Additional parameters for nomination hearings.
    NominationHearings(u32, String, NominationHearingsParams),

    // ================================
    // Treaty Endpoints
    // ================================

    /// Endpoint to list treaties based on provided parameters.
    TreatyList(TreatyListParams),

    /// Endpoint to retrieve treaties by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`TreatyByCongressParams`]: Additional parameters for filtering treaties.
    TreatyByCongress(u32, TreatyByCongressParams),

    /// Endpoint to get detailed information about a specific treaty.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The treaty number.
    /// - [`TreatyDetailsParams`]: Additional parameters for treaty details.
    TreatyDetails(u32, u32, TreatyDetailsParams),

    /// Endpoint to retrieve partitioned information about a specific treaty.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The treaty number.
    /// - [`String`]: The treaty suffix.
    /// - [`TreatyPartitionedParams`]: Additional parameters for partitioned treaties.
    TreatyPartitioned(u32, u32, String, TreatyPartitionedParams),

    /// Endpoint to retrieve committees associated with a specific treaty.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The treaty number.
    /// - [`TreatyCommitteesParams`]: Additional parameters for treaty committees.
    TreatyCommittees(u32, u32, TreatyCommitteesParams),

    /// Endpoint to fetch actions taken on a specific treaty.
    ///
    /// # Parameters
    ///
    /// - [`u32`]: The congress number.
    /// - [`u32`]: The treaty number.
    /// - [`TreatyActionsParams`]: Additional parameters for treaty actions.
    TreatyActions(u32, u32, TreatyActionsParams),
}

/// Trait defining constructors for creating new instances of [`Endpoints`].
///
/// This trait provides a standardized way to instantiate each variant of the
/// [`Endpoints`] enum by supplying the necessary parameters.
pub trait NewEndpoint {
    fn new_generic(endpoint: String, params: GenericParams) -> Self;

    // ================================
    // Bill Constructors
    // ================================

    /// Constructs a [`BillList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing bills.
    fn new_bill_list(params: BillListParams) -> Self;

    /// Constructs a [`BillByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering bills by congress.
    fn new_bill_by_congress(congress: u32, params: BillByCongressParams) -> Self;

    /// Constructs a [`BillByType`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`params`]: Parameters for filtering bills by type.
    fn new_bill_by_type(congress: u32, bill_type: BillType, params: BillByTypeParams) -> Self;

    /// Constructs a [`BillDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`bill_number`]: The bill number.
    /// - [`params`]: Parameters for bill details.
    fn new_bill_details(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillDetailsParams,
    ) -> Self;

    /// Constructs a [`BillActions`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`bill_number`]: The bill number.
    /// - [`params`]: Parameters for bill actions.
    fn new_bill_actions(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillActionsParams,
    ) -> Self;

    /// Constructs a [`BillAmendments`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`bill_number`]: The bill number.
    /// - [`params`]: Parameters for bill amendments.
    fn new_bill_amendments(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillAmendmentsParams,
    ) -> Self;

    /// Constructs a [`BillCommittees`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`bill_number`]: The bill number.
    /// - [`params`]: Parameters for bill committees.
    fn new_bill_committees(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillCommitteesParams,
    ) -> Self;

    /// Constructs a [`BillCosponsors`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`bill_number`]: The bill number.
    /// - [`params`]: Parameters for bill cosponsors.
    fn new_bill_cosponsors(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillCosponsorsParams,
    ) -> Self;

    // ================================
    // Law Constructors
    // ================================

    /// Constructs a [`LawList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing laws.
    fn new_law_type(congress: u32, law_type: LawType, params: LawParams) -> Self;

    /// Constructs a [`LawByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering laws by congress.
    fn new_law_by_congress(congress: u32, params: LawParams) -> Self;

    /// Constructs a [`LawDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for law details.
    fn new_law_details(congress: u32, law_type: LawType, law_number: u32, params: LawParams) -> Self;

    // ================================
    // Amendment Constructors
    // ================================

    /// Constructs an [`AmendmentList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing amendments.
    fn new_amendment_list(params: AmendmentListParams) -> Self;

    /// Constructs an [`AmendmentByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering amendments by congress.
    fn new_amendment_by_congress(congress: u32, params: AmendmentByCongressParams) -> Self;

    /// Constructs an [`AmendmentByType`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`params`]: Parameters for filtering amendments by type.
    fn new_amendment_by_type(
        congress: u32,
        amendment_type: AmendmentType,
        params: AmendmentByTypeParams,
    ) -> Self;

    /// Constructs an [`AmendmentDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`amendment_number`]: The amendment number.
    /// - [`params`]: Parameters for amendment details.
    fn new_amendment_details(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: u32,
        params: AmendmentDetailsParams,
    ) -> Self;

    /// Constructs an [`AmendmentActions`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`amendment_number`]: The amendment number.
    /// - [`params`]: Parameters for amendment actions.
    fn new_amendment_actions(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentActionsParams,
    ) -> Self;

    /// Constructs an [`AmendmentCosponsors`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`amendment_number`]: The amendment number.
    /// - [`params`]: Parameters for amendment cosponsors.
    fn new_amendment_cosponsors(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentCosponsorsParams,
    ) -> Self;

    /// Constructs an [`AmendmentAmendments`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`amendment_number`]: The amendment number.
    /// - [`params`]: Parameters for amendment amendments.
    fn new_amendment_amendments(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentAmendmentsParams,
    ) -> Self;

    /// Constructs an [`AmendmentText`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`amendment_type`]: The type of amendment.
    /// - [`amendment_number`]: The amendment number.
    /// - [`params`]: Parameters for amendment text.
    fn new_amendment_text(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentTextParams,
    ) -> Self;

    // ================================
    // Summaries Constructors
    // ================================

    /// Constructs a [`SummariesList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing summaries.
    fn new_summaries_list(params: SummariesListParams) -> Self;

    /// Constructs a [`SummariesByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering summaries by congress.
    fn new_summaries_by_congress(congress: u32, params: SummariesByCongressParams) -> Self;

    /// Constructs a [`SummariesByType`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`bill_type`]: The type of bill.
    /// - [`params`]: Parameters for filtering summaries by type.
    fn new_summaries_by_type(
        congress: u32,
        bill_type: BillType,
        params: SummariesByTypeParams,
    ) -> Self;

    // ================================
    // Congress Constructors
    // ================================

    /// Constructs a [`CongressList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing congress sessions.
    fn new_congress_list(params: CongressListParams) -> Self;

    /// Constructs a [`CongressDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for congress details.
    fn new_congress_details(congress: u32, params: CongressDetailsParams) -> Self;

    /// Constructs a [`CongressCurrent`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for current congress information.
    fn new_congress_current(params: CongressCurrentParams) -> Self;

    // ================================
    // Member Constructors
    // ================================

    /// Constructs a [`MemberList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing members.
    fn new_member_list(params: MemberListParams) -> Self;

    /// Constructs a [`MemberByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering members by congress.
    fn new_member_by_congress(congress: u32, params: MemberByCongressParams) -> Self;

    /// Constructs a [`MemberByState`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`state_code`]: The state code (e.g., "CA" for California).
    /// - [`params`]: Parameters for filtering members by state.
    fn new_member_by_state(state_code: String, params: MemberByStateParams) -> Self;

    /// Constructs a [`MemberByCongressStateDistrict`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`state_code`]: The state code.
    /// - [`district`]: The district number.
    /// - [`params`]: Parameters for filtering members by congress, state, and district.
    fn new_member_by_congress_state_district(
        congress: u32,
        state_code: String,
        district: u32,
        params: MemberByCongressStateDistrictParams,
    ) -> Self;

    /// Constructs a [`MemberDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`bio_guide_id`]: The bio guide ID of the member.
    /// - [`params`]: Parameters for member details.
    fn new_member_details(bio_guide_id: String, params: MemberDetailsParams) -> Self;

    /// Constructs a [`SponsorshipList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`bio_guide_id`]: The bio guide ID of the member.
    /// - [`params`]: Parameters for sponsorships.
    fn new_sponsorship_list(bio_guide_id: String, params: SponsorshipListParams) -> Self;

    /// Constructs a [`CosponsorshipList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`bio_guide_id`]: The bio guide ID of the member.
    /// - [`params`]: Parameters for cosponsorships.
    fn new_cosponsorship_list(bio_guide_id: String, params: CosponsorshipListParams) -> Self;

    // ================================
    // Committee Constructors
    // ================================

    /// Constructs a [`CommitteeList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing committees.
    fn new_committee_list(params: CommitteeListParams) -> Self;

    /// Constructs a [`CommitteeByChamber`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type (House or Senate).
    /// - [`params`]: Parameters for filtering committees by chamber.
    fn new_committee_by_chamber(
        chamber: ChamberType,
        params: CommitteeByChamberParams,
    ) -> Self;

    /// Constructs a [`CommitteeByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering committees by congress.
    fn new_committee_by_congress(congress: u32, params: CommitteeByCongressParams) -> Self;

    /// Constructs a [`CommitteeByCongressChamber`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`chamber`]: The chamber type.
    /// - [`params`]: Parameters for filtering committees by congress and chamber.
    fn new_committee_by_congress_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteeByCongressChamberParams,
    ) -> Self;

    /// Constructs a [`CommitteeDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee details.
    fn new_committee_details(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeDetailsParams,
    ) -> Self;

    /// Constructs a [`CommitteeBills`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee bills.
    fn new_committee_bills(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeBillsParams,
    ) -> Self;

    /// Constructs a [`CommitteeReports`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee reports.
    fn new_committee_reports(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeReportsParams,
    ) -> Self;

    /// Constructs a [`CommitteeNominations`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee nominations.
    fn new_committee_nominations(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeNominationsParams,
    ) -> Self;

    /// Constructs a [`CommitteeHouseCommunication`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee house communications.
    fn new_committee_house_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeHouseCommunicationParams,
    ) -> Self;

    /// Constructs a [`CommitteeSenateCommunication`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`chamber`]: The chamber type.
    /// - [`committee_code`]: The committee code.
    /// - [`params`]: Parameters for committee senate communications.
    fn new_committee_senate_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeSenateCommunicationParams,
    ) -> Self;

    // ================================
    // Nomination Constructors
    // ================================

    /// Constructs a [`NominationList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing nominations.
    fn new_nomination_list(params: NominationListParams) -> Self;

    /// Constructs a [`NominationByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering nominations by congress.
    fn new_nomination_by_congress(
        congress: u32,
        params: NominationByCongressParams,
    ) -> Self;

    /// Constructs a [`NominationDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`nomination_number`]: The nomination number.
    /// - [`params`]: Parameters for nomination details.
    fn new_nomination_details(
        congress: u32,
        nomination_number: String,
        params: NominationDetailsParams,
    ) -> Self;

    /// Constructs a [`Nominees`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`nomination_number`]: The nomination number.
    /// - [`ordinal`]: The ordinal number.
    /// - [`params`]: Parameters for nominees.
    fn new_nominees(
        congress: u32,
        nomination_number: String,
        ordinal: u32,
        params: NomineesParams,
    ) -> Self;

    /// Constructs a [`NominationActions`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`nomination_number`]: The nomination number.
    /// - [`params`]: Parameters for nomination actions.
    fn new_nomination_actions(
        congress: u32,
        nomination_number: String,
        params: NominationActionsParams,
    ) -> Self;

    /// Constructs a [`NominationCommittees`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`nomination_number`]: The nomination number.
    /// - [`params`]: Parameters for nomination committees.
    fn new_nomination_committees(
        congress: u32,
        nomination_number: String,
        params: NominationCommitteesParams,
    ) -> Self;

    /// Constructs a [`NominationHearings`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`nomination_number`]: The nomination number.
    /// - [`params`]: Parameters for nomination hearings.
    fn new_nomination_hearings(
        congress: u32,
        nomination_number: String,
        params: NominationHearingsParams,
    ) -> Self;

    // ================================
    // Treaty Constructors
    // ================================

    /// Constructs a [`TreatyList`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`params`]: Parameters for listing treaties.
    fn new_treaty_list(params: TreatyListParams) -> Self;

    /// Constructs a [`TreatyByCongress`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`params`]: Parameters for filtering treaties by congress.
    fn new_treaty_by_congress(congress: u32, params: TreatyByCongressParams) -> Self;

    /// Constructs a [`TreatyDetails`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`treaty_number`]: The treaty number.
    /// - [`params`]: Parameters for treaty details.
    fn new_treaty_details(
        congress: u32,
        treaty_number: u32,
        params: TreatyDetailsParams,
    ) -> Self;

    /// Constructs a [`TreatyPartitioned`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`treaty_number`]: The treaty number.
    /// - [`treaty_suffix`]: The treaty suffix.
    /// - [`params`]: Parameters for partitioned treaties.
    fn new_treaty_partitioned(
        congress: u32,
        treaty_number: u32,
        treaty_suffix: String,
        params: TreatyPartitionedParams,
    ) -> Self;

    /// Constructs a [`TreatyCommittees`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`treaty_number`]: The treaty number.
    /// - [`params`]: Parameters for treaty committees.
    fn new_treaty_committees(
        congress: u32,
        treaty_number: u32,
        params: TreatyCommitteesParams,
    ) -> Self;

    /// Constructs a [`TreatyActions`] endpoint variant.
    ///
    /// # Parameters
    ///
    /// - [`congress`]: The congress number.
    /// - [`treaty_number`]: The treaty number.
    /// - [`params`]: Parameters for treaty actions.
    fn new_treaty_actions(
        congress: u32,
        treaty_number: u32,
        params: TreatyActionsParams,
    ) -> Self;

    fn new_bill_related(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillRelatedParams,
    ) -> Self;

    /// Constructs a [`BillSubjects`] endpoint variant.
    fn new_bill_subjects(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillSubjectsParams,
    ) -> Self;

    /// Constructs a [`BillSummaries`] endpoint variant.
    fn new_bill_summaries(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillSummariesParams,
    ) -> Self;

    /// Constructs a [`BillText`] endpoint variant.
    fn new_bill_text(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillTextParams,
    ) -> Self;

    /// Constructs a [`BillTitles`] endpoint variant.
    fn new_bill_titles(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillTitlesParams,
    ) -> Self;

    // ================================
    // Committee Report Constructors
    // ================================

    /// Constructs a [`CommitteeReportList`] endpoint variant.
    fn new_committee_report_list(params: CommitteeReportListParams) -> Self;

    /// Constructs a [`CommitteeReportByCongress`] endpoint variant.
    fn new_committee_report_by_congress(
        congress: u32,
        params: CommitteeReportByCongressParams,
    ) -> Self;

    /// Constructs a [`CommitteeReportByType`] endpoint variant.
    fn new_committee_report_by_type(
        congress: u32,
        report_type: CommitteeReportType,
        params: CommitteeReportByTypeParams,
    ) -> Self;

    /// Constructs a [`CommitteeReportDetails`] endpoint variant.
    fn new_committee_report_details(
        congress: u32,
        report_type: CommitteeReportType,
        report_number: u32,
        params: CommitteeReportDetailsParams,
    ) -> Self;

    /// Constructs a [`CommitteeReportText`] endpoint variant.
    fn new_committee_report_text(
        congress: u32,
        report_type: CommitteeReportType,
        report_number: u32,
        params: CommitteeReportTextParams,
    ) -> Self;

    // ================================
    // Committee Print Constructors
    // ================================

    /// Constructs a [`CommitteePrintList`] endpoint variant.
    fn new_committee_print_list(params: CommitteePrintListParams) -> Self;

    /// Constructs a [`CommitteePrintByCongress`] endpoint variant.
    fn new_committee_print_by_congress(
        congress: u32,
        params: CommitteePrintByCongressParams,
    ) -> Self;

    /// Constructs a [`CommitteePrintByCongressChamber`] endpoint variant.
    fn new_committee_print_by_congress_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteePrintByCongressChamberParams,
    ) -> Self;

    /// Constructs a [`CommitteePrintByJacketNumber`] endpoint variant.
    fn new_committee_print_by_jacket_number(
        congress: u32,
        jacket_number: u32,
        params: CommitteePrintByJacketNumberParams,
    ) -> Self;

    /// Constructs a [`CommitteePrintText`] endpoint variant.
    fn new_committee_print_text(
        congress: u32,
        chamber: String,
        jacket_number: u32,
        params: CommitteePrintDetailsParams,
    ) -> Self;

    // ================================
    // Committee Meeting Constructors
    // ================================

    /// Constructs a [`CommitteeMeetingList`] endpoint variant.
    fn new_committee_meeting_list(params: CommitteeMeetingListParams) -> Self;

    /// Constructs a [`CommitteeMeetingByCongress`] endpoint variant.
    fn new_committee_meeting_by_congress(
        congress: u32,
        params: CommitteeMeetingByCongressParams,
    ) -> Self;

    /// Constructs a [`CommitteeMeetingByChamber`] endpoint variant.
    fn new_committee_meeting_by_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteeMeetingByChamberParams,
    ) -> Self;

    /// Constructs a [`CommitteeMeetingByEvent`] endpoint variant.
    fn new_committee_meeting_by_event(
        congress: u32,
        chamber: ChamberType,
        event_id: String,
        params: CommitteeMeetingByEventParams,
    ) -> Self;

    // ================================
    // Hearing Constructors
    // ================================

    /// Constructs a [`HearingList`] endpoint variant.
    fn new_hearing_list(params: HearingListParams) -> Self;

    /// Constructs a [`HearingByCongress`] endpoint variant.
    fn new_hearing_by_congress(congress: u32, params: HearingByCongressParams) -> Self;

    /// Constructs a [`HearingByChamber`] endpoint variant.
    fn new_hearing_by_chamber(
        congress: u32,
        chamber: ChamberType,
        params: HearingByChamberParams,
    ) -> Self;

    /// Constructs a [`HearingByJacketNumber`] endpoint variant.
    fn new_hearing_by_jacket_number(
        congress: u32,
        chamber: ChamberType,
        jacket_number: u32,
        params: HearingByJacketNumberParams,
    ) -> Self;

    // ================================
    // Congressional Record Constructors
    // ================================

    /// Constructs a [`CongressionalRecordList`] endpoint variant.
    fn new_congressional_record_list(params: CongressionalRecordListParams) -> Self;

    // ================================
    // Daily Congressional Record Constructors
    // ================================

    /// Constructs a [`DailyCongressionalRecordList`] endpoint variant.
    fn new_daily_congressional_record_list(
        params: DailyCongressionalRecordListParams,
    ) -> Self;

    /// Constructs a [`DailyCongressionalRecordVolume`] endpoint variant.
    fn new_daily_congressional_record_volume(
        volume_number: u32,
        params: DailyCongressionalVolumeNumberParams,
    ) -> Self;

    /// Constructs a [`DailyCongressionalRecordVolumeIssue`] endpoint variant.
    fn new_daily_congressional_record_volume_issue(
        volume_number: u32,
        issue_number: u32,
        params: DailyCongressionalVolumeNumberIssueNumberParams,
    ) -> Self;

    /// Constructs a [`DailyCongressionalRecordArticles`] endpoint variant.
    fn new_daily_congressional_record_articles(
        volume_number: u32,
        issue_number: u32,
        params: DailyCongressionalVolumeNumberIssueNumberParams,
    ) -> Self;

    // ================================
    // Bound Congressional Record Constructors
    // ================================

    /// Constructs a [`BoundCongressionalRecordList`] endpoint variant.
    fn new_bound_congressional_record_list(params: BoundCongressionalRecordParams) -> Self;

    /// Constructs a [`BoundCongressionalRecordByYear`] endpoint variant.
    fn new_bound_congressional_record_by_year(
        year: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self;

    /// Constructs a [`BoundCongressionalRecordByYearMonth`] endpoint variant.
    fn new_bound_congressional_record_by_year_month(
        year: u32,
        month: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self;

    /// Constructs a [`BoundCongressionalRecordByYearMonthDay`] endpoint variant.
    fn new_bound_congressional_record_by_year_month_day(
        year: u32,
        month: u32,
        day: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self;

    // ================================
    // House Communication Constructors
    // ================================

    /// Constructs a [`HouseCommunicationList`] endpoint variant.
    fn new_house_communication_list(params: CommunicationParams) -> Self;

    /// Constructs a [`HouseCommunicationByCongress`] endpoint variant.
    fn new_house_communication_by_congress(
        congress: u32,
        params: CommunicationParams,
    ) -> Self;

    /// Constructs a [`HouseCommunicationByType`] endpoint variant.
    fn new_house_communication_by_type(
        congress: u32,
        communication_type: CommunicationType,
        params: CommunicationParams,
    ) -> Self;

    /// Constructs a [`HouseCommunicationDetails`] endpoint variant.
    fn new_house_communication_details(
        congress: u32,
        communication_type: CommunicationType,
        communication_number: u32,
        params: CommunicationDetailsParams,
    ) -> Self;

    // ================================
    // House Requirement Constructors
    // ================================

    /// Constructs a [`HouseRequirementList`] endpoint variant.
    fn new_house_requirement_list(params: RequirementParams) -> Self;

    /// Constructs a [`HouseRequirementDetails`] endpoint variant.
    fn new_house_requirement_details(
        requirement_number: u32,
        params: RequirementDetailsParams,
    ) -> Self;

    /// Constructs a [`HouseRequirementMatching`] endpoint variant.
    fn new_house_requirement_matching(
        requirement_number: u32,
        params: RequirementParams,
    ) -> Self;

    // ================================
    // Senate Communication Constructors
    // ================================

    /// Constructs a [`SenateCommunicationList`] endpoint variant.
    fn new_senate_communication_list(params: CommunicationParams) -> Self;

    /// Constructs a [`SenateCommunicationByCongress`] endpoint variant.
    fn new_senate_communication_by_congress(
        congress: u32,
        params: CommunicationParams,
    ) -> Self;

    /// Constructs a [`SenateCommunicationByType`] endpoint variant.
    fn new_senate_communication_by_type(
        congress: u32,
        communication_type: CommunicationType,
        params: CommunicationParams,
    ) -> Self;

    /// Constructs a [`SenateCommunicationDetails`] endpoint variant.
    fn new_senate_communication_details(
        congress: u32,
        communication_type: CommunicationType,
        communication_number: u32,
        params: CommunicationDetailsParams,
    ) -> Self;
}

/// Implementation of the [`NewEndpoint`] trait for the [`Endpoints`] enum.
impl NewEndpoint for Endpoints {
    fn new_generic(endpoint: String, params: GenericParams) -> Self {
        Endpoints::Generic(endpoint, params)
    }

    // ================================
    // Bill Endpoints
    // ================================

    fn new_bill_list(params: BillListParams) -> Self {
        Endpoints::BillList(params)
    }

    fn new_bill_by_congress(congress: u32, params: BillByCongressParams) -> Self {
        Endpoints::BillByCongress(congress, params)
    }

    fn new_bill_by_type(congress: u32, bill_type: BillType, params: BillByTypeParams) -> Self {
        Endpoints::BillByType(congress, bill_type, params)
    }

    fn new_bill_details(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillDetailsParams,
    ) -> Self {
        Endpoints::BillDetails(congress, bill_type, bill_number, params)
    }

    fn new_bill_actions(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillActionsParams,
    ) -> Self {
        Endpoints::BillActions(congress, bill_type, bill_number, params)
    }

    fn new_bill_amendments(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillAmendmentsParams,
    ) -> Self {
        Endpoints::BillAmendments(congress, bill_type, bill_number, params)
    }

    fn new_bill_committees(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillCommitteesParams,
    ) -> Self {
        Endpoints::BillCommittees(congress, bill_type, bill_number, params)
    }

    fn new_bill_cosponsors(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillCosponsorsParams,
    ) -> Self {
        Endpoints::BillCosponsors(congress, bill_type, bill_number, params)
    }

    // ================================
    // Law Endpoints
    // ================================

    fn new_law_type(congress: u32, law_type: LawType, params: LawParams) -> Self {
        Endpoints::LawByType(congress, law_type, params)
    }

    fn new_law_by_congress(congress: u32, params: LawParams) -> Self {
        Endpoints::LawByCongress(congress, params)
    }

    fn new_law_details(congress: u32, law_type: LawType, law_number: u32, params: LawParams) -> Self {
        Endpoints::LawDetails(congress, law_type, law_number, params)
    }

    // ================================
    // Amendment Endpoints
    // ================================

    fn new_amendment_list(params: AmendmentListParams) -> Self {
        Endpoints::AmendmentList(params)
    }

    fn new_amendment_by_congress(congress: u32, params: AmendmentByCongressParams) -> Self {
        Endpoints::AmendmentByCongress(congress, params)
    }

    fn new_amendment_by_type(
        congress: u32,
        amendment_type: AmendmentType,
        params: AmendmentByTypeParams,
    ) -> Self {
        Endpoints::AmendmentByType(congress, amendment_type, params)
    }

    fn new_amendment_details(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: u32,
        params: AmendmentDetailsParams,
    ) -> Self {
        Endpoints::AmendmentDetails(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_actions(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentActionsParams,
    ) -> Self {
        Endpoints::AmendmentActions(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_cosponsors(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentCosponsorsParams,
    ) -> Self {
        Endpoints::AmendmentCosponsors(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_amendments(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentAmendmentsParams,
    ) -> Self {
        Endpoints::AmendmentAmendments(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_text(
        congress: u32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentTextParams,
    ) -> Self {
        Endpoints::AmendmentText(congress, amendment_type, amendment_number, params)
    }

    // ================================
    // Summaries Endpoints
    // ================================

    fn new_summaries_list(params: SummariesListParams) -> Self {
        Endpoints::SummariesList(params)
    }

    fn new_summaries_by_congress(congress: u32, params: SummariesByCongressParams) -> Self {
        Endpoints::SummariesByCongress(congress, params)
    }

    fn new_summaries_by_type(
        congress: u32,
        bill_type: BillType,
        params: SummariesByTypeParams,
    ) -> Self {
        Endpoints::SummariesByType(congress, bill_type, params)
    }

    // ================================
    // Congress Endpoints
    // ================================

    fn new_congress_list(params: CongressListParams) -> Self {
        Endpoints::CongressList(params)
    }

    fn new_congress_details(congress: u32, params: CongressDetailsParams) -> Self {
        Endpoints::CongressDetails(congress, params)
    }

    fn new_congress_current(params: CongressCurrentParams) -> Self {
        Endpoints::CongressCurrent(params)
    }

    // ================================
    // Member Endpoints
    // ================================

    fn new_member_list(params: MemberListParams) -> Self {
        Endpoints::MemberList(params)
    }

    fn new_member_by_congress(congress: u32, params: MemberByCongressParams) -> Self {
        Endpoints::MemberByCongress(congress, params)
    }

    fn new_member_by_state(state_code: String, params: MemberByStateParams) -> Self {
        Endpoints::MemberByState(state_code, params)
    }

    fn new_member_by_congress_state_district(
        congress: u32,
        state_code: String,
        district: u32,
        params: MemberByCongressStateDistrictParams,
    ) -> Self {
        Endpoints::MemberByCongressStateDistrict(congress, state_code, district, params)
    }

    fn new_member_details(bio_guide_id: String, params: MemberDetailsParams) -> Self {
        Endpoints::MemberDetails(bio_guide_id, params)
    }

    fn new_sponsorship_list(bio_guide_id: String, params: SponsorshipListParams) -> Self {
        Endpoints::SponsorshipList(bio_guide_id, params)
    }

    fn new_cosponsorship_list(bio_guide_id: String, params: CosponsorshipListParams) -> Self {
        Endpoints::CosponsorshipList(bio_guide_id, params)
    }

    // ================================
    // Committee Endpoints
    // ================================

    fn new_committee_list(params: CommitteeListParams) -> Self {
        Endpoints::CommitteeList(params)
    }

    fn new_committee_by_chamber(
        chamber: ChamberType,
        params: CommitteeByChamberParams,
    ) -> Self {
        Endpoints::CommitteeByChamber(chamber, params)
    }

    fn new_committee_by_congress(congress: u32, params: CommitteeByCongressParams) -> Self {
        Endpoints::CommitteeByCongress(congress, params)
    }

    fn new_committee_by_congress_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteeByCongressChamberParams,
    ) -> Self {
        Endpoints::CommitteeByCongressChamber(congress, chamber, params)
    }

    fn new_committee_details(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeDetailsParams,
    ) -> Self {
        Endpoints::CommitteeDetails(chamber, committee_code, params)
    }

    fn new_committee_bills(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeBillsParams,
    ) -> Self {
        Endpoints::CommitteeBills(chamber, committee_code, params)
    }

    fn new_committee_reports(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeReportsParams,
    ) -> Self {
        Endpoints::CommitteeReports(chamber, committee_code, params)
    }

    fn new_committee_nominations(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeNominationsParams,
    ) -> Self {
        Endpoints::CommitteeNominations(chamber, committee_code, params)
    }

    fn new_committee_house_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeHouseCommunicationParams,
    ) -> Self {
        Endpoints::CommitteeHouseCommunication(chamber, committee_code, params)
    }

    fn new_committee_senate_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeSenateCommunicationParams,
    ) -> Self {
        Endpoints::CommitteeSenateCommunication(chamber, committee_code, params)
    }

    // ================================
    // Nomination Endpoints
    // ================================

    fn new_nomination_list(params: NominationListParams) -> Self {
        Endpoints::NominationList(params)
    }

    fn new_nomination_by_congress(
        congress: u32,
        params: NominationByCongressParams,
    ) -> Self {
        Endpoints::NominationByCongress(congress, params)
    }

    fn new_nomination_details(
        congress: u32,
        nomination_number: String,
        params: NominationDetailsParams,
    ) -> Self {
        Endpoints::NominationDetails(congress, nomination_number, params)
    }

    fn new_nominees(
        congress: u32,
        nomination_number: String,
        ordinal: u32,
        params: NomineesParams,
    ) -> Self {
        Endpoints::Nominees(congress, nomination_number, ordinal, params)
    }

    fn new_nomination_actions(
        congress: u32,
        nomination_number: String,
        params: NominationActionsParams,
    ) -> Self {
        Endpoints::NominationActions(congress, nomination_number, params)
    }

    fn new_nomination_committees(
        congress: u32,
        nomination_number: String,
        params: NominationCommitteesParams,
    ) -> Self {
        Endpoints::NominationCommittees(congress, nomination_number, params)
    }

    fn new_nomination_hearings(
        congress: u32,
        nomination_number: String,
        params: NominationHearingsParams,
    ) -> Self {
        Endpoints::NominationHearings(congress, nomination_number, params)
    }

    // ================================
    // Treaty Endpoints
    // ================================

    fn new_treaty_list(params: TreatyListParams) -> Self {
        Endpoints::TreatyList(params)
    }

    fn new_treaty_by_congress(congress: u32, params: TreatyByCongressParams) -> Self {
        Endpoints::TreatyByCongress(congress, params)
    }

    fn new_treaty_details(
        congress: u32,
        treaty_number: u32,
        params: TreatyDetailsParams,
    ) -> Self {
        Endpoints::TreatyDetails(congress, treaty_number, params)
    }

    fn new_treaty_partitioned(
        congress: u32,
        treaty_number: u32,
        treaty_suffix: String,
        params: TreatyPartitionedParams,
    ) -> Self {
        Endpoints::TreatyPartitioned(congress, treaty_number, treaty_suffix, params)
    }

    fn new_treaty_committees(
        congress: u32,
        treaty_number: u32,
        params: TreatyCommitteesParams,
    ) -> Self {
        Endpoints::TreatyCommittees(congress, treaty_number, params)
    }

    fn new_treaty_actions(
        congress: u32,
        treaty_number: u32,
        params: TreatyActionsParams,
    ) -> Self {
        Endpoints::TreatyActions(congress, treaty_number, params)
    }

        fn new_bill_related(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillRelatedParams,
    ) -> Self {
        Endpoints::BillRelated(congress, bill_type, bill_number, params)
    }

    fn new_bill_subjects(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillSubjectsParams,
    ) -> Self {
        Endpoints::BillSubjects(congress, bill_type, bill_number, params)
    }

    fn new_bill_summaries(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillSummariesParams,
    ) -> Self {
        Endpoints::BillSummaries(congress, bill_type, bill_number, params)
    }

    fn new_bill_text(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillTextParams,
    ) -> Self {
        Endpoints::BillText(congress, bill_type, bill_number, params)
    }

    fn new_bill_titles(
        congress: u32,
        bill_type: BillType,
        bill_number: u32,
        params: BillTitlesParams,
    ) -> Self {
        Endpoints::BillTitles(congress, bill_type, bill_number, params)
    }

    // ================================
    // Committee Report Endpoints
    // ================================

    fn new_committee_report_list(params: CommitteeReportListParams) -> Self {
        Endpoints::CommitteeReportList(params)
    }

    fn new_committee_report_by_congress(
        congress: u32,
        params: CommitteeReportByCongressParams,
    ) -> Self {
        Endpoints::CommitteeReportByCongress(congress, params)
    }

    fn new_committee_report_by_type(
        congress: u32,
        report_type: CommitteeReportType,
        params: CommitteeReportByTypeParams,
    ) -> Self {
        Endpoints::CommitteeReportByType(congress, report_type, params)
    }

    fn new_committee_report_details(
        congress: u32,
        report_type: CommitteeReportType,
        report_number: u32,
        params: CommitteeReportDetailsParams,
    ) -> Self {
        Endpoints::CommitteeReportDetails(congress, report_type, report_number, params)
    }

    fn new_committee_report_text(
        congress: u32,
        report_type: CommitteeReportType,
        report_number: u32,
        params: CommitteeReportTextParams,
    ) -> Self {
        Endpoints::CommitteeReportText(congress, report_type, report_number, params)
    }

    // ================================
    // Committee Print Endpoints
    // ================================

    fn new_committee_print_list(params: CommitteePrintListParams) -> Self {
        Endpoints::CommitteePrintList(params)
    }

    fn new_committee_print_by_congress(
        congress: u32,
        params: CommitteePrintByCongressParams,
    ) -> Self {
        Endpoints::CommitteePrintByCongress(congress, params)
    }

    fn new_committee_print_by_congress_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteePrintByCongressChamberParams,
    ) -> Self {
        Endpoints::CommitteePrintByCongressChamber(congress, chamber, params)
    }

    fn new_committee_print_by_jacket_number(
        congress: u32,
        jacket_number: u32,
        params: CommitteePrintByJacketNumberParams,
    ) -> Self {
        Endpoints::CommitteePrintByJacketNumber(congress, jacket_number, params)
    }

    fn new_committee_print_text(
        congress: u32,
        chamber: String,
        jacket_number: u32,
        params: CommitteePrintDetailsParams,
    ) -> Self {
        Endpoints::CommitteePrintText(congress, chamber, jacket_number, params)
    }

    // ================================
    // Committee Meeting Endpoints
    // ================================

    fn new_committee_meeting_list(params: CommitteeMeetingListParams) -> Self {
        Endpoints::CommitteeMeetingList(params)
    }

    fn new_committee_meeting_by_congress(
        congress: u32,
        params: CommitteeMeetingByCongressParams,
    ) -> Self {
        Endpoints::CommitteeMeetingByCongress(congress, params)
    }

    fn new_committee_meeting_by_chamber(
        congress: u32,
        chamber: ChamberType,
        params: CommitteeMeetingByChamberParams,
    ) -> Self {
        Endpoints::CommitteeMeetingByChamber(congress, chamber, params)
    }
    fn new_committee_meeting_by_event(
        congress: u32,
        chamber: ChamberType,
        event_id: String,
        params: CommitteeMeetingByEventParams,
    ) -> Self {
        Endpoints::CommitteeMeetingByEvent(congress, chamber, event_id, params)
    }

    // ================================
    // Hearing Constructors
    // ================================

    fn new_hearing_list(params: HearingListParams) -> Self {
        Endpoints::HearingList(params)
    }

    fn new_hearing_by_congress(congress: u32, params: HearingByCongressParams) -> Self {
        Endpoints::HearingByCongress(congress, params)
    }

    fn new_hearing_by_chamber(
        congress: u32,
        chamber: ChamberType,
        params: HearingByChamberParams,
    ) -> Self {
        Endpoints::HearingByChamber(congress, chamber, params)
    }

    fn new_hearing_by_jacket_number(
        congress: u32,
        chamber: ChamberType,
        jacket_number: u32,
        params: HearingByJacketNumberParams,
    ) -> Self {
        Endpoints::HearingByJacketNumber(congress, chamber, jacket_number, params)
    }

    // ================================
    // Congressional Record Constructors
    // ================================

    fn new_congressional_record_list(params: CongressionalRecordListParams) -> Self {
        Endpoints::CongressionalRecordList(params)
    }

    // ====================================
    // Daily Congressional Record Constructors
    // ====================================

    fn new_daily_congressional_record_list(
        params: DailyCongressionalRecordListParams,
    ) -> Self {
        Endpoints::DailyCongressionalRecordList(params)
    }

    fn new_daily_congressional_record_volume(
        volume_number: u32,
        params: DailyCongressionalVolumeNumberParams,
    ) -> Self {
        Endpoints::DailyCongressionalRecordVolume(volume_number, params)
    }

    fn new_daily_congressional_record_volume_issue(
        volume_number: u32,
        issue_number: u32,
        params: DailyCongressionalVolumeNumberIssueNumberParams,
    ) -> Self {
        Endpoints::DailyCongressionalRecordVolumeIssue(volume_number, issue_number, params)
    }

    fn new_daily_congressional_record_articles(
        volume_number: u32,
        issue_number: u32,
        params: DailyCongressionalVolumeNumberIssueNumberParams,
    ) -> Self {
        Endpoints::DailyCongressionalRecordArticles(volume_number, issue_number, params)
    }

    // ================================
    // Bound Congressional Record Constructors
    // ================================

    fn new_bound_congressional_record_list(params: BoundCongressionalRecordParams) -> Self {
        Endpoints::BoundCongressionalRecordList(params)
    }

    fn new_bound_congressional_record_by_year(
        year: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self {
        Endpoints::BoundCongressionalRecordByYear(year, params)
    }

    fn new_bound_congressional_record_by_year_month(
        year: u32,
        month: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self {
        Endpoints::BoundCongressionalRecordByYearMonth(year, month, params)
    }

    fn new_bound_congressional_record_by_year_month_day(
        year: u32,
        month: u32,
        day: u32,
        params: BoundCongressionalRecordParams,
    ) -> Self {
        Endpoints::BoundCongressionalRecordByYearMonthDay(year, month, day, params)
    }

    // ================================
    // House Communication Constructors
    // ================================

    fn new_house_communication_list(params: CommunicationParams) -> Self {
        Endpoints::HouseCommunicationList(params)
    }

    fn new_house_communication_by_congress(congress: u32, params: CommunicationParams) -> Self {
        Endpoints::HouseCommunicationByCongress(congress, params)
    }

    fn new_house_communication_by_type(
        congress: u32,
        communication_type: CommunicationType,
        params: CommunicationParams,
    ) -> Self {
        Endpoints::HouseCommunicationByType(congress, communication_type, params)
    }

    fn new_house_communication_details(
        congress: u32,
        communication_type: CommunicationType,
        communication_number: u32,
        params: CommunicationDetailsParams,
    ) -> Self {
        Endpoints::HouseCommunicationDetails(
            congress,
            communication_type,
            communication_number,
            params,
        )
    }

    // ================================
    // House Requirement Constructors
    // ================================

    fn new_house_requirement_list(params: RequirementParams) -> Self {
        Endpoints::HouseRequirementList(params)
    }

    fn new_house_requirement_details(
        requirement_number: u32,
        params: RequirementDetailsParams,
    ) -> Self {
        Endpoints::HouseRequirementDetails(requirement_number, params)
    }

    fn new_house_requirement_matching(
        requirement_number: u32,
        params: RequirementParams,
    ) -> Self {
        Endpoints::HouseRequirementMatching(requirement_number, params)
    }

    // ================================
    // Senate Communication Constructors
    // ================================

    fn new_senate_communication_list(params: CommunicationParams) -> Self {
        Endpoints::SenateCommunicationList(params)
    }

    fn new_senate_communication_by_congress(congress: u32, params: CommunicationParams) -> Self {
        Endpoints::SenateCommunicationByCongress(congress, params)
    }

    fn new_senate_communication_by_type(
        congress: u32,
        communication_type: CommunicationType,
        params: CommunicationParams,
    ) -> Self {
        Endpoints::SenateCommunicationByType(congress, communication_type, params)
    }

    fn new_senate_communication_details(
        congress: u32,
        communication_type: CommunicationType,
        communication_number: u32,
        params: CommunicationDetailsParams,
    ) -> Self {
        Endpoints::SenateCommunicationDetails(
            congress,
            communication_type,
            communication_number,
            params,
        )
    }
}

