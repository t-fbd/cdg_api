use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::*;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::DailyCongressionalRecordResponse;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use default API key

    // Create the endpoint
    let endpoint = Endpoints::DailyCongressionalRecordList(
        DailyCongressionalRecordListParams::default()
            .format(FormatType::Json)
    );

    // Fetch the data
    let response: DailyCongressionalRecordResponse = client.fetch(endpoint)?;

    // Print the data
    for record in response.daily_congressional_record {
        println!("Iss. Date: {}", record.issue_date.unwrap_or("N/A".to_string()));
        println!("Iss. Number: {}", record.issue_number.unwrap_or("N/A".to_string()));
        println!("Vol. Number: {}", record.volume_number.unwrap_or(0));
        println!("Session: {}", record.session_number.unwrap_or(0));
        println!("Congress: {}", record.congress.unwrap_or(0));
        println!("URL: {}", record.url.unwrap_or("N/A".to_string()));
        println!("\n");
        println!("Full Issue: {:#?}", record.full_issue.unwrap_or_default());
        println!("\n");
        println!("=====================================");
    }

    Ok(())
}
