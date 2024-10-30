//! # `main.rs`
//! 
//! ## Available Commands
//! 
//! - `list_bills`        : List recent bills introduced in Congress.
//! - `current_congress`  : Display information about the current congress session.
//! - `list_nominations`  : List recent nominations.
//! - `list_treaties`     : List recent treaties.
//! - `member_details`    : Get detailed information about a specific member (requires additional argument: `bioguide_id`).
//! - `bill_details`      : Get detailed information about a specific bill (requires additional arguments: `congress`, `bill_type`, `bill_number`).
//! 
//! ## Usage
//! 
//! ```bash
//! cargo run -- <command> [additional arguments]
//! 
//! # Examples:
//! cargo run -- current_members
//! cargo run -- list_bills
//! cargo run -- member_details <bioguide_id>
//! cargo run -- bill_details 117 h 1150
//! ```
use std::env;
use std::error::Error;
use std::process;

use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::{
    BillListParams, BillDetailsParams, MemberDetailsParams,
    TreatyListParams, BillType,
};
use cdg_api::response_models::{
    BillsResponse, LawDetailsResponse,
    MemberDetailsResponse, NominationsResponse,
    TreatiesResponse,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Runs the main application logic.
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

    match command.as_str() {
        "list_bills" => {
            let params = BillListParams {
                format: Some(cdg_api::param_models::FormatType::Json),
                limit: Some(50),
                ..BillListParams::default()
            };
            let endpoint = Endpoints::BillList(params);
            let response: BillsResponse = client.fetch(endpoint)?;
            display_bills(&response);
        },
        "current_congress" => {
            let endpoint = Endpoints::CongressCurrent(cdg_api::param_models::CongressCurrentParams::default());
            let response: cdg_api::response_models::CongressDetailsResponse = client.fetch(endpoint)?;
            display_congress_details(&response);
        },
        "list_nominations" => {
            let params = cdg_api::param_models::NominationListParams::default();
            let endpoint = Endpoints::NominationList(params);
            let response: NominationsResponse = client.fetch(endpoint)?;
            display_nominations(&response);
        },
        "list_treaties" => {
            let params = TreatyListParams {
                format: Some(cdg_api::param_models::FormatType::Json),
                limit: Some(50),
                ..TreatyListParams::default()
            };
            let endpoint = Endpoints::TreatyList(params);
            let response: TreatiesResponse = client.fetch(endpoint)?;
            display_treaties(&response);
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
            let bill_type = BillType::from_str(&args[3]).unwrap_or(BillType::default());
            let bill_number: i32 = args[4].parse()?;
            let params = BillDetailsParams::default();
            let endpoint = Endpoints::BillDetails(congress, bill_type, bill_number, params);
            let response: LawDetailsResponse = client.fetch(endpoint)?;
            display_bill_details(&response);
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
            return Err("Invalid command.".into());
        }
    }

    Ok(())
}

/// Prints the usage instructions.
fn print_usage() {
    println!("Usage: cargo run -- <command> [additional arguments]");
    println!("\nAvailable commands:");
    println!("  current_members         - Fetch and display all current members of Congress.");
    println!("  list_bills              - List recent bills introduced in Congress.");
    println!("  current_congress        - Display information about the current congress session.");
    println!("  list_nominations        - List recent nominations.");
    println!("  list_treaties           - List recent treaties.");
    println!("  member_details <id>     - Get detailed information about a specific member (bioguide_id).");
    println!("  bill_details <congress> <type> <number> - Get detailed information about a specific bill.");
}


/// Displays the list of bills in a formatted manner.
fn display_bills(response: &BillsResponse) {
    println!("Recent Bills:");
    for bill in &response.bills {
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
    println!("Total Bills: {}", response.bills.len());
}

/// Displays the congress details in a formatted manner.
fn display_congress_details(response: &cdg_api::response_models::CongressDetailsResponse) {
    let congress = &response.congress;
    println!("Congress Details:");
    println!("----------------------------------------");
    println!("Name      : {}", congress.name);
    println!("Number    : {}", congress.number);
    println!("Start Year: {}", congress.start_year);
    println!("End Year  : {}", congress.end_year);
    println!("Sessions:");
    for session in &congress.sessions {
        println!("  - Session {}: {} to {}", session.number, session.start_date, session.end_date.as_deref().unwrap_or("Ongoing"));
    }
    if let Some(url) = &congress.url {
        println!("URL       : {}", url);
    }
    println!("----------------------------------------");
}
///
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
    }
    println!("----------------------------------------");
    println!("Total Treaties: {}", response.treaties.len());
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
        println!("Attribution        : {}", depiction.attribution);
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
