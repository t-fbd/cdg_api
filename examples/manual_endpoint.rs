use cdg_api::cdg_types::FormatType;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::GenericParams;
use cdg_api::response_models::{
    parse_response, serialize_response, DailyCongressionalRecordResponse, GenericResponse,
};
use cdg_api::{unwrap_option, unwrap_option_string, unwrap_option_u32, CongressApiClient};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Assume there's an endpoint that doesn't have a specific response model
    let endpoint = Endpoints::new_generic(
        "daily-congressional-record".to_string(),
        GenericParams::default().format(FormatType::Json).limit(250),
    );

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response into a generic JSON value if specific parsing fails
    match parse_response::<DailyCongressionalRecordResponse, GenericResponse>(&response) {
        Ok(json) => {
            json.daily_congressional_record.iter().for_each(|records| {
                let record = records.clone();
                println!("Date: {}", unwrap_option_string(record.issue_date));
                println!("Update Date: {}", unwrap_option_string(record.update_date));
                println!("Volume: {}", unwrap_option_u32(record.volume_number));
                println!("Issue: {}", unwrap_option_string(record.issue_number));
                println!("Sess. #: {}", unwrap_option_u32(record.session_number));
                println!("Congress: {}", unwrap_option_u32(record.congress));
                println!("URL: {}", unwrap_option_string(record.url));
                println!();
                println!("Full Issue: {:#?}", unwrap_option(record.full_issue));
            });
        }
        Err(e) => {
            println!("Failed to parse response: {}", e);
            // Serialize the response as a generic JSON value, pretty-printed = true
            println!("{}", serialize_response(&response, true)?);
        }
    }

    Ok(())
}
