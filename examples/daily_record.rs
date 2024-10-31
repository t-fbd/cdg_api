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
        DailyCongressionalRecordListParams {
            format: Some(FormatType::Json),
            ..Default::default()
        }
    );

    // Fetch the data
    let response: DailyCongressionalRecordResponse = client.fetch(endpoint)?;

    // Print the data
    println!("{:#?}", response);

    Ok(())
}
