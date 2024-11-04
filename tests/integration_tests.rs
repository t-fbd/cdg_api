use cdg_api::{
    cdg_types::{
        AmendmentType, BillType, ChamberType, CommitteeReportType, CommunicationType, FormatType,
        LawType, SortType,
    },
    endpoints::{Endpoints, NewEndpoint},
    param_models::*,
    response_models::{
        BillsResponse, CommitteeDetailsResponse, CommitteeReportsResponse, CongressDetailsResponse,
        HearingsResponse, LawDetailsResponse, LawsResponse, MembersResponse,
        NominationDetailsResponse, NominationsResponse, RelatedBillsResponse, TreatiesResponse,
        TreatyDetailsResponse,
    },
    CongressApiClient,
};

#[test]
fn test_fetch_members() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching members.
    let params = MemberListParams::default()
        .format(FormatType::Json)
        .limit(10)
        .current_member(true);

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_member_list(params);

    // Fetch the data using the client.
    let response: MembersResponse = client.fetch(endpoint).expect("Failed to fetch members");

    // Assert that the members list is not empty.
    assert!(
        !response.members.is_empty(),
        "Members list should not be empty"
    );

    // Optionally, print out the fetched members for manual verification.
    for member in response.members {
        println!(
            "{}, {}, {}",
            member.name.unwrap_or("".to_string()),
            member.party_name.unwrap_or("".to_string()),
            member.state.unwrap_or("".to_string())
        );
    }
}

#[test]
fn test_fetch_bills() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching bills.
    let params = BillListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_bill_list(params);

    // Fetch the data using the client.
    let response: BillsResponse = client.fetch(endpoint).expect("Failed to fetch bills");

    // Assert that the bills list is not empty.
    assert!(!response.bills.is_empty(), "Bills list should not be empty");

    // Optionally, print out the fetched bills for manual verification.
    for bill in response.bills {
        println!(
            "{}, {}, {}",
            bill.title.unwrap_or("".to_string()),
            bill.bill_type.unwrap_or_default(),
            bill.number.unwrap_or_default()
        );
    }
}

#[test]
fn test_fetch_law_by_congress_and_type() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching laws.
    let params = LawParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..LawParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_law_type(117, LawType::Priv, params);

    // Fetch the data using the client.
    let response: LawsResponse = client.fetch(endpoint).expect("Failed to fetch laws");

    // Assert that the laws list is not empty.
    assert!(!response.bills.is_empty(), "Laws list should not be empty");

    // Optionally, print out the fetched laws for manual verification.
    for law in response.bills {
        println!(
            "{}, {}, {}",
            law.title.unwrap_or("".to_string()),
            law.bill_type.unwrap_or_default(),
            law.number.unwrap_or_default()
        );
    }
}

#[test]
fn test_fetch_amendments() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching amendments.
    let params = AmendmentListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..AmendmentListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_amendment_list(params);

    // Fetch the data using the client.
    let response: cdg_api::response_models::AmendmentsResponse =
        client.fetch(endpoint).expect("Failed to fetch amendments");

    // Assert that the amendments list is not empty.
    assert!(
        !response.amendments.is_empty(),
        "Amendments list should not be empty"
    );

    // Optionally, print out the fetched amendments for manual verification.
    for amendment in response.amendments {
        println!(
            "{:#?}, {}, {}",
            amendment.latest_action,
            amendment.amendment_type.unwrap_or_default(),
            amendment.number.unwrap_or("".to_string())
        );
    }
}

#[test]
fn test_fetch_committee_details() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching committee details.
    let params = CommitteeDetailsParams::default();

    // Use a known committee code for testing, e.g., "SSAF" for Senate Armed Services Committee
    let chamber = ChamberType::House;
    let committee_code = String::from("hspw00");

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_committee_details(chamber, committee_code.clone(), params);

    // Fetch the data using the client.
    let response: CommitteeDetailsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch committee details");

    // Assert that the committee code matches.
    assert_eq!(
        response.committee.system_code.clone().unwrap(),
        committee_code,
        "Committee code should match"
    );

    // Optionally, print out the fetched committee details for manual verification.
    println!("Committee: {:#?}", response.committee);
}

#[test]
fn test_fetch_nomination_details() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching nomination details.
    let params = NominationDetailsParams::default();

    // Use a known nomination number for testing, e.g., "PN1" in Congress 117
    let congress = 117;
    let nomination_number = String::from("2467");

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_nomination_details(congress, nomination_number.clone(), params);

    // Fetch the data using the client.
    let response: NominationDetailsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch nomination details");

    // Optionally, print out the fetched nomination for manual verification.
    println!("Nomination: {:#?}", response.nomination);
}

#[test]
fn test_fetch_treaty_details() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching treaty details.
    let params = TreatyDetailsParams::default();

    // Use a known treaty number for testing, e.g., "1" in Congress 112
    let congress = 112;
    let treaty_number = 1;

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_treaty_details(congress, treaty_number, params);

    // Fetch the data using the client.
    let response: TreatyDetailsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch treaty details");

    // Assert that the treaty number matches.
    assert_eq!(
        response.treaty.number.unwrap_or(0),
        treaty_number,
        "Treaty number should match"
    );

    // Optionally, print out the fetched treaty details for manual verification.
    println!(
        "Treaty Topic: {}",
        response.treaty.topic.unwrap_or("".to_string())
    );
}

#[test]
fn test_fetch_hearings() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching hearings.
    let params = HearingListParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..HearingListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_hearing_list(params);

    // Fetch the data using the client.
    let response: HearingsResponse = client.fetch(endpoint).expect("Failed to fetch hearings");

    // Assert that the hearings list is not empty.
    assert!(
        !response.hearings.is_empty(),
        "Hearings list should not be empty"
    );

    // Optionally, print out the fetched hearings for manual verification.
    for hearing in response.hearings {
        println!(
            "num: {}, jacket_num: {}",
            hearing.number.unwrap_or_default(),
            hearing.jacket_number.unwrap_or(0)
        );
    }
}

#[test]
fn test_fetch_current_congress() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching current congress.
    let params = CongressCurrentParams::default();

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_congress_current(params);

    // Fetch the data using the client.
    let response: CongressDetailsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch current congress");

    // Assert that the congress number is greater than zero.
    assert!(
        response.congress.number.unwrap_or(0) > 0,
        "Congress number should be greater than zero"
    );

    // Optionally, print out the fetched congress details for manual verification.
    println!(
        "Current Congress Number: {}",
        response.congress.number.unwrap_or(0)
    );
}

#[test]
fn test_fetch_committee_reports() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching committee reports.
    let params = CommitteeReportListParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..CommitteeReportListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_committee_report_list(params);

    // Fetch the data using the client.
    let response: CommitteeReportsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch committee reports");

    // Assert that the reports list is not empty.
    assert!(
        !response.reports.is_empty(),
        "Committee reports list should not be empty"
    );

    // Optionally, print out the fetched reports for manual verification.
    for report in response.reports {
        println!(
            "Report Number: {}, url: {}",
            report.number.unwrap_or(0),
            report.url.unwrap_or("".to_string())
        );
    }
}

#[test]
fn test_fetch_related_bills() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching related bills.
    let params = BillRelatedParams::default();

    // Use known values for testing
    let congress = 117;
    let bill_type = BillType::Hr;
    let bill_number = 1;

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_bill_related(congress, bill_type, bill_number, params);

    // Fetch the data using the client.
    let response: RelatedBillsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch related bills");

    // Assert that the related bills list is not empty.
    assert!(
        !response.related_bills.is_empty(),
        "Related bills list should not be empty"
    );

    // Optionally, print out the fetched related bills for manual verification.
    for related_bill in response.related_bills {
        println!(
            "Related Bill Number: {}",
            related_bill.number.unwrap_or(0000)
        );
        println!(
            "Related Bill Title: {}",
            related_bill.title.unwrap_or("".to_string())
        );
        println!(
            "Related Bill Type: {}",
            related_bill.bill_type.unwrap_or_default()
        );
        println!(
            "Relationship Info: {:#?}",
            related_bill.relationship_details
        );
    }
}

#[test]
fn test_fetch_house_communications() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching house communications.
    let params = CommunicationParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..CommunicationParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_house_communication_list(params);

    // Fetch the data using the client.
    let response: cdg_api::response_models::HouseCommunicationsResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch house communications");

    // Assert that the communications list is not empty.
    assert!(
        !response.house_communications.is_empty(),
        "Communications list should not be empty"
    );

    // Optionally, print out the fetched communications for manual verification.
    for communication in response.house_communications {
        println!(
            "Communication Number: {}",
            communication.number.unwrap_or(0000)
        );
        println!(
            "Chamber: {}",
            communication.chamber.unwrap_or("".to_string())
        );
        println!(
            "Congress Number: {}",
            communication.congress_number.unwrap_or(0000)
        );
        println!(
            "Communication Type: {}, {}",
            communication
                .communication_type
                .clone()
                .unwrap_or_default()
                .name
                .unwrap_or("".to_string()),
            communication
                .communication_type
                .unwrap_or_default()
                .code
                .unwrap_or("".to_string())
        );
        println!("url: {}", communication.url.unwrap_or("".to_string()));
    }
}

#[test]
fn test_fetch_daily_congressional_records() {
    // Initialize the CongressApiClient.
    let client = CongressApiClient::new(None).expect("Failed to create CongressApiClient");

    // Define the parameters for fetching daily congressional records.
    let params = DailyCongressionalRecordListParams {
        format: Some(FormatType::Json),
        limit: Some(5),
        ..DailyCongressionalRecordListParams::default()
    };

    // Create the endpoint using the NewEndpoint trait.
    let endpoint = Endpoints::new_daily_congressional_record_list(params);

    // Fetch the data using the client.
    let response: cdg_api::response_models::DailyCongressionalRecordResponse = client
        .fetch(endpoint)
        .expect("Failed to fetch daily congressional records");

    // Assert that the records list is not empty.
    assert!(
        !response.daily_congressional_record.is_empty(),
        "Daily congressional records list should not be empty"
    );

    // Optionally, print out the fetched records for manual verification.
    for record in response.daily_congressional_record {
        println!(
            "vol #: {}, iss #: {}",
            record.volume_number.unwrap_or(0),
            record.issue_number.unwrap_or("".to_string())
        );
    }
}
