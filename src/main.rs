//! # main.rs
//! 
//! ## Available Commands
//! 
//! - `list_bills`        : List recent bills introduced in Congress.
//! - `current_congress`  : Display information about the current congress session.
//! - `list_nominations`  : List recent nominations.
//! - `list_treaties`     : List recent treaties.
//! - `member_details`    : Get detailed information about a specific member (requires additional argument: `bioguide_id`).
//! - `bill_details`      : Get detailed information about a specific bill (requires additional arguments: `congress`, `bill_type`, `bill_number`).
//! - `current_members`   : Fetch and display all current members of Congress.
//! - `list_committees`   : List all congressional committees.
//! - `list_laws`         : List recently passed laws.
//! - `list_amendments`   : List recent amendments.
//! 
//! ## Usage
//! 
//! ```bash
//! cargo run -- <command> [additional arguments]
//! 
//! # Examples:
//! cargo run -- list_bills {amount}
//! cargo run -- current_congress
//! cargo run -- member_details {bioguide_id}
//! cargo run -- bill_details {congress} {bill_type} {bill_number}
//! cargo run -- current_members
//! cargo run -- list_committees
//! cargo run -- list_laws
//! cargo run -- list_amendments
//! ```

use std::env;
use std::error::Error;
use std::process;

use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::{
    AmendmentListParams, BillDetailsParams, BillListParams, CommitteeListParams,
    LawParams, MemberDetailsParams, MemberListParams, NominationListParams,
    TreatyListParams, BillType,
};
use cdg_api::response_models::{
    AmendmentsResponse, BillsResponse, CommitteesResponse, CongressDetailsResponse, Depiction, LawDetailsResponse, LawsResponse, MemberDetailsResponse, MembersResponse, NominationsResponse, PrimaryResponse, TreatiesResponse, TreatyParts
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Runs the main application logic.
/// Some of these can take a considerable amount of time to fetch all data depending on the amount requested.
fn run() -> Result<(), Box<dyn Error>> {
    // Retrieve the API key from the environment variable or use default
    let api_key = env::var("CDG_API_KEY").ok();
    let client = CongressApiClient::new(api_key)?;

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Err("No command provided.".into());
    }

    // The first argument is the command
    let command = args[1].to_lowercase();
    let results_max = 1000;

    match command.as_str() {
        "list_bills" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run -- list_bills {{amount}}");
                return Err("Missing amount for list_bills command.".into());
            }
            let bill_amount = args[2].parse::<i32>().unwrap_or(10);
            println!("Searching for {} bills...", bill_amount);
            let limit = 250;
            let all_bills = fetch_all(
                &client,
                |offset, limit| Endpoints::BillList(BillListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..BillListParams::default()
                }),
                |response: &BillsResponse| response.bills.clone(),
                bill_amount as usize,
                limit,
            )?;
            display_bills(&all_bills);
        },
        "current_congress" => {
            let endpoint = Endpoints::CongressCurrent(cdg_api::param_models::CongressCurrentParams::default());
            let response: CongressDetailsResponse = client.fetch(endpoint)?;
            display_congress_details(&response);
        },
        "list_nominations" => {
            let limit = 250;
            let all_nominations = fetch_all(
                &client,
                |offset, limit| Endpoints::NominationList(NominationListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..NominationListParams::default()
                }),
                |response: &NominationsResponse| response.nominations.clone(),
                results_max,
                limit,
            )?;
            display_nominations(&NominationsResponse { nominations: all_nominations, unknown: None });
        },
        "list_treaties" => {
            let limit = 250;
            let all_treaties = fetch_all(
                &client,
                |offset, limit| Endpoints::TreatyList(TreatyListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..TreatyListParams::default()
                }),
                |response: &TreatiesResponse| response.treaties.clone(),
                results_max,
                limit,
            )?;
            display_treaties(&TreatiesResponse { treaties: all_treaties, unknown: None });
        },
        "member_details" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run -- member_details <bioguide_id>");
                return Err("Missing bioguide_id for member_details command.".into());
            }
            let bioguide_id = &args[2];
            let params = MemberDetailsParams::default();
            let endpoint = Endpoints::MemberDetails(bioguide_id.clone(), params);
            let response: MemberDetailsResponse = client.fetch(endpoint)?;
            display_member_details(&response);
        },
        "bill_details" => {
            if args.len() < 5 {
                eprintln!("Usage: cargo run -- bill_details <congress> <bill_type> <bill_number>");
                return Err("Missing arguments for bill_details command.".into());
            }
            let congress: i32 = args[2].parse()?;
            let bill_type = BillType::from_str(&args[3]).unwrap_or_default();
            let bill_number: i32 = args[4].parse()?;
            let params = BillDetailsParams::default();
            let endpoint = Endpoints::BillDetails(congress, bill_type, bill_number, params);
            let response: LawDetailsResponse = client.fetch(endpoint)?;
            display_bill_details(&response);
        },
        "current_members" => {
            let limit = 250;
            let all_members = fetch_all(
                &client,
                |offset, limit| Endpoints::MemberList(MemberListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    current_member: Some(true),
                    ..MemberListParams::default()
                }),
                |response: &MembersResponse| response.members.clone(),
                results_max,
                limit,
            )?;
            display_members(&MembersResponse { members: all_members, unknown: None });
        },
        "list_committees" => {
            let limit = 250;
            let all_committees = fetch_all(
                &client,
                |offset, limit| Endpoints::CommitteeList(CommitteeListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..CommitteeListParams::default()
                }),
                |response: &CommitteesResponse| response.committees.clone(),
                results_max,
                limit,
            )?;
            display_committees(&CommitteesResponse { committees: all_committees });
        },
        "list_laws" => {
            let limit = 250;
            let congress = 118; // Current Congress
            let all_laws = fetch_all(
                &client,
                |offset, limit| Endpoints::LawByCongress(congress, LawParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..LawParams::default()
                }),
                |response: &LawsResponse| response.bills.clone(),
                results_max,
                limit,
            )?;
            display_laws(&LawsResponse { bills: all_laws, unknown: None });
        },
        "list_amendments" => {
            let limit = 250;
            let all_amendments = fetch_all(
                &client,
                |offset, limit| Endpoints::AmendmentList(AmendmentListParams {
                    format: Some(cdg_api::param_models::FormatType::Json),
                    limit: Some(limit as i32),
                    offset: Some(offset as i32),
                    ..AmendmentListParams::default()
                }),
                |response: &AmendmentsResponse| response.amendments.clone(),
                results_max,
                limit,
            )?;
            display_amendments(&AmendmentsResponse { amendments: all_amendments, unknown: None });
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
            return Err("Invalid command.".into());
        }
    }

    Ok(())
}

/// Fetches all items from a paginated endpoint.
///
/// # Arguments
///
/// * `client` - Reference to the CongressApiClient.
/// * `endpoint_fn` - A closure that takes `offset` and `limit` and returns an `Endpoints` enum.
/// * `extract_fn` - A closure that takes a reference to the response and returns a vector of items.
/// * `limit` - The number of items to fetch per request.
///
/// # Returns
///
/// A `Result` containing a vector of items or an error.
fn fetch_all<T, U, F, G>(client: &CongressApiClient, 
                         endpoint_fn: F, 
                         extract_fn: G,
                         max: usize,
                         page_limit: usize) -> Result<Vec<U>, Box<dyn Error>>
where
    F: Fn(usize, usize) -> Endpoints,
    G: Fn(&T) -> Vec<U>,
    T: serde::de::DeserializeOwned + PrimaryResponse,
{
    let mut all_items = Vec::new();
    let mut offset = 0;

    loop {
        let endpoint = endpoint_fn(offset, page_limit);
        let response: T = client.fetch(endpoint.clone())?;
        let items = extract_fn(&response);
        let fetched_count = items.len();
        all_items.extend(items);

        if all_items.len() >= max {
            all_items.truncate(max);
            break;
        }

        if fetched_count < page_limit || all_items.len() >= max {
            break;
        }

        offset += fetched_count;
    }

    Ok(all_items)
}

/// Prints the usage instructions.
fn print_usage() {
    println!("Usage: cargo run -- <command> [additional arguments]");
    println!("\nAvailable commands:");
    println!("  list_bills              - List recent bills introduced in Congress.");
    println!("  current_congress        - Display information about the current congress session.");
    println!("  list_nominations        - List recent nominations.");
    println!("  list_treaties           - List recent treaties.");
    println!("  member_details <id>     - Get detailed information about a specific member (bioguide_id).");
    println!("  bill_details <congress> <type> <number> - Get detailed information about a specific bill.");
    println!("  current_members         - Fetch and display all current members of Congress.");
    println!("  list_committees         - List all congressional committees.");
    println!("  list_laws               - List recently passed laws.");
    println!("  list_amendments         - List recent amendments.");
}

/// Displays the list of members in a formatted manner.
fn display_members(response: &MembersResponse) {
    println!("Current Members of Congress:");
    for member in &response.members {
        println!("----------------------------------------");
        println!("Name       : {}", member.name);
        println!("State      : {}", member.state);
        println!("Party      : {}", member.party_name);
        if let Some(district) = member.district {
            println!("District   : {}", district);
        }
        let def_depiction = Depiction::default();
        let depiction = member.depiction.as_ref().unwrap_or(&def_depiction);
        println!("Image URL  : {}", depiction.image_url);
        println!("Attribution: {}", depiction.attribution.clone().unwrap_or("N/A".to_string()));
    }
    println!("----------------------------------------");
    println!("Total Members: {}", response.members.len());
}

/// Displays the list of bills in a formatted manner.
fn display_bills(all_bills: &Vec<cdg_api::response_models::BillSummary>) {
    println!("Recent Bills:");
    for bill in all_bills {
        println!("----------------------------------------");
        println!("Bill Number   : {}", bill.number);
        println!("Title         : {}", bill.title);
        println!("Congress      : {}", bill.congress);
        println!("Origin Chamber: {}", bill.origin_chamber);
        if let Some(action) = &bill.latest_action {
            println!("Latest Action : {} on {}", action.text, action.action_date);
        }
        println!("URL           : {}", bill.url);
    }
    println!("----------------------------------------");
    println!("Total Bills: {}", all_bills.len());
}

/// Displays the congress details in a formatted manner.
fn display_congress_details(response: &CongressDetailsResponse) {
    let congress = &response.congress;
    println!("Congress Details:");
    println!("----------------------------------------");
    println!("Name       : {}", congress.name);
    println!("Number     : {}", congress.number);
    println!("Start Year : {}", congress.start_year);
    println!("End Year   : {}", congress.end_year);
    println!("Sessions:");
    for session in &congress.sessions {
        println!("  - Session {}: {} to {}", session.number, session.start_date, session.end_date.as_deref().unwrap_or("Ongoing"));
    }
    if let Some(url) = &congress.url {
        println!("URL        : {}", url);
    }
    println!("----------------------------------------");
}

/// Displays the list of nominations in a formatted manner.
fn display_nominations(response: &NominationsResponse) {
    println!("Recent Nominations:");
    for nomination in &response.nominations {
        println!("----------------------------------------");
        println!("Number          : {}", nomination.number);
        println!("Citation        : {}", nomination.citation);
        println!("Description     : {}", nomination.description.clone().unwrap_or("N/A".to_string()));
        println!("Received Date   : {}", nomination.received_date);
        println!("Nomination Type : {:?}", nomination.nomination_type);
        println!("Latest Action   : {}", nomination.latest_action.text);
        println!("Organization    : {}", nomination.organization);
        println!("URL             : {}", nomination.url);
    }
    println!("----------------------------------------");
    println!("Total Nominations: {}", response.nominations.len());
}

/// Displays the list of treaties in a formatted manner.
fn display_treaties(response: &TreatiesResponse) {
    println!("Recent Treaties:");
    for treaty in &response.treaties {
        println!("----------------------------------------");
        println!("Number              : {}", treaty.number);
        if let Some(suffix) = &treaty.suffix {
            println!("Suffix              : {}", suffix);
        }
        println!("Topic               : {}", treaty.topic);
        println!("Transmitted Date    : {}", treaty.transmitted_date);
        println!("Resolution Text     : {}", treaty.resolution_text);
        println!("Congress Received   : {}", treaty.congress_received);
        println!("Congress Considered : {}", treaty.congress_considered);
        let def_treaty_parts = TreatyParts::default();
        let parts = treaty.parts.as_ref().unwrap_or(&def_treaty_parts);
        println!("Parts Count         : {}", parts.count);
        if !parts.urls.is_empty() {
            println!("Parts URLs          : {}", parts.urls.join(", "));
        }
    }
    println!("----------------------------------------");
    println!("Total Treaties: {}", response.treaties.len());
}

/// Displays the list of committees in a formatted manner.
fn display_committees(response: &CommitteesResponse) {
    println!("Congressional Committees:");
    for committee in &response.committees {
        println!("----------------------------------------");
        println!("Name     : {}", committee.name);
        println!("Chamber  : {}", committee.chamber);
        println!("Type     : {}", committee.committee_type_code);
        println!("URL      : {}", committee.url);
    }
    println!("----------------------------------------");
    println!("Total Committees: {}", response.committees.len());
}

/// Displays the list of laws in a formatted manner.
fn display_laws(response: &LawsResponse) {
    println!("Recent Laws:");
    for law in &response.bills {
        println!("----------------------------------------");
        println!("Law Number    : {}", law.number);
        println!("Title         : {}", law.title);
        println!("Congress      : {}", law.congress);
        println!("Origin Chamber: {}", law.origin_chamber);
        if let Some(action) = &law.latest_action {
            println!("Latest Action : {} on {}", action.text, action.action_date);
        }
        println!("URL           : {}", law.url);
    }
    println!("----------------------------------------");
    println!("Total Laws: {}", response.bills.len());
}

/// Displays the list of amendments in a formatted manner.
fn display_amendments(response: &AmendmentsResponse) {
    println!("Recent Amendments:");
    for amendment in &response.amendments {
        println!("----------------------------------------");
        println!("Amendment Number : {}", amendment.number);
        println!("Type             : {}", amendment.amendment_type);
        println!("Congress         : {}", amendment.congress);
        println!("Purpose          : {}", amendment.purpose.clone().unwrap_or("N/A".to_string()));
        println!("Update Date      : {}", amendment.update_date.clone().unwrap_or("N/A".to_string()));
        if let Some(action) = &amendment.latest_action {
            println!("Latest Action    : {} on {}", action.text, action.action_date);
        }
        println!("URL              : {}", amendment.url);
    }
    println!("----------------------------------------");
    println!("Total Amendments: {}", response.amendments.len());
}

/// Displays detailed information about a specific member.
fn display_member_details(response: &MemberDetailsResponse) {
    let member = &response.member;
    println!("Member Details:");
    println!("----------------------------------------");
    println!("Name               : {} {} {}", 
        member.first_name, 
        member.middle_name.as_deref().unwrap_or(""), 
        member.last_name);
    if let Some(suffix) = &member.suffix_name {
        println!("Suffix             : {}", suffix);
    }
    if let Some(nick) = &member.nick_name {
        println!("Nickname           : {}", nick);
    }
    println!("Honorific Name     : {:?}", member.honorific_name);
    if let Some(url) = &member.official_url {
        println!("Official URL       : {}", url);
    }
    if let Some(address) = &member.address_information {
        println!("Office Address     : {}", address.office_address);
        println!("City               : {}", address.city);
        println!("District           : {}", address.district);
        println!("ZIP Code           : {}", address.zip_code);
        println!("Phone Number       : {}", address.phone_number);
    }
    if let Some(depiction) = &member.depiction {
        println!("Image URL          : {}", depiction.image_url);
        println!("Attribution        : {}", depiction.attribution.clone().unwrap_or("N/A".to_string()));
    }
    println!("Terms of Service:");
    for term in &member.terms {
        println!("  - Chamber: {}", term.chamber);
        println!("    Congress: {}", term.congress);
        println!("    State: {} ({})", term.state_name, term.state_code);
        println!("    Party: {} ({})", term.party_name, term.party_code);
        println!("    Term: {} - {:?}", term.start_year, term.end_year);
    }
    println!("Sponsored Legislation: {} bills", member.sponsored_legislation.count);
    println!("Cosponsored Legislation: {} bills", member.cosponsored_legislation.count);
    println!("----------------------------------------");
}

/// Displays detailed information about a specific bill.
fn display_bill_details(response: &LawDetailsResponse) {
    let bill = &response.bill;
    println!("Bill Details:");
    println!("----------------------------------------");
    println!("Number             : {}", bill.number);
    println!("Title              : {}", bill.title);
    println!("Congress           : {}", bill.congress);
    println!("Origin Chamber     : {}", bill.origin_chamber);
    if let Some(update_date) = &bill.update_date {
        println!("Last Update Date   : {}", update_date);
    }
    println!("Laws Associated    : {} laws", bill.related_bills.count);
    println!("Latest Action:");
    if let Some(action) = &bill.latest_action {
        println!("  - Text       : {}", action.text);
        println!("  - Action Date: {}", action.action_date);
    }
    println!("----------------------------------------");
}
