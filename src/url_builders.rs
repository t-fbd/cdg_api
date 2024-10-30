use std::fmt::Display;
use crate::param_models::*;
use crate::endpoints::Endpoints;

/// Trait representing Parameters for API Endpoints.
///
/// This trait is implemented by structs representing parameters for
/// various API endpoints. It provides a common interface for interacting
/// with the parameters and converting them to query parameters.
pub trait ApiParams {
    /// Converts the parameters to a query string.
    ///
    /// # Returns
    ///
    /// A `String` representing the query parameters for the API endpoint.
    fn to_query_string(&self) -> String;
}

/// Display implementation for `ApiParams`.
///
/// This implementation allows `ApiParams` to be displayed as a query string when
/// using the `format!` macro or similar.
impl Display for dyn ApiParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.to_query_string())
    }
}

// ================================
// Bill Params to Query String
// ================================

/// Implementation of `ApiParams` for `BillListParams`.
///
/// Converts `BillListParams` into a query string suitable for the `BillList` endpoint.
impl ApiParams for BillListParams {
    /// Converts the `BillListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing bills.
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

/// Implementation of `ApiParams` for `BillByCongressParams`.
///
/// Converts `BillByCongressParams` into a query string suitable for the `BillByCongress` endpoint.
impl ApiParams for BillByCongressParams {
    /// Converts the `BillByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving bills by congress.
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

/// Implementation of `ApiParams` for `BillByTypeParams`.
///
/// Converts `BillByTypeParams` into a query string suitable for the `BillByType` endpoint.
impl ApiParams for BillByTypeParams {
    /// Converts the `BillByTypeParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving bills by type.
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

/// Implementation of `ApiParams` for `BillDetailsParams`.
///
/// Converts `BillDetailsParams` into a query string suitable for the `BillDetails` endpoint.
impl ApiParams for BillDetailsParams {
    /// Converts the `BillDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for bill details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `BillActionsParams`.
///
/// Converts `BillActionsParams` into a query string suitable for the `BillActions` endpoint.
impl ApiParams for BillActionsParams {
    /// Converts the `BillActionsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for bill actions.
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

/// Implementation of `ApiParams` for `BillAmendmentsParams`.
///
/// Converts `BillAmendmentsParams` into a query string suitable for the `BillAmendments` endpoint.
impl ApiParams for BillAmendmentsParams {
    /// Converts the `BillAmendmentsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for bill amendments.
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

/// Implementation of `ApiParams` for `BillCommitteesParams`.
///
/// Converts `BillCommitteesParams` into a query string suitable for the `BillCommittees` endpoint.
impl ApiParams for BillCommitteesParams {
    /// Converts the `BillCommitteesParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for bill committees.
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

/// Implementation of `ApiParams` for `BillCosponsorsParams`.
///
/// Converts `BillCosponsorsParams` into a query string suitable for the `BillCosponsors` endpoint.
impl ApiParams for BillCosponsorsParams {
    /// Converts the `BillCosponsorsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for bill cosponsors.
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

/// Implementation of `ApiParams` for `AmendmentListParams`.
///
/// Converts `AmendmentListParams` into a query string suitable for the `AmendmentList` endpoint.
impl ApiParams for AmendmentListParams {
    /// Converts the `AmendmentListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing amendments.
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

/// Implementation of `ApiParams` for `AmendmentByCongressParams`.
///
/// Converts `AmendmentByCongressParams` into a query string suitable for the `AmendmentByCongress` endpoint.
impl ApiParams for AmendmentByCongressParams {
    /// Converts the `AmendmentByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving amendments by congress.
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

/// Implementation of `ApiParams` for `AmendmentByTypeParams`.
///
/// Converts `AmendmentByTypeParams` into a query string suitable for the `AmendmentByType` endpoint.
impl ApiParams for AmendmentByTypeParams {
    /// Converts the `AmendmentByTypeParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving amendments by type.
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

/// Implementation of `ApiParams` for `AmendmentDetailsParams`.
///
/// Converts `AmendmentDetailsParams` into a query string suitable for the `AmendmentDetails` endpoint.
impl ApiParams for AmendmentDetailsParams {
    /// Converts the `AmendmentDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for amendment details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `AmendmentActionsParams`.
///
/// Converts `AmendmentActionsParams` into a query string suitable for the `AmendmentActions` endpoint.
impl ApiParams for AmendmentActionsParams {
    /// Converts the `AmendmentActionsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for amendment actions.
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

/// Implementation of `ApiParams` for `AmendmentCosponsorsParams`.
///
/// Converts `AmendmentCosponsorsParams` into a query string suitable for the `AmendmentCosponsors` endpoint.
impl ApiParams for AmendmentCosponsorsParams {
    /// Converts the `AmendmentCosponsorsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for amendment cosponsors.
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

/// Implementation of `ApiParams` for `AmendmentAmendmentsParams`.
///
/// Converts `AmendmentAmendmentsParams` into a query string suitable for the `AmendmentAmendments` endpoint.
impl ApiParams for AmendmentAmendmentsParams {
    /// Converts the `AmendmentAmendmentsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for amendment amendments.
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

/// Implementation of `ApiParams` for `AmendmentTextParams`.
///
/// Converts `AmendmentTextParams` into a query string suitable for the `AmendmentText` endpoint.
impl ApiParams for AmendmentTextParams {
    /// Converts the `AmendmentTextParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for amendment text.
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

/// Implementation of `ApiParams` for `LawListParams`.
///
/// Converts `LawListParams` into a query string suitable for the `LawList` endpoint.
impl ApiParams for LawParams {
    /// Converts the `LawListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing laws.
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

/// Implementation of `ApiParams` for `MemberListParams`.
///
/// Converts `MemberListParams` into a query string suitable for the `MemberList` endpoint.
impl ApiParams for MemberListParams {
    /// Converts the `MemberListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing members.
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

/// Implementation of `ApiParams` for `MemberByCongressParams`.
///
/// Converts `MemberByCongressParams` into a query string suitable for the `MemberByCongress` endpoint.
impl ApiParams for MemberByCongressParams {
    /// Converts the `MemberByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving members by congress.
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

/// Implementation of `ApiParams` for `MemberByStateParams`.
///
/// Converts `MemberByStateParams` into a query string suitable for the `MemberByState` endpoint.
impl ApiParams for MemberByStateParams {
    /// Converts the `MemberByStateParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving members by state.
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

/// Implementation of `ApiParams` for `MemberByCongressStateDistrictParams`.
///
/// Converts `MemberByCongressStateDistrictParams` into a query string suitable for the `MemberByCongressStateDistrict` endpoint.
impl ApiParams for MemberByCongressStateDistrictParams {
    /// Converts the `MemberByCongressStateDistrictParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving members by congress, state, and district.
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

/// Implementation of `ApiParams` for `MemberDetailsParams`.
///
/// Converts `MemberDetailsParams` into a query string suitable for the `MemberDetails` endpoint.
impl ApiParams for MemberDetailsParams {
    /// Converts the `MemberDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for member details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `SponsorshipListParams`.
///
/// Converts `SponsorshipListParams` into a query string suitable for the `SponsorshipList` endpoint.
impl ApiParams for SponsorshipListParams {
    /// Converts the `SponsorshipListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for sponsorship lists.
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

/// Implementation of `ApiParams` for `CosponsorshipListParams`.
///
/// Converts `CosponsorshipListParams` into a query string suitable for the `CosponsorshipList` endpoint.
impl ApiParams for CosponsorshipListParams {
    /// Converts the `CosponsorshipListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for cosponsorship lists.
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

/// Implementation of `ApiParams` for `CommitteeListParams`.
///
/// Converts `CommitteeListParams` into a query string suitable for the `CommitteeList` endpoint.
impl ApiParams for CommitteeListParams {
    /// Converts the `CommitteeListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing committees.
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

/// Implementation of `ApiParams` for `CommitteeByChamberParams`.
///
/// Converts `CommitteeByChamberParams` into a query string suitable for the `CommitteeByChamber` endpoint.
impl ApiParams for CommitteeByChamberParams {
    /// Converts the `CommitteeByChamberParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving committees by chamber.
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

/// Implementation of `ApiParams` for `CommitteeByCongressParams`.
///
/// Converts `CommitteeByCongressParams` into a query string suitable for the `CommitteeByCongress` endpoint.
impl ApiParams for CommitteeByCongressParams {
    /// Converts the `CommitteeByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving committees by congress.
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

/// Implementation of `ApiParams` for `CommitteeByCongressChamberParams`.
///
/// Converts `CommitteeByCongressChamberParams` into a query string suitable for the `CommitteeByCongressChamber` endpoint.
impl ApiParams for CommitteeByCongressChamberParams {
    /// Converts the `CommitteeByCongressChamberParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving committees by congress and chamber.
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

        query_params.push("chamber=senate".to_string()); // Example: Assuming chamber is Senate
        // Modify the above line as needed based on the actual ChamberType.

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of `ApiParams` for `CommitteeDetailsParams`.
///
/// Converts `CommitteeDetailsParams` into a query string suitable for the `CommitteeDetails` endpoint.
impl ApiParams for CommitteeDetailsParams {
    /// Converts the `CommitteeDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for committee details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format.to_query_param()
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `CommitteeBillsParams`.
///
/// Converts `CommitteeBillsParams` into a query string suitable for the `CommitteeBills` endpoint.
impl ApiParams for CommitteeBillsParams {
    /// Converts the `CommitteeBillsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for committee bills.
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

/// Implementation of `ApiParams` for `CommitteeReportsParams`.
///
/// Converts `CommitteeReportsParams` into a query string suitable for the `CommitteeReports` endpoint.
impl ApiParams for CommitteeReportsParams {
    /// Converts the `CommitteeReportsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for committee reports.
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

/// Implementation of `ApiParams` for `CommitteeNominationsParams`.
///
/// Converts `CommitteeNominationsParams` into a query string suitable for the `CommitteeNominations` endpoint.
impl ApiParams for CommitteeNominationsParams {
    /// Converts the `CommitteeNominationsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for committee nominations.
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

/// Implementation of `ApiParams` for `CommitteeHouseCommunicationParams`.
///
/// Converts `CommitteeHouseCommunicationParams` into a query string suitable for the `CommitteeHouseCommunication` endpoint.
impl ApiParams for CommitteeHouseCommunicationParams {
    /// Converts the `CommitteeHouseCommunicationParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for committee house communications.
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

/// Implementation of `ApiParams` for `CommitteeSenateCommunicationParams`.
///
/// Converts `CommitteeSenateCommunicationParams` into a query string suitable for the `CommitteeSenateCommunication` endpoint.
impl ApiParams for CommitteeSenateCommunicationParams {
    /// Converts the `CommitteeSenateCommunicationParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for committee senate communications.
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

/// Implementation of `ApiParams` for `NominationListParams`.
///
/// Converts `NominationListParams` into a query string suitable for the `NominationList` endpoint.
impl ApiParams for NominationListParams {
    /// Converts the `NominationListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing nominations.
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

/// Implementation of `ApiParams` for `NominationByCongressParams`.
///
/// Converts `NominationByCongressParams` into a query string suitable for the `NominationByCongress` endpoint.
impl ApiParams for NominationByCongressParams {
    /// Converts the `NominationByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving nominations by congress.
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

/// Implementation of `ApiParams` for `NominationDetailsParams`.
///
/// Converts `NominationDetailsParams` into a query string suitable for the `NominationDetails` endpoint.
impl ApiParams for NominationDetailsParams {
    /// Converts the `NominationDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for nomination details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `NomineesParams`.
///
/// Converts `NomineesParams` into a query string suitable for the `Nominees` endpoint.
impl ApiParams for NomineesParams {
    /// Converts the `NomineesParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for nominees.
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

/// Implementation of `ApiParams` for `NominationActionsParams`.
///
/// Converts `NominationActionsParams` into a query string suitable for the `NominationActions` endpoint.
impl ApiParams for NominationActionsParams {
    /// Converts the `NominationActionsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for nomination actions.
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

/// Implementation of `ApiParams` for `NominationCommitteesParams`.
///
/// Converts `NominationCommitteesParams` into a query string suitable for the `NominationCommittees` endpoint.
impl ApiParams for NominationCommitteesParams {
    /// Converts the `NominationCommitteesParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for nomination committees.
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

/// Implementation of `ApiParams` for `NominationHearingsParams`.
///
/// Converts `NominationHearingsParams` into a query string suitable for the `NominationHearings` endpoint.
impl ApiParams for NominationHearingsParams {
    /// Converts the `NominationHearingsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for nomination hearings.
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

/// Implementation of `ApiParams` for `TreatyListParams`.
///
/// Converts `TreatyListParams` into a query string suitable for the `TreatyList` endpoint.
impl ApiParams for TreatyListParams {
    /// Converts the `TreatyListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing treaties.
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

/// Implementation of `ApiParams` for `TreatyByCongressParams`.
///
/// Converts `TreatyByCongressParams` into a query string suitable for the `TreatyByCongress` endpoint.
impl ApiParams for TreatyByCongressParams {
    /// Converts the `TreatyByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for retrieving treaties by congress.
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

/// Implementation of `ApiParams` for `TreatyDetailsParams`.
///
/// Converts `TreatyDetailsParams` into a query string suitable for the `TreatyDetails` endpoint.
impl ApiParams for TreatyDetailsParams {
    /// Converts the `TreatyDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for treaty details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format.to_query_param()
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `TreatyPartitionedParams`.
///
/// Converts `TreatyPartitionedParams` into a query string suitable for the `TreatyPartitioned` endpoint.
impl ApiParams for TreatyPartitionedParams {
    /// Converts the `TreatyPartitionedParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameter for partitioned treaties.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `TreatyCommitteesParams`.
///
/// Converts `TreatyCommitteesParams` into a query string suitable for the `TreatyCommittees` endpoint.
impl ApiParams for TreatyCommitteesParams {
    /// Converts the `TreatyCommitteesParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for treaty committees.
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

/// Implementation of `ApiParams` for `TreatyActionsParams`.
///
/// Converts `TreatyActionsParams` into a query string suitable for the `TreatyActions` endpoint.
impl ApiParams for TreatyActionsParams {
    /// Converts the `TreatyActionsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for treaty actions.
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

/// Implementation of `ApiParams` for `SummariesListParams`.
///
/// Converts `SummariesListParams` into a query string suitable for the `SummariesList` endpoint.
impl ApiParams for SummariesListParams {
    /// Converts the `SummariesListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for summaries lists.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of `ApiParams` for `SummariesByCongressParams`.
///
/// Converts `SummariesByCongressParams` into a query string suitable for the `SummariesByCongress`
impl ApiParams for SummariesByCongressParams {
    /// Converts the `SummariesByCongressParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for summaries by congress.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of `ApiParams` for `SummariesByCongressChamberParams`.
///
/// Converts `SummariesByCongressChamberParams` into a query string suitable for the
impl ApiParams for SummariesByTypeParams {
    /// Converts the `SummariesByCongressChamberParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for summaries by congress and chamber.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of `ApiParams` for `CongressListParams`.
///
/// Converts `CongressListParams` into a query string suitable for the `CongressList` endpoint.
impl ApiParams for CongressListParams {
    /// Converts the `CongressListParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for listing congresses.
    fn to_query_string(&self) -> String {
        let mut query_params = vec![];

        if let Some(format) = &self.format {
            query_params.push(format.to_query_param());
        }

        "?".to_string() + &query_params.join("&")
    }
}

/// Implementation of `ApiParams` for `CongressDetailsParams`.
///
/// Converts `CongressDetailsParams` into a query string suitable for the `CongressDetails`
/// endpoint.
impl ApiParams for CongressDetailsParams {
    /// Converts the `CongressDetailsParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for congress details.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}

/// Implementation of `ApiParams` for `CongressCurrentParams`.
///
/// Converts `CongressCurrentParams` into a query string suitable for the `CongressCurrent`
/// endpoint.
impl ApiParams for CongressCurrentParams {
    /// Converts the `CongressCurrentParams` into a query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the query parameters for the current congress.
    fn to_query_string(&self) -> String {
        if let Some(format) = &self.format {
            format!("?{}", format.to_query_param())
        } else {
            "".to_string()
        }
    }
}


/// called by the api client to generate the complete URL for the request.
pub fn generate_url(endpoint: Endpoints, api_key: &str) -> String {
    format!("{}{}&api_key={}", crate::BASE_URL, endpoint, api_key)
}

/// Implementation of the `Display` trait for the `Endpoints` enum.
///
/// This, in conjunction with the `Display` implementation for the
/// `ApiParam` enums, allows for easy conversion of `Endpoints` to
/// a URL string.
impl std::fmt::Display for Endpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // NOTE: A '?' is appended to the params string via the `Display`
        // implementation for the `ApiParam` enums.
        match self {
            Endpoints::Manual(endpoint) => write!(f, "{}", endpoint),
            // ================================
            // Bill Endpoints
            // ================================
            Endpoints::BillList(params) => write!(f, "bill/?{}", params.to_query_string()),
            Endpoints::BillByCongress(congress, params) => {
                write!(f, "bill/{}{}", congress, params.to_query_string())
            }
            Endpoints::BillByType(congress, bill_type, params) => {
                write!(f, "bill/{}/{}{}", congress, bill_type.to_string(), params.to_query_string())
            }
            Endpoints::BillDetails(congress, bill_type, bill_number, params) => {
                write!(f, "bill/{}/{}/{}{}", congress, bill_type.to_string(), bill_number, params.to_query_string())
            }
            Endpoints::BillActions(congress, bill_type, bill_number, params) => {
                write!(f, "bill/{}/{}/{}/actions{}", congress, bill_type.to_string(), bill_number, params.to_query_string())
            }

            // ================================
            // Law Endpoints
            // ================================
            Endpoints::LawByType(congress, law_type, params) => {
                write!(f, "law/{}/{}{}", congress, law_type.to_string(), params.to_query_string())
            }
            Endpoints::LawByCongress(congress, params) => {
                write!(f, "law/{}{}", congress, params.to_query_string())
            }
            Endpoints::LawDetails(congress, law_type, law_number, params) => {
                write!(f, "law/{}/{}/{}{}", congress, law_type.to_string(), law_number, params.to_query_string())
            }

            // ================================
            // Amendment Endpoints
            // ================================
            Endpoints::AmendmentList(params) => write!(f, "amendment{}", params.to_query_string()),
            Endpoints::AmendmentByCongress(congress, params) => {
                write!(f, "amendment/{}{}", congress, params.to_query_string())
            }
            Endpoints::AmendmentByType(congress, amendment_type, params) => {
                write!(f, "amendment/{}/{}{}", congress, amendment_type.to_string(), params.to_query_string())
            }
            Endpoints::AmendmentDetails(congress, amendment_type, amendment_number, params) => {
                write!(f, "amendment/{}/{}/{}{}", congress, amendment_type.to_string(), amendment_number, params.to_query_string())
            }
            Endpoints::AmendmentActions(congress, amendment_type, amendment_number, params) => {
                write!(f, "amendment/{}/{}/{}/actions{}", congress, amendment_type.to_string(), amendment_number, params.to_query_string())
            }
            Endpoints::AmendmentCosponsors(congress, amendment_type, amendment_number, params) => {
                write!(f, "amendment/{}/{}/{}/cosponsors{}", congress, amendment_type.to_string(), amendment_number, params.to_query_string())
            }
            Endpoints::AmendmentAmendments(congress, amendment_type, amendment_number, params) => {
                write!(f, "amendment/{}/{}/{}/amendments{}", congress, amendment_type.to_string(), amendment_number, params.to_query_string())
            }
            Endpoints::AmendmentText(congress, amendment_type, amendment_number, params) => {
                write!(f, "amendment/{}/{}/{}/text{}", congress, amendment_type.to_string(), amendment_number, params.to_query_string())
            }

            // ================================
            // Summaries Endpoints
            // ================================
            Endpoints::SummariesList(params) => write!(f, "summary{}", params.to_query_string()),
            Endpoints::SummariesByCongress(congress, params) => {
                write!(f, "summary/{}{}", congress, params.to_query_string())
            }
            Endpoints::SummariesByType(congress, bill_type, params) => {
                write!(f, "summary/{}/{}{}", congress, bill_type.to_string(), params.to_query_string())
            }

            // ================================
            // Congress Endpoints
            // ================================
            Endpoints::CongressList(params) => write!(f, "congress{}", params.to_query_string()),
            Endpoints::CongressDetails(congress, params) => {
                write!(f, "congress/{}{}", congress, params.to_query_string())
            }
            Endpoints::CongressCurrent(params) => write!(f, "congress/current{}", params.to_query_string()),

            // ================================
            // Member Endpoints
            // ================================
            Endpoints::MemberList(params) => write!(f, "member{}", params.to_query_string()),
            Endpoints::MemberByCongress(congress, params) => {
                write!(f, "member/{}{}", congress, params.to_query_string())
            }
            Endpoints::MemberByState(state_code, params) => {
                write!(f, "member/{}{}", state_code, params.to_query_string())
            }
            Endpoints::MemberByCongressStateDistrict(congress, state_code, district, params) => {
                write!(f, "member/{}/{}/{}{}", congress, state_code, district, params.to_query_string())
            }
            Endpoints::MemberDetails(bio_guide_id, params) => {
                write!(f, "member/{}/details{}", bio_guide_id, params.to_query_string())
            }
            Endpoints::SponsorshipList(bio_guide_id, params) => {
                write!(f, "member/{}/sponsorship{}", bio_guide_id, params.to_query_string())
            }
            Endpoints::CosponsorshipList(bio_guide_id, params) => {
                write!(f, "member/{}/cosponsorship{}", bio_guide_id, params.to_query_string())
            }

            // ================================
            // Committee Endpoints
            // ================================
            Endpoints::CommitteeList(params) => write!(f, "committee?{}", params.to_query_string()),
            Endpoints::CommitteeByChamber(chamber, params) => {
                write!(f, "committee/chamber/{}{}", chamber.to_string(), params.to_query_string())
            }
            Endpoints::CommitteeByCongress(congress, params) => {
                write!(f, "committee/{}{}", congress, params.to_query_string())
            }
            Endpoints::CommitteeByCongressChamber(congress, chamber, params) => {
                write!(f, "committee/{}/{}{}", congress, chamber.to_string(), params.to_query_string())
            }
            Endpoints::CommitteeDetails(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}{}", chamber.to_string(), committee_code, params.to_query_string())
            }
            Endpoints::CommitteeBills(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}/bills{}", chamber.to_string(), committee_code, params.to_query_string())
            }
            Endpoints::CommitteeReports(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}/reports{}", chamber.to_string(), committee_code, params.to_query_string())
            }
            Endpoints::CommitteeNominations(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}/nominations{}", chamber.to_string(), committee_code, params.to_query_string())
            }
            Endpoints::CommitteeHouseCommunication(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}/house-communication{}", chamber.to_string(), committee_code, params.to_query_string())
            }
            Endpoints::CommitteeSenateCommunication(chamber, committee_code, params) => {
                write!(f, "committee/{}/{}/senate-communication{}", chamber.to_string(), committee_code, params.to_query_string())
            }

            // ================================
            // Nomination Endpoints
            // ================================
            Endpoints::NominationList(params) => write!(f, "nomination{}", params.to_query_string()),
            Endpoints::NominationByCongress(congress, params) => {
                write!(f, "nomination/{}{}", congress, params.to_query_string())
            }
            Endpoints::NominationDetails(congress, nomination_number, params) => {
                write!(f, "nomination/{}/{}{}", congress, nomination_number, params.to_query_string())
            }
            Endpoints::Nominees(congress, nomination_number, ordinal, params) => {
                write!(f, "nomination/{}/{}/{}{}", congress, nomination_number, ordinal, params.to_query_string())
            }
            Endpoints::NominationActions(congress, nomination_number, params) => {
                write!(f, "nomination/{}/{}/actions{}", congress, nomination_number, params.to_query_string())
            }
            Endpoints::NominationCommittees(congress, nomination_number, params) => {
                write!(f, "nomination/{}/{}/committees{}", congress, nomination_number, params.to_query_string())
            }
            Endpoints::NominationHearings(congress, nomination_number, params) => {
                write!(f, "nomination/{}/{}/hearings{}", congress, nomination_number, params.to_query_string())
            }

            // ================================
            // Treaty Endpoints
            // ================================
            Endpoints::TreatyList(params) => write!(f, "treaty{}", params.to_query_string()),
            Endpoints::TreatyByCongress(congress, params) => {
                write!(f, "treaty/{}{}", congress, params.to_query_string())
            }
            Endpoints::TreatyDetails(congress, treaty_number, params) => {
                write!(f, "treaty/{}/{}{}", congress, treaty_number, params.to_query_string())
            }
            Endpoints::TreatyPartitioned(congress, treaty_number, treaty_suffix, params) => {
                write!(f, "treaty/{}/{}/{}{}", congress, treaty_number, treaty_suffix, params.to_query_string())
            }
            Endpoints::TreatyCommittees(congress, treaty_number, params) => {
                write!(f, "treaty/{}/{}/committees{}", congress, treaty_number, params.to_query_string())
            }
            Endpoints::TreatyActions(congress, treaty_number, params) => {
                write!(f, "treaty/{}/{}/actions{}", congress, treaty_number, params.to_query_string())
            }
            _ => write!(f, ""),
        }
    }
}
