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
    AmendmentListParams, BillDetailsParams, BillListParams, CommitteeListParams, LawParams,
    MemberDetailsParams, MemberListParams, NominationListParams, TreatyListParams,
};
use cdg_api::cdg_types::*;
use cdg_api::response_models::{
    AmendmentsResponse, BillsResponse, CommitteesResponse, CongressDetailsResponse,
    LawDetailsResponse, LawsResponse, MemberDetailsResponse, MembersResponse, NominationsResponse,
    PrimaryResponse, TreatiesResponse,
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
            let bill_amount = args[2].parse::<u32>().unwrap_or(10);
            println!("Searching for {} bills...", bill_amount);
            let limit = 250;
            let all_bills = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::BillList(BillListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                    )
                },
                |response: &BillsResponse| response.bills.clone(),
                bill_amount as usize,
                limit,
            )?;
            display_bills(&all_bills);
        }
        "current_congress" => {
            let endpoint = Endpoints::CongressCurrent(cdg_api::param_models::CongressCurrentParams::default());
            let response: CongressDetailsResponse = client.fetch(endpoint)?;
            display_congress_details(&response);
        }
        "list_nominations" => {
            let limit = 250;
            let all_nominations: Vec<cdg_api::response_models::NominationItem> = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::NominationList(NominationListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                    )
                },
                |response: &NominationsResponse| response.nominations.clone(),
                results_max,
                limit,
            )
            .unwrap_or_default();
            display_nominations(&NominationsResponse {
                nominations: all_nominations,
                unknown: None,
            });
        }
        "list_treaties" => {
            let limit = 250;
            let all_treaties = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::TreatyList(TreatyListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                    )
                },
                |response: &TreatiesResponse| response.treaties.clone(),
                results_max,
                limit,
            )?;
            display_treaties(&TreatiesResponse {
                treaties: all_treaties,
                unknown: None,
            });
        }
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
        }
        "bill_details" => {
            if args.len() < 5 {
                eprintln!("Usage: cargo run -- bill_details <congress> <bill_type> <bill_number>");
                return Err("Missing arguments for bill_details command.".into());
            }
            let congress: u32 = args[2].parse()?;
            let bill_type = BillType::from_str(&args[3]).unwrap_or_default();
            let bill_number: u32 = args[4].parse()?;
            let params = BillDetailsParams::default();
            let endpoint = Endpoints::BillDetails(congress, bill_type, bill_number, params);
            let response: LawDetailsResponse = client.fetch(endpoint)?;
            display_bill_details(&response);
        }
        "current_members" => {
            let limit = 250;
            let all_members = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::MemberList(MemberListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                        .current_member(true)
                    )
                },
                |response: &MembersResponse| response.members.clone(),
                results_max,
                limit,
            )?;
            display_members(&MembersResponse {
                members: all_members,
                unknown: None,
            });
        }
        "list_committees" => {
            let limit = 250;
            let all_committees = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::CommitteeList(CommitteeListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                    )
                },
                |response: &CommitteesResponse| response.committees.clone(),
                results_max,
                limit,
            )?;
            display_committees(&CommitteesResponse {
                committees: all_committees,
            });
        }
        "list_laws" => {
            let limit = 250;
            let congress = 118; // Current Congress
            let all_laws = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::LawByCongress(
                        congress,
                        LawParams::default()
                            .format(FormatType::Json)
                            .limit(limit as u32)
                            .offset(offset as u32)
                    )
                },
                |response: &LawsResponse| response.bills.clone(),
                results_max,
                limit,
            )?;
            display_laws(&LawsResponse {
                bills: all_laws,
                unknown: None,
            });
        }
        "list_amendments" => {
            let limit = 250;
            let all_amendments = fetch_all(
                &client,
                |offset, limit| {
                    Endpoints::AmendmentList(AmendmentListParams::default()
                        .format(FormatType::Json)
                        .limit(limit as u32)
                        .offset(offset as u32)
                    )
                },
                |response: &AmendmentsResponse| response.amendments.clone(),
                results_max,
                limit,
            )?;
            display_amendments(&AmendmentsResponse {
                amendments: all_amendments,
                unknown: None,
            });
        }
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
/// * `max` - The maximum number of items to fetch.
/// * `page_limit` - The number of items to fetch per request.
///
/// # Returns
///
/// A `Result` containing a vector of items or an error.
fn fetch_all<T, U, F, G>(
    client: &CongressApiClient,
    endpoint_fn: F,
    extract_fn: G,
    max: usize,
    page_limit: usize,
) -> Result<Vec<U>, Box<dyn Error>>
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
    println!("  list_bills {{amount}}           : List recent bills introduced in Congress.");
    println!("  current_congress                : Display information about the current congress session.");
    println!("  list_nominations                : List recent nominations.");
    println!("  list_treaties                   : List recent treaties.");
    println!("  member_details {{bioguide_id}}  : Get detailed information about a specific member.");
    println!("  bill_details                    : Get detailed information about a specific bill.");
    println!("    {{congress}}");
    println!("     {{bill_type}}");
    println!("      {{bill_number}}");
    println!("  current_members                 : Fetch and display all current members of Congress.");
    println!("  list_committees                 : List all congressional committees.");
    println!("  list_laws                       : List recently passed laws.");
    println!("  list_amendments                 : List recent amendments.");
}

/// Displays the list of members in a formatted manner.
fn display_members(response: &MembersResponse) {
    println!("Current Members of Congress:");
    for member in &response.members {
        println!("----------------------------------------");
        println!(
            "Name       : {}",
            member.name.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "bioguideId : {}",
            member.bioguide_id.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "State      : {}",
            member.state.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Party      : {}",
            member
                .party_name
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "District   : {}",
            member.district.unwrap_or(0)
        );
        let depiction = member.depiction.clone().unwrap_or_default();
        println!(
            "Image URL  : {}",
            depiction.image_url.unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Attribution: {}",
            depiction.attribution.unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Members: {}", response.members.len());
}

/// Displays the list of bills in a formatted manner.
fn display_bills(all_bills: &[cdg_api::response_models::BillSummary]) {
    println!("Recent Bills:");
    for bill in all_bills {
        println!("----------------------------------------");
        println!(
            "Bill Number   : {}",
            bill.number.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Title         : {}",
            bill.title.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!("Congress      : {}", bill.congress.unwrap_or(0));
        println!(
            "Origin Chamber: {}",
            bill.origin_chamber
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        if let Some(action) = &bill.latest_action {
            println!(
                "Latest Action : {} on {}",
                action.text.clone().unwrap_or_else(|| "N/A".to_string()),
                action.action_date.clone().unwrap_or_else(|| "N/A".to_string())
            );
        } else {
            println!("Latest Action : N/A");
        }
        println!(
            "URL           : {}",
            bill.url.clone().unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Bills: {}", all_bills.len());
}

/// Displays the congress details in a formatted manner.
fn display_congress_details(response: &CongressDetailsResponse) {
    let congress = &response.congress;
    println!("Congress Details:");
    println!("----------------------------------------");
    println!(
        "Name       : {}",
        congress.name.clone().unwrap_or_else(|| "N/A".to_string())
    );
    println!("Number     : {}", congress.number.unwrap_or(0));
    println!(
        "Start Year : {}",
        congress
            .start_year
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "End Year   : {}",
        congress
            .end_year
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!("Sessions:");
    if let Some(sessions) = &congress.sessions {
        for session in sessions {
            println!(
                "  - Session {}: {} to {}",
                session.number.unwrap_or(0),
                session
                    .start_date
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string()),
                session
                    .end_date
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            );
        }
    }
    if let Some(url) = &congress.url {
        println!("URL        : {}", url.clone());
    } else {
        println!("URL        : N/A");
    }
    println!("----------------------------------------");
}

/// Displays the list of nominations in a formatted manner.
fn display_nominations(response: &NominationsResponse) {
    println!("Recent Nominations:");
    for nomination in &response.nominations {
        println!("----------------------------------------");
        println!("Number          : {}", nomination.number.clone().unwrap_or_else(|| 0000));
        println!(
            "Citation        : {}",
            nomination
                .citation
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Description     : {}",
            nomination
                .description
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Received Date   : {}",
            nomination
                .received_date
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        if let Some(nomination_type) = &nomination.nomination_type {
            println!(
                "Nomination Type : is_civilian: {}, is_military: {}",
                nomination_type.is_civilian.unwrap_or(false),
                nomination_type.is_military.unwrap_or(false)
            );
        } else {
            println!("Nomination Type : N/A");
        }
        if let Some(latest_action) = &nomination.latest_action {
            println!(
                "Latest Action   : {}",
                latest_action
                    .text
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            );
        } else {
            println!("Latest Action   : N/A");
        }
        println!(
            "Organization    : {}",
            nomination
                .organization
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "URL             : {}",
            nomination.url.clone().unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Nominations: {}", response.nominations.len());
}

/// Displays the list of treaties in a formatted manner.
fn display_treaties(response: &TreatiesResponse) {
    println!("Recent Treaties:");
    for treaty in &response.treaties {
        println!("----------------------------------------");
        println!("Number              : {}", treaty.number.clone().unwrap_or_else(|| 0000));
        println!(
            "Suffix              : {}",
            treaty.suffix.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Topic               : {}",
            treaty.topic.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Transmitted Date    : {}",
            treaty
                .transmitted_date
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Resolution Text     : {}",
            treaty
                .resolution_text
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Congress Received   : {}",
            treaty.congress_received.clone().unwrap_or_else(|| 0000)
        );
        println!(
            "Congress Considered : {}",
            treaty.congress_considered.clone().unwrap_or_else(|| 0000)
        );
        let parts = treaty.parts.clone().unwrap_or_default();
        println!("Parts Count         : {}", parts.count.unwrap_or(0));
        if let Some(urls) = &parts.urls {
            println!("Parts URLs          : {}", urls.join(", "));
        } else {
            println!("Parts URLs          : N/A");
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
        println!(
            "Name     : {}",
            committee.name.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Chamber  : {}",
            committee.chamber.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Type     : {}",
            committee
                .committee_type_code
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "URL      : {}",
            committee.url.clone().unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Committees: {}", response.committees.len());
}

/// Displays the list of laws in a formatted manner.
fn display_laws(response: &LawsResponse) {
    println!("Recent Laws:");
    for law in &response.bills {
        println!("----------------------------------------");
        println!(
            "Law Number    : {}",
            law.number.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Title         : {}",
            law.title.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!("Congress      : {}", law.congress.unwrap_or(0));
        println!(
            "Origin Chamber: {}",
            law.origin_chamber
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        if let Some(action) = &law.latest_action {
            println!(
                "Latest Action : {} on {}",
                action.text.clone().unwrap_or_else(|| "N/A".to_string()),
                action.action_date.clone().unwrap_or_else(|| "N/A".to_string())
            );
        } else {
            println!("Latest Action : N/A");
        }
        println!(
            "URL           : {}",
            law.url.clone().unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Laws: {}", response.bills.len());
}

/// Displays the list of amendments in a formatted manner.
fn display_amendments(response: &AmendmentsResponse) {
    println!("Recent Amendments:");
    for amendment in &response.amendments {
        println!("----------------------------------------");
        println!(
            "Amendment Number : {}",
            amendment.number.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Type             : {}",
            amendment
                .amendment_type
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!("Congress         : {}", amendment.congress.unwrap_or(0));
        println!(
            "Purpose          : {}",
            amendment
                .purpose
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Update Date      : {}",
            amendment
                .update_date
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        if let Some(action) = &amendment.latest_action {
            println!(
                "Latest Action    : {} on {}",
                action.text.clone().unwrap_or_else(|| "N/A".to_string()),
                action.action_date.clone().unwrap_or_else(|| "N/A".to_string())
            );
        } else {
            println!("Latest Action    : N/A");
        }
        println!(
            "URL              : {}",
            amendment.url.clone().unwrap_or_else(|| "N/A".to_string())
        );
    }
    println!("----------------------------------------");
    println!("Total Amendments: {}", response.amendments.len());
}

/// Displays detailed information about a specific member.
fn display_member_details(response: &MemberDetailsResponse) {
    let member = &response.member;
    println!("Member Details:");
    println!("----------------------------------------");
    println!(
        "Name               : {} {} {}",
        member.first_name.clone().unwrap_or_else(|| "".to_string()),
        member
            .middle_name
            .clone()
            .unwrap_or_else(|| "".to_string()),
        member.last_name.clone().unwrap_or_else(|| "".to_string())
    );
    println!(
        "Suffix             : {}",
        member
            .suffix_name
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Nickname           : {}",
        member
            .nick_name
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Honorific Name     : {}",
        member
            .honorific_name
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Bioguide ID        : {}",
        member.bioguide_id.clone().unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Official URL       : {}",
        member
            .official_website_url
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    if let Some(address) = &member.address_information {
        println!(
            "Office Address     : {}",
            address
                .office_address
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "City               : {}",
            address.city.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "District           : {}",
            address
                .district
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "ZIP Code           : {}",
            address
                .zip_code.unwrap_or(00000)
        );
        println!(
            "Phone Number       : {}",
            address
                .phone_number
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
    } else {
        println!("Address Information: N/A");
    }
    if let Some(depiction) = &member.depiction {
        println!(
            "Image URL          : {}",
            depiction
                .image_url
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "Attribution        : {}",
            depiction
                .attribution
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
    } else {
        println!("Depiction          : N/A");
    }
    println!("\nParty Affiliation:");
    if let Some(party_history) = &member.party_history {
        for party in party_history {
            println!("----------------------------------------");
            println!(
                "  - Party: {}",
                party.party_name.clone().unwrap_or_else(|| "N/A".to_string())
            );
            println!(
                "    Start: {}",
                party.start_year.unwrap_or(0)
            );
            println!(
                "    End  : {}",
                party.end_year.unwrap_or(0)
            );
        }
    } else {
        println!("Party: N/A");
    }
    println!("\nTerms of Service:");
    if let Some(terms) = &member.terms {
        for term in terms {
            println!("----------------------------------------");
            println!("  - Chamber: {}", term.chamber.clone().unwrap_or_else(|| "N/A".to_string()));
            println!("    Congress: {}", term.congress.unwrap_or(0));
            println!(
                "    State: {} ({})",
                term.state_name.clone().unwrap_or_else(|| "N/A".to_string()),
                term.state_code.clone().unwrap_or_else(|| "N/A".to_string())
            );
            println!(
                "    Party: {} ({})",
                term.party_name.clone().unwrap_or_else(|| "N/A".to_string()),
                term.party_code.clone().unwrap_or_else(|| "N/A".to_string())
            );
            println!(
                "    Term: {} - {}",
                term.start_year.unwrap_or(0),
                term.end_year.unwrap_or(0)
            );
        }
    } else {
        println!("Terms: N/A");
    }
    println!("\n");
    println!(
        "Sponsored Legislation: {} bills",
        member.sponsored_legislation.clone().unwrap_or_default().count.unwrap_or(0)
    );
    println!(
        "Cosponsored Legislation: {} bills",
        member.cosponsored_legislation.clone().unwrap_or_default().count.unwrap_or(0)
    );
    println!("----------------------------------------");
}

/// Displays detailed information about a specific bill.
fn display_bill_details(response: &LawDetailsResponse) {
    let bill = &response.bill;
    println!("\nBill Details:");
    println!("----------------------------------------");
    println!(
        "Number             : {}",
        bill.number.clone().unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Title              : {}",
        bill.title.clone().unwrap_or_else(|| "N/A".to_string())
    );
    println!("Congress           : {}", bill.congress.unwrap_or(0));
    println!(
        "Origin Chamber     : {}",
        bill.origin_chamber
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Last Update Date   : {}",
        bill.update_date
            .clone()
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Laws Associated    : {} laws",
        bill.related_bills.clone().unwrap_or_default().count.unwrap_or(0)
    );
    println!("Latest Action:");
    if let Some(action) = &bill.latest_action {
        println!(
            "  - Text       : {}",
            action.text.clone().unwrap_or_else(|| "N/A".to_string())
        );
        println!(
            "  - Action Date: {}",
            action
                .action_date
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        );
    } else {
        println!("  - Latest Action: N/A");
    }
    println!("----------------------------------------");

    println!("Raw JSON Response:");
    println!("{:#?}", response);
}
