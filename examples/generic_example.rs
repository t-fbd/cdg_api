use cdg_api::{unwrap_option_string, CongressApiClient};
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::BillListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::{BillsResponse, GenericResponse};

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

    // Parse the response into a specific primary response,
    // if it fails, it will just print the json response
    let bill_list: BillsResponse = match response.parse_generic_response() {
        Ok(bill_list) => bill_list,
        Err(_) => {
            println!("{}", response.serialize_generic_response(true)?);
            return Ok(());
        }
    };

    // Print the bill list
    for bill in bill_list.bills {
        println!("Bill: {}", unwrap_option_string(bill.bill_type));
        println!("Title: {}", unwrap_option_string(bill.title));
        println!("Number: {}", unwrap_option_string(bill.number));
        println!("Origin Chamber: {}", unwrap_option_string(bill.origin_chamber));
        println!("Update Date: {}", unwrap_option_string(bill.update_date));
        println!("URL: {}", unwrap_option_string(bill.url));
        println!();
    }

    Ok(())
}



