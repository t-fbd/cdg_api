use cdg_api::cdg_types::{BillType, FormatType};
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::BillActionsParams;
use cdg_api::response_models::BillActionsResponse;
use cdg_api::CongressApiClient;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = CongressApiClient::new(None)?; // Use default API key

    // Define parameters
    let params = BillActionsParams::default()
        .format(FormatType::Json)
        .limit(10);

    // Create the endpoint
    let endpoint = Endpoints::BillActions(118, BillType::S, 4361, params);

    // Fetch the data
    let response: BillActionsResponse = client.fetch(endpoint)?;

    // Process the response
    for action in response.actions {
        println!("{:#?}\n", action);
        println!("=====================");
        println!("=====================\n");
    }

    Ok(())
}

//fn main() -> Result<(), Box<dyn Error>> {
//    let client = CongressApiClient::new(None)?; // Use default API key
//
//    // Define parameters
//    let params = BillListParams::default().format(FormatType::Json).limit(10);
//
//    // Create the endpoint
//    let endpoint = Endpoints::BillList(params);
//
//    // Fetch the data
//    let response: BillsResponse = client.fetch(endpoint)?;
//
//    // Process the response
//    for bill in response.bills {
//        println!(
//            "{}, {}, {}\n",
//            bill.title.unwrap_or("".to_string()),
//            bill.bill_type.unwrap_or("".to_string()),
//            bill.number.unwrap_or("".to_string())
//        );
//    }
//
//    Ok(())
//}
