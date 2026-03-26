use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::{BillListParams, BillDetailsParams, MemberListParams};
use cdg_api::response_models::{BillsResponse, BillDetailsResponse, MembersResponse};
use cdg_api::CongressApiClient;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?;

    // --- 1. Fetch recent bills ---
    println!("=== 5 Most Recently Updated Bills ===\n");

    let bill_params = BillListParams::default()
        .format(FormatType::Json)
        .limit(5);

    let response: BillsResponse = client.fetch(Endpoints::BillList(bill_params))?;

    for bill in &response.bills {
        let number = bill.number.as_deref().unwrap_or("?");
        let kind = bill.bill_type.as_deref().unwrap_or("?");
        let title = bill.title.as_deref().unwrap_or("(no title)");
        let chamber = bill.origin_chamber.as_deref().unwrap_or("?");
        let updated = bill.update_date.as_deref().unwrap_or("?");

        println!("  {kind} {number} ({chamber})");
        println!("    {title}");
        println!("    updated: {updated}\n");
    }

    // --- 2. Fetch details for a specific bill (HR 148, 118th Congress) ---
    println!("=== Bill Detail: HR 148 (118th Congress) ===\n");

    let detail_params = BillDetailsParams::default()
        .format(FormatType::Json);

    let detail: BillDetailsResponse =
        client.fetch(Endpoints::BillDetails(118, BillType::Hr, 148, detail_params))?;

    let b = &detail.bill;
    println!("  Title:   {}", b.title.as_deref().unwrap_or("?"));
    println!("  Sponsor: {}", b.sponsors
        .as_ref()
        .and_then(|s| s.first())
        .and_then(|s| s.full_name.as_deref())
        .unwrap_or("?"));
    println!("  Status:  {}", b.latest_action
        .as_ref()
        .and_then(|a| a.text.as_deref())
        .unwrap_or("?"));
    println!();

    // --- 3. Fetch current members of Congress ---
    println!("=== 10 Current Members of Congress ===\n");

    let member_params = MemberListParams::default()
        .format(FormatType::Json)
        .limit(10)
        .current_member(true);

    let members: MembersResponse = client.fetch(Endpoints::MemberList(member_params))?;

    for m in &members.members {
        let name = m.name.as_deref().unwrap_or("?");
        let party = m.party_name.as_deref().unwrap_or("?");
        let state = m.state.as_deref().unwrap_or("?");
        let district = m.district
            .map(|d| format!(", district {d}"))
            .unwrap_or_default();

        println!("  {name} ({party}) - {state}{district}");
    }

    Ok(())
}
