use cdg_api::{unwrap_option_string, CongressApiClient};
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::BillListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::GenericResponse;

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
    let response: GenericResponse = client.fetch(endpoint)?;

    let bills = {
        if let Some(bills) = response.bills {
            bills
        } else {
            return Err("No bills found".into());
        }
    };

    // Process the response
    for bill in bills {
        println!("{}, {}, {}\n", 
            unwrap_option_string(bill.title),
            unwrap_option_string(bill.bill_type),
            unwrap_option_string(bill.number)
        );
    }

    Ok(())
}
