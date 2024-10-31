use cdg_api::cdg_types::FormatType;
use cdg_api::param_models::GenericParams;
use cdg_api::CongressApiClient;
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::response_models::{DailyCongressionalRecordResponse, GenericResponse};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Assume there's an endpoint that doesn't have a specific response model
    let endpoint = Endpoints::new_generic(
        "daily-congressional-record".to_string(),
        GenericParams::new(
            FormatType::Json.into(),
            None,
            250.into(),
            None,
            None,
            None,
            None,
            None,
            None,
           None,
            None,
            None,
        )
    );

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response into a generic JSON value if specific parsing fails
    match response.parse_generic_response::<DailyCongressionalRecordResponse>() {
        Ok(json) => {
            println!("Received unexpected data:");
            println!("{}", serde_json::to_string_pretty(&json)?);
        },
        Err(e) => {
            println!("Failed to parse response: {}", e);
        }
    }

    Ok(())
}
