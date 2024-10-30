//! # `endpoints` Module
//! 
//! This module defines the `Endpoints` enum, which represents the various API endpoints
//! available in the US Congress API. Each variant encapsulates the necessary parameters
//! required to interact with a specific endpoint.
//! 
//! Additionally, the `NewEndpoint` trait provides constructor methods for creating
//! instances of each `Endpoints` variant in a standardized manner.
//! 
//! ## Example
//! 
//! ```rust
//! use cdg_api::endpoints::{Endpoints, NewEndpoint};
//! use cdg_api::param_models::{BillListParams, FormatType};
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
use crate::param_models::*;

/// Each variant of the `Endpoints` enum corresponds to a specific API endpoint,
/// encapsulating the necessary parameters required to interact with that endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Endpoints {
    /// Endpoint for manual API requests, where the user provides the entire endpoint.
    /// The base URL, and the API key are automatically appended to the provided endpoint.
    Manual(String),

    // ================================
    // Bill Endpoints
    // ================================

    /// Endpoint to list bills based on provided parameters.
    BillList(BillListParams),

    /// Endpoint to retrieve bills by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillByCongressParams`: Additional parameters for filtering bills.
    BillByCongress(i32, BillByCongressParams),

    /// Endpoint to get bills filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill (e.g., House, Senate).
    /// - `BillByTypeParams`: Additional parameters for filtering bills by type.
    BillByType(i32, BillType, BillByTypeParams),

    /// Endpoint to retrieve detailed information about a specific bill.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `String`: The bill number.
    /// - `BillDetailsParams`: Additional parameters for bill details.
    BillDetails(i32, BillType, i32, BillDetailsParams),

    /// Endpoint to fetch actions taken on a specific bill.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `String`: The bill number.
    /// - `BillActionsParams`: Additional parameters for bill actions.
    BillActions(i32, BillType, String, BillActionsParams),

    /// Endpoint to list amendments of a specific bill.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `String`: The bill number.
    /// - `BillAmendmentsParams`: Additional parameters for bill amendments.
    BillAmendments(i32, BillType, String, BillAmendmentsParams),

    /// Endpoint to get committees associated with a specific bill.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `String`: The bill number.
    /// - `BillCommitteesParams`: Additional parameters for bill committees.
    BillCommittees(i32, BillType, String, BillCommitteesParams),

    /// Endpoint to retrieve cosponsors of a specific bill.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `String`: The bill number.
    /// - `BillCosponsorsParams`: Additional parameters for bill cosponsors.
    BillCosponsors(i32, BillType, String, BillCosponsorsParams),

    // ================================
    // Law Endpoints
    // ================================

    /// Endpoint to list laws based on provided parameters.
    LawByType(i32, LawType, LawParams),

    /// Endpoint to retrieve laws by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `LawParams`: Additional parameters for filtering laws.
    LawByCongress(i32, LawParams),

    /// Endpoint to get detailed information about a specific law.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `LawType`: The type of law.
    /// - `i32`: The law number.
    /// - `LawDetailsParams`: Additional parameters for law details.
    LawDetails(i32, LawType, i32, LawParams),

    // ================================
    // Amendment Endpoints
    // ================================

    /// Endpoint to list amendments based on provided parameters.
    AmendmentList(AmendmentListParams),

    /// Endpoint to retrieve amendments by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentByCongressParams`: Additional parameters for filtering amendments.
    AmendmentByCongress(i32, AmendmentByCongressParams),

    /// Endpoint to get amendments filtered by type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `AmendmentByTypeParams`: Additional parameters for filtering amendments by type.
    AmendmentByType(i32, AmendmentType, AmendmentByTypeParams),

    /// Endpoint to retrieve detailed information about a specific amendment.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `String`: The amendment number.
    /// - `AmendmentDetailsParams`: Additional parameters for amendment details.
    AmendmentDetails(i32, AmendmentType, String, AmendmentDetailsParams),

    /// Endpoint to fetch actions taken on a specific amendment.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `String`: The amendment number.
    /// - `AmendmentActionsParams`: Additional parameters for amendment actions.
    AmendmentActions(i32, AmendmentType, String, AmendmentActionsParams),

    /// Endpoint to retrieve cosponsors of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `String`: The amendment number.
    /// - `AmendmentCosponsorsParams`: Additional parameters for amendment cosponsors.
    AmendmentCosponsors(i32, AmendmentType, String, AmendmentCosponsorsParams),

    /// Endpoint to list amendments of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `String`: The amendment number.
    /// - `AmendmentAmendmentsParams`: Additional parameters for amendment amendments.
    AmendmentAmendments(i32, AmendmentType, String, AmendmentAmendmentsParams),

    /// Endpoint to retrieve the text of a specific amendment.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `AmendmentType`: The type of amendment.
    /// - `String`: The amendment number.
    /// - `AmendmentTextParams`: Additional parameters for amendment text.
    AmendmentText(i32, AmendmentType, String, AmendmentTextParams),

    // ================================
    // Summaries Endpoints
    // ================================

    /// Endpoint to list summaries based on provided parameters.
    SummariesList(SummariesListParams),

    /// Endpoint to retrieve summaries by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `SummariesByCongressParams`: Additional parameters for filtering summaries.
    SummariesByCongress(i32, SummariesByCongressParams),

    /// Endpoint to get summaries filtered by bill type within a specific congress.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `BillType`: The type of bill.
    /// - `SummariesByTypeParams`: Additional parameters for filtering summaries by type.
    SummariesByType(i32, BillType, SummariesByTypeParams),

    // ================================
    // Congress Endpoints
    // ================================

    /// Endpoint to list congress sessions based on provided parameters.
    CongressList(CongressListParams),

    /// Endpoint to retrieve detailed information about a specific congress session.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `CongressDetailsParams`: Additional parameters for congress details.
    CongressDetails(i32, CongressDetailsParams),

    /// Endpoint to get information about the current congress session.
    ///
    /// # Parameters
    ///
    /// - `CongressCurrentParams`: Additional parameters for current congress.
    CongressCurrent(CongressCurrentParams),

    // ================================
    // Member Endpoints
    // ================================

    /// Endpoint to list members of Congress based on provided parameters.
    MemberList(MemberListParams),

    /// Endpoint to retrieve members by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `MemberByCongressParams`: Additional parameters for filtering members by congress.
    MemberByCongress(i32, MemberByCongressParams),

    /// Endpoint to get members representing a specific state.
    ///
    /// # Parameters
    ///
    /// - `String`: The state code (e.g., "NY" for New York).
    /// - `MemberByStateParams`: Additional parameters for filtering members by state.
    MemberByState(String, MemberByStateParams),

    /// Endpoint to retrieve members by congress, state, and district.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The state code.
    /// - `i32`: The district number.
    /// - `MemberByCongressStateDistrictParams`: Additional parameters for filtering.
    MemberByCongressStateDistrict(i32, String, i32, MemberByCongressStateDistrictParams),

    /// Endpoint to get detailed information about a specific member.
    ///
    /// # Parameters
    ///
    /// - `String`: The bio guide ID of the member.
    /// - `MemberDetailsParams`: Additional parameters for member details.
    MemberDetails(String, MemberDetailsParams),

    /// Endpoint to list sponsorships of a specific member.
    ///
    /// # Parameters
    ///
    /// - `String`: The bio guide ID of the member.
    /// - `SponsorshipListParams`: Additional parameters for sponsorships.
    SponsorshipList(String, SponsorshipListParams),

    /// Endpoint to list cosponsorships of a specific member.
    ///
    /// # Parameters
    ///
    /// - `String`: The bio guide ID of the member.
    /// - `CosponsorshipListParams`: Additional parameters for cosponsorships.
    CosponsorshipList(String, CosponsorshipListParams),

    // ================================
    // Committee Endpoints
    // ================================

    /// Endpoint to list committees based on provided parameters.
    CommitteeList(CommitteeListParams),

    /// Endpoint to retrieve committees by chamber (House or Senate).
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type (House or Senate).
    /// - `CommitteeByChamberParams`: Additional parameters for filtering committees by chamber.
    CommitteeByChamber(ChamberType, CommitteeByChamberParams),

    /// Endpoint to get committees associated with a specific congress.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `CommitteeByCongressParams`: Additional parameters for filtering committees by congress.
    CommitteeByCongress(i32, CommitteeByCongressParams),

    /// Endpoint to retrieve committees by congress and chamber.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `ChamberType`: The chamber type.
    /// - `CommitteeByCongressChamberParams`: Additional parameters for filtering.
    CommitteeByCongressChamber(i32, ChamberType, CommitteeByCongressChamberParams),

    /// Endpoint to get detailed information about a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeDetailsParams`: Additional parameters for committee details.
    CommitteeDetails(ChamberType, String, CommitteeDetailsParams),

    /// Endpoint to list bills under a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeBillsParams`: Additional parameters for committee bills.
    CommitteeBills(ChamberType, String, CommitteeBillsParams),

    /// Endpoint to retrieve reports from a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeReportsParams`: Additional parameters for committee reports.
    CommitteeReports(ChamberType, String, CommitteeReportsParams),

    /// Endpoint to list nominations handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeNominationsParams`: Additional parameters for committee nominations.
    CommitteeNominations(ChamberType, String, CommitteeNominationsParams),

    /// Endpoint to retrieve house communications handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeHouseCommunicationParams`: Additional parameters for house communications.
    CommitteeHouseCommunication(ChamberType, String, CommitteeHouseCommunicationParams),

    /// Endpoint to retrieve senate communications handled by a specific committee.
    ///
    /// # Parameters
    ///
    /// - `ChamberType`: The chamber type.
    /// - `String`: The committee code.
    /// - `CommitteeSenateCommunicationParams`: Additional parameters for senate communications.
    CommitteeSenateCommunication(ChamberType, String, CommitteeSenateCommunicationParams),

    // ================================
    // Nomination Endpoints
    // ================================

    /// Endpoint to list nominations based on provided parameters.
    NominationList(NominationListParams),

    /// Endpoint to retrieve nominations by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `NominationByCongressParams`: Additional parameters for filtering nominations.
    NominationByCongress(i32, NominationByCongressParams),

    /// Endpoint to get detailed information about a specific nomination.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The nomination number.
    /// - `NominationDetailsParams`: Additional parameters for nomination details.
    NominationDetails(i32, String, NominationDetailsParams),

    /// Endpoint to list nominees of a specific nomination.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The nomination number.
    /// - `i32`: The ordinal number.
    /// - `NomineesParams`: Additional parameters for nominees.
    Nominees(i32, String, i32, NomineesParams),

    /// Endpoint to fetch actions taken on a specific nomination.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The nomination number.
    /// - `NominationActionsParams`: Additional parameters for nomination actions.
    NominationActions(i32, String, NominationActionsParams),

    /// Endpoint to retrieve committees involved in a specific nomination.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The nomination number.
    /// - `NominationCommitteesParams`: Additional parameters for nomination committees.
    NominationCommittees(i32, String, NominationCommitteesParams),

    /// Endpoint to list hearings related to a specific nomination.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The nomination number.
    /// - `NominationHearingsParams`: Additional parameters for nomination hearings.
    NominationHearings(i32, String, NominationHearingsParams),

    // ================================
    // Treaty Endpoints
    // ================================

    /// Endpoint to list treaties based on provided parameters.
    TreatyList(TreatyListParams),

    /// Endpoint to retrieve treaties by a specific congress number.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `TreatyByCongressParams`: Additional parameters for filtering treaties.
    TreatyByCongress(i32, TreatyByCongressParams),

    /// Endpoint to get detailed information about a specific treaty.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The treaty number.
    /// - `TreatyDetailsParams`: Additional parameters for treaty details.
    TreatyDetails(i32, String, TreatyDetailsParams),

    /// Endpoint to retrieve partitioned information about a specific treaty.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The treaty number.
    /// - `String`: The treaty suffix.
    /// - `TreatyPartitionedParams`: Additional parameters for partitioned treaties.
    TreatyPartitioned(i32, String, String, TreatyPartitionedParams),

    /// Endpoint to retrieve committees associated with a specific treaty.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The treaty number.
    /// - `TreatyCommitteesParams`: Additional parameters for treaty committees.
    TreatyCommittees(i32, String, TreatyCommitteesParams),

    /// Endpoint to fetch actions taken on a specific treaty.
    ///
    /// # Parameters
    ///
    /// - `i32`: The congress number.
    /// - `String`: The treaty number.
    /// - `TreatyActionsParams`: Additional parameters for treaty actions.
    TreatyActions(i32, String, TreatyActionsParams),
}

/// Trait defining constructors for creating new instances of `Endpoints`.
///
/// This trait provides a standardized way to instantiate each variant of the
/// `Endpoints` enum by supplying the necessary parameters.
pub trait NewEndpoint {
    fn new_manual(manual: String) -> Self;

    // ================================
    // Bill Constructors
    // ================================

    /// Constructs a `BillList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing bills.
    fn new_bill_list(params: BillListParams) -> Self;

    /// Constructs a `BillByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering bills by congress.
    fn new_bill_by_congress(congress: i32, params: BillByCongressParams) -> Self;

    /// Constructs a `BillByType` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `params`: Parameters for filtering bills by type.
    fn new_bill_by_type(congress: i32, bill_type: BillType, params: BillByTypeParams) -> Self;

    /// Constructs a `BillDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `bill_number`: The bill number.
    /// - `params`: Parameters for bill details.
    fn new_bill_details(
        congress: i32,
        bill_type: BillType,
        bill_number: i32,
        params: BillDetailsParams,
    ) -> Self;

    /// Constructs a `BillActions` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `bill_number`: The bill number.
    /// - `params`: Parameters for bill actions.
    fn new_bill_actions(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillActionsParams,
    ) -> Self;

    /// Constructs a `BillAmendments` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `bill_number`: The bill number.
    /// - `params`: Parameters for bill amendments.
    fn new_bill_amendments(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillAmendmentsParams,
    ) -> Self;

    /// Constructs a `BillCommittees` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `bill_number`: The bill number.
    /// - `params`: Parameters for bill committees.
    fn new_bill_committees(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillCommitteesParams,
    ) -> Self;

    /// Constructs a `BillCosponsors` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `bill_number`: The bill number.
    /// - `params`: Parameters for bill cosponsors.
    fn new_bill_cosponsors(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillCosponsorsParams,
    ) -> Self;

    // ================================
    // Law Constructors
    // ================================

    /// Constructs a `LawList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing laws.
    fn new_law_type(congress: i32, law_type: LawType, params: LawParams) -> Self;

    /// Constructs a `LawByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering laws by congress.
    fn new_law_by_congress(congress: i32, params: LawParams) -> Self;

    /// Constructs a `LawDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for law details.
    fn new_law_details(congress: i32, law_type: LawType, law_number: i32, params: LawParams) -> Self;

    // ================================
    // Amendment Constructors
    // ================================

    /// Constructs an `AmendmentList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing amendments.
    fn new_amendment_list(params: AmendmentListParams) -> Self;

    /// Constructs an `AmendmentByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering amendments by congress.
    fn new_amendment_by_congress(congress: i32, params: AmendmentByCongressParams) -> Self;

    /// Constructs an `AmendmentByType` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `params`: Parameters for filtering amendments by type.
    fn new_amendment_by_type(
        congress: i32,
        amendment_type: AmendmentType,
        params: AmendmentByTypeParams,
    ) -> Self;

    /// Constructs an `AmendmentDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `amendment_number`: The amendment number.
    /// - `params`: Parameters for amendment details.
    fn new_amendment_details(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentDetailsParams,
    ) -> Self;

    /// Constructs an `AmendmentActions` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `amendment_number`: The amendment number.
    /// - `params`: Parameters for amendment actions.
    fn new_amendment_actions(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentActionsParams,
    ) -> Self;

    /// Constructs an `AmendmentCosponsors` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `amendment_number`: The amendment number.
    /// - `params`: Parameters for amendment cosponsors.
    fn new_amendment_cosponsors(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentCosponsorsParams,
    ) -> Self;

    /// Constructs an `AmendmentAmendments` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `amendment_number`: The amendment number.
    /// - `params`: Parameters for amendment amendments.
    fn new_amendment_amendments(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentAmendmentsParams,
    ) -> Self;

    /// Constructs an `AmendmentText` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `amendment_type`: The type of amendment.
    /// - `amendment_number`: The amendment number.
    /// - `params`: Parameters for amendment text.
    fn new_amendment_text(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentTextParams,
    ) -> Self;

    // ================================
    // Summaries Constructors
    // ================================

    /// Constructs a `SummariesList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing summaries.
    fn new_summaries_list(params: SummariesListParams) -> Self;

    /// Constructs a `SummariesByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering summaries by congress.
    fn new_summaries_by_congress(congress: i32, params: SummariesByCongressParams) -> Self;

    /// Constructs a `SummariesByType` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `bill_type`: The type of bill.
    /// - `params`: Parameters for filtering summaries by type.
    fn new_summaries_by_type(
        congress: i32,
        bill_type: BillType,
        params: SummariesByTypeParams,
    ) -> Self;

    // ================================
    // Congress Constructors
    // ================================

    /// Constructs a `CongressList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing congress sessions.
    fn new_congress_list(params: CongressListParams) -> Self;

    /// Constructs a `CongressDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for congress details.
    fn new_congress_details(congress: i32, params: CongressDetailsParams) -> Self;

    /// Constructs a `CongressCurrent` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for current congress information.
    fn new_congress_current(params: CongressCurrentParams) -> Self;

    // ================================
    // Member Constructors
    // ================================

    /// Constructs a `MemberList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing members.
    fn new_member_list(params: MemberListParams) -> Self;

    /// Constructs a `MemberByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering members by congress.
    fn new_member_by_congress(congress: i32, params: MemberByCongressParams) -> Self;

    /// Constructs a `MemberByState` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `state_code`: The state code (e.g., "CA" for California).
    /// - `params`: Parameters for filtering members by state.
    fn new_member_by_state(state_code: String, params: MemberByStateParams) -> Self;

    /// Constructs a `MemberByCongressStateDistrict` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `state_code`: The state code.
    /// - `district`: The district number.
    /// - `params`: Parameters for filtering members by congress, state, and district.
    fn new_member_by_congress_state_district(
        congress: i32,
        state_code: String,
        district: i32,
        params: MemberByCongressStateDistrictParams,
    ) -> Self;

    /// Constructs a `MemberDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `bio_guide_id`: The bio guide ID of the member.
    /// - `params`: Parameters for member details.
    fn new_member_details(bio_guide_id: String, params: MemberDetailsParams) -> Self;

    /// Constructs a `SponsorshipList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `bio_guide_id`: The bio guide ID of the member.
    /// - `params`: Parameters for sponsorships.
    fn new_sponsorship_list(bio_guide_id: String, params: SponsorshipListParams) -> Self;

    /// Constructs a `CosponsorshipList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `bio_guide_id`: The bio guide ID of the member.
    /// - `params`: Parameters for cosponsorships.
    fn new_cosponsorship_list(bio_guide_id: String, params: CosponsorshipListParams) -> Self;

    // ================================
    // Committee Constructors
    // ================================

    /// Constructs a `CommitteeList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing committees.
    fn new_committee_list(params: CommitteeListParams) -> Self;

    /// Constructs a `CommitteeByChamber` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type (House or Senate).
    /// - `params`: Parameters for filtering committees by chamber.
    fn new_committee_by_chamber(
        chamber: ChamberType,
        params: CommitteeByChamberParams,
    ) -> Self;

    /// Constructs a `CommitteeByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering committees by congress.
    fn new_committee_by_congress(congress: i32, params: CommitteeByCongressParams) -> Self;

    /// Constructs a `CommitteeByCongressChamber` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `chamber`: The chamber type.
    /// - `params`: Parameters for filtering committees by congress and chamber.
    fn new_committee_by_congress_chamber(
        congress: i32,
        chamber: ChamberType,
        params: CommitteeByCongressChamberParams,
    ) -> Self;

    /// Constructs a `CommitteeDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee details.
    fn new_committee_details(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeDetailsParams,
    ) -> Self;

    /// Constructs a `CommitteeBills` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee bills.
    fn new_committee_bills(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeBillsParams,
    ) -> Self;

    /// Constructs a `CommitteeReports` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee reports.
    fn new_committee_reports(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeReportsParams,
    ) -> Self;

    /// Constructs a `CommitteeNominations` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee nominations.
    fn new_committee_nominations(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeNominationsParams,
    ) -> Self;

    /// Constructs a `CommitteeHouseCommunication` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee house communications.
    fn new_committee_house_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeHouseCommunicationParams,
    ) -> Self;

    /// Constructs a `CommitteeSenateCommunication` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `chamber`: The chamber type.
    /// - `committee_code`: The committee code.
    /// - `params`: Parameters for committee senate communications.
    fn new_committee_senate_communication(
        chamber: ChamberType,
        committee_code: String,
        params: CommitteeSenateCommunicationParams,
    ) -> Self;

    // ================================
    // Nomination Constructors
    // ================================

    /// Constructs a `NominationList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing nominations.
    fn new_nomination_list(params: NominationListParams) -> Self;

    /// Constructs a `NominationByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering nominations by congress.
    fn new_nomination_by_congress(
        congress: i32,
        params: NominationByCongressParams,
    ) -> Self;

    /// Constructs a `NominationDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `nomination_number`: The nomination number.
    /// - `params`: Parameters for nomination details.
    fn new_nomination_details(
        congress: i32,
        nomination_number: String,
        params: NominationDetailsParams,
    ) -> Self;

    /// Constructs a `Nominees` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `nomination_number`: The nomination number.
    /// - `ordinal`: The ordinal number.
    /// - `params`: Parameters for nominees.
    fn new_nominees(
        congress: i32,
        nomination_number: String,
        ordinal: i32,
        params: NomineesParams,
    ) -> Self;

    /// Constructs a `NominationActions` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `nomination_number`: The nomination number.
    /// - `params`: Parameters for nomination actions.
    fn new_nomination_actions(
        congress: i32,
        nomination_number: String,
        params: NominationActionsParams,
    ) -> Self;

    /// Constructs a `NominationCommittees` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `nomination_number`: The nomination number.
    /// - `params`: Parameters for nomination committees.
    fn new_nomination_committees(
        congress: i32,
        nomination_number: String,
        params: NominationCommitteesParams,
    ) -> Self;

    /// Constructs a `NominationHearings` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `nomination_number`: The nomination number.
    /// - `params`: Parameters for nomination hearings.
    fn new_nomination_hearings(
        congress: i32,
        nomination_number: String,
        params: NominationHearingsParams,
    ) -> Self;

    // ================================
    // Treaty Constructors
    // ================================

    /// Constructs a `TreatyList` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `params`: Parameters for listing treaties.
    fn new_treaty_list(params: TreatyListParams) -> Self;

    /// Constructs a `TreatyByCongress` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `params`: Parameters for filtering treaties by congress.
    fn new_treaty_by_congress(congress: i32, params: TreatyByCongressParams) -> Self;

    /// Constructs a `TreatyDetails` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `treaty_number`: The treaty number.
    /// - `params`: Parameters for treaty details.
    fn new_treaty_details(
        congress: i32,
        treaty_number: String,
        params: TreatyDetailsParams,
    ) -> Self;

    /// Constructs a `TreatyPartitioned` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `treaty_number`: The treaty number.
    /// - `treaty_suffix`: The treaty suffix.
    /// - `params`: Parameters for partitioned treaties.
    fn new_treaty_partitioned(
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        params: TreatyPartitionedParams,
    ) -> Self;

    /// Constructs a `TreatyCommittees` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `treaty_number`: The treaty number.
    /// - `params`: Parameters for treaty committees.
    fn new_treaty_committees(
        congress: i32,
        treaty_number: String,
        params: TreatyCommitteesParams,
    ) -> Self;

    /// Constructs a `TreatyActions` endpoint variant.
    ///
    /// # Parameters
    ///
    /// - `congress`: The congress number.
    /// - `treaty_number`: The treaty number.
    /// - `params`: Parameters for treaty actions.
    fn new_treaty_actions(
        congress: i32,
        treaty_number: String,
        params: TreatyActionsParams,
    ) -> Self;
}

/// Implementation of the `NewEndpoint` trait for the `Endpoints` enum.
impl NewEndpoint for Endpoints {
    fn new_manual(manual: String) -> Self {
        Endpoints::Manual(manual)
    }

    // ================================
    // Bill Endpoints
    // ================================

    fn new_bill_list(params: BillListParams) -> Self {
        Endpoints::BillList(params)
    }

    fn new_bill_by_congress(congress: i32, params: BillByCongressParams) -> Self {
        Endpoints::BillByCongress(congress, params)
    }

    fn new_bill_by_type(congress: i32, bill_type: BillType, params: BillByTypeParams) -> Self {
        Endpoints::BillByType(congress, bill_type, params)
    }

    fn new_bill_details(
        congress: i32,
        bill_type: BillType,
        bill_number: i32,
        params: BillDetailsParams,
    ) -> Self {
        Endpoints::BillDetails(congress, bill_type, bill_number, params)
    }

    fn new_bill_actions(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillActionsParams,
    ) -> Self {
        Endpoints::BillActions(congress, bill_type, bill_number, params)
    }

    fn new_bill_amendments(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillAmendmentsParams,
    ) -> Self {
        Endpoints::BillAmendments(congress, bill_type, bill_number, params)
    }

    fn new_bill_committees(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillCommitteesParams,
    ) -> Self {
        Endpoints::BillCommittees(congress, bill_type, bill_number, params)
    }

    fn new_bill_cosponsors(
        congress: i32,
        bill_type: BillType,
        bill_number: String,
        params: BillCosponsorsParams,
    ) -> Self {
        Endpoints::BillCosponsors(congress, bill_type, bill_number, params)
    }

    // ================================
    // Law Endpoints
    // ================================

    fn new_law_type(congress: i32, law_type: LawType, params: LawParams) -> Self {
        Endpoints::LawByType(congress, law_type, params)
    }

    fn new_law_by_congress(congress: i32, params: LawParams) -> Self {
        Endpoints::LawByCongress(congress, params)
    }

    fn new_law_details(congress: i32, law_type: LawType, law_number: i32, params: LawParams) -> Self {
        Endpoints::LawDetails(congress, law_type, law_number, params)
    }

    // ================================
    // Amendment Endpoints
    // ================================

    fn new_amendment_list(params: AmendmentListParams) -> Self {
        Endpoints::AmendmentList(params)
    }

    fn new_amendment_by_congress(congress: i32, params: AmendmentByCongressParams) -> Self {
        Endpoints::AmendmentByCongress(congress, params)
    }

    fn new_amendment_by_type(
        congress: i32,
        amendment_type: AmendmentType,
        params: AmendmentByTypeParams,
    ) -> Self {
        Endpoints::AmendmentByType(congress, amendment_type, params)
    }

    fn new_amendment_details(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentDetailsParams,
    ) -> Self {
        Endpoints::AmendmentDetails(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_actions(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentActionsParams,
    ) -> Self {
        Endpoints::AmendmentActions(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_cosponsors(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentCosponsorsParams,
    ) -> Self {
        Endpoints::AmendmentCosponsors(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_amendments(
        congress: i32,
        amendment_type: AmendmentType,
        amendment_number: String,
        params: AmendmentAmendmentsParams,
    ) -> Self {
        Endpoints::AmendmentAmendments(congress, amendment_type, amendment_number, params)
    }

    fn new_amendment_text(
        congress: i32,
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

    fn new_summaries_by_congress(congress: i32, params: SummariesByCongressParams) -> Self {
        Endpoints::SummariesByCongress(congress, params)
    }

    fn new_summaries_by_type(
        congress: i32,
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

    fn new_congress_details(congress: i32, params: CongressDetailsParams) -> Self {
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

    fn new_member_by_congress(congress: i32, params: MemberByCongressParams) -> Self {
        Endpoints::MemberByCongress(congress, params)
    }

    fn new_member_by_state(state_code: String, params: MemberByStateParams) -> Self {
        Endpoints::MemberByState(state_code, params)
    }

    fn new_member_by_congress_state_district(
        congress: i32,
        state_code: String,
        district: i32,
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

    fn new_committee_by_congress(congress: i32, params: CommitteeByCongressParams) -> Self {
        Endpoints::CommitteeByCongress(congress, params)
    }

    fn new_committee_by_congress_chamber(
        congress: i32,
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
        congress: i32,
        params: NominationByCongressParams,
    ) -> Self {
        Endpoints::NominationByCongress(congress, params)
    }

    fn new_nomination_details(
        congress: i32,
        nomination_number: String,
        params: NominationDetailsParams,
    ) -> Self {
        Endpoints::NominationDetails(congress, nomination_number, params)
    }

    fn new_nominees(
        congress: i32,
        nomination_number: String,
        ordinal: i32,
        params: NomineesParams,
    ) -> Self {
        Endpoints::Nominees(congress, nomination_number, ordinal, params)
    }

    fn new_nomination_actions(
        congress: i32,
        nomination_number: String,
        params: NominationActionsParams,
    ) -> Self {
        Endpoints::NominationActions(congress, nomination_number, params)
    }

    fn new_nomination_committees(
        congress: i32,
        nomination_number: String,
        params: NominationCommitteesParams,
    ) -> Self {
        Endpoints::NominationCommittees(congress, nomination_number, params)
    }

    fn new_nomination_hearings(
        congress: i32,
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

    fn new_treaty_by_congress(congress: i32, params: TreatyByCongressParams) -> Self {
        Endpoints::TreatyByCongress(congress, params)
    }

    fn new_treaty_details(
        congress: i32,
        treaty_number: String,
        params: TreatyDetailsParams,
    ) -> Self {
        Endpoints::TreatyDetails(congress, treaty_number, params)
    }

    fn new_treaty_partitioned(
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        params: TreatyPartitionedParams,
    ) -> Self {
        Endpoints::TreatyPartitioned(congress, treaty_number, treaty_suffix, params)
    }

    fn new_treaty_committees(
        congress: i32,
        treaty_number: String,
        params: TreatyCommitteesParams,
    ) -> Self {
        Endpoints::TreatyCommittees(congress, treaty_number, params)
    }

    fn new_treaty_actions(
        congress: i32,
        treaty_number: String,
        params: TreatyActionsParams,
    ) -> Self {
        Endpoints::TreatyActions(congress, treaty_number, params)
    }
}

