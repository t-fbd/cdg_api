use cdg_api::{
    endpoints::{Endpoints, NewEndpoint}, 
    param_models::*,
    cdg_types::{FormatType, LawType, AmendmentType, BillType, SortType, ChamberType, CommunicationType, CommitteeReportType},
    response_models::{BillsResponse, MembersResponse, GenericResponseModel, TreatyDetailsResponse, RelatedBillsResponse},
    CongressApiClient
};

#[test]
fn test_fetch_members() {
    // Initialize the CongressApiClient.
    // It will automatically use the `CDG_API_KEY` from the environment.
    let client = match CongressApiClient::new(None) {
        Ok(c) => c,
        Err(e) => panic!("Failed to create CongressApiClient: {}", e),
    };

    // Define the parameters for fetching members.
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_member_list(params);

    // Fetch the data using the client.
    let response: MembersResponse = match client.fetch(endpoint) {
        Ok(res) => res,
        Err(e) => panic!("Failed to fetch members: {}", e),
    };

    // Assert that the members list is not empty.
    assert!(
        !response.members.is_empty(),
        "Members list should not be empty"
    );

    // Optionally, print out the fetched members for manual verification.
    for member in response.members {
        println!("{}, {}, {}", member.name, member.state, member.party_name);
    }
}

#[test]
fn test_fetch_bills() {
    // Initialize the CongressApiClient.
    // It will automatically use the `CDG_API_KEY` from the environment.
    let client = match CongressApiClient::new(None) {
        Ok(c) => c,
        Err(e) => panic!("Failed to create CongressApiClient: {}", e),
    };

    // Define the parameters for fetching bills.
    let params = BillListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_bill_list(params);

    // Fetch the data using the client.
    let response: BillsResponse = match client.fetch(endpoint) {
        Ok(res) => res,
        Err(e) => panic!("Failed to fetch bills: {}", e),
    };

    // Assert that the bills list is not empty.
    assert!(!response.bills.is_empty(), "Bills list should not be empty");

    // Optionally, print out the fetched bills for manual verification.
    for bill in response.bills {
        println!("{}, {}, {}", bill.title, bill.bill_type, bill.number);
    }
}

#[test]
fn test_fetch_law_by_congress_and_type() {
    // Initialize the CongressApiClient.
    let client = match CongressApiClient::new(None) {
        Ok(c) => c,
        Err(e) => panic!("Failed to create CongressApiClient: {}", e),
    };

    // Define the parameters for fetching laws.
    let params = cdg_api::param_models::LawParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..cdg_api::param_models::LawParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_law_type(118, LawType::Pub, params);

    // Fetch the data using the client.
    let response: cdg_api::response_models::LawsResponse = match client.fetch(endpoint) {
        Ok(res) => res,
        Err(e) => panic!("Failed to fetch laws: {}", e),
    };

    // Assert that the laws list is not empty.
    assert!(!response.bills.is_empty(), "Laws list should not be empty");

    // Optionally, print out the fetched laws for manual verification.
    for law in response.bills {
        println!("{}, {}, {}", law.title, law.bill_type, law.number);
    }
}

#[test]
fn test_fetch_amendments() {
    // Initialize the CongressApiClient.
    let client = match CongressApiClient::new(None) {
        Ok(c) => c,
        Err(e) => panic!("Failed to create CongressApiClient: {}", e),
    };

    // Define the parameters for fetching amendments.
    let params = cdg_api::param_models::AmendmentListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..cdg_api::param_models::AmendmentListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_amendment_list(params);

    // Fetch the data using the client.
    let response: cdg_api::response_models::AmendmentsResponse = match client.fetch(endpoint) {
        Ok(res) => res,
        Err(e) => panic!("Failed to fetch amendments: {}", e),
    };

    // Assert that the amendments list is not empty.
    assert!(
        !response.amendments.is_empty(),
        "Amendments list should not be empty"
    );

    // Optionally, print out the fetched amendments for manual verification.
    for amendment in response.amendments {
        println!("{:#?}, {}, {}", amendment.latest_action, amendment.amendment_type, amendment.number);
    }
}
