use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::endpoints::{Endpoints, NewEndpoint};
use cdg_api::param_models::BillDetailsParams;
use cdg_api::response_models::{parse_response, BillDetailsResponse, GenericResponse};
use cdg_api::{unwrap_option, unwrap_option_string, CongressApiClient};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use API key from environment

    // Define parameters for bill details
    let params = BillDetailsParams::default().format(FormatType::Json);

    // Specify the bill to fetch (e.g., H.R. 1234 from the 118th Congress)
    let endpoint = Endpoints::new_bill_details(118, BillType::Hr, 1234, params);

    // Fetch the data as GenericResponse
    let response: GenericResponse = client.fetch(endpoint)?;

    // Parse the response as BillDetails
    let bill_details: BillDetailsResponse = parse_response(&response)?;
    let bill = bill_details.bill;

    println!("Bill: {}", unwrap_option_string(bill.bill_type));
    println!("Title: {}", unwrap_option_string(bill.title));
    println!("Summary: {:#?}", unwrap_option(bill.summaries));

    Ok(())
}
