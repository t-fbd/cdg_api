use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::BillListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::BillsResponse;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use default API key

    // Define parameters
    let params = BillListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..BillListParams::default()
    };

    // Create the endpoint
    let endpoint = Endpoints::BillList(params);

    // Fetch the data
    let response: BillsResponse = client.fetch(endpoint)?;

    // Process the response
    for bill in response.bills {
        println!("{}, {}, {}\n", bill.title, bill.bill_type, bill.number);
    }

    Ok(())
}
