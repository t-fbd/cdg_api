//! [`param_chains`] module
//!
//! Simply holds a macro that allows each Param model to call `build` methods for user ease

use crate::cdg_types::*;
use crate::param_models::*;

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
