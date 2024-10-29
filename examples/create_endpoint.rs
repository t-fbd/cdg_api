use cdg_api::{Endpoints, BillType, CommonParams, NewEndpoint};

fn main() {
    // Example: Create a bill endpoint
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        offset: Some(0),
        limit: Some(10),
        from_date_time: None,
        to_date_time: None,
        sort: Some(cdg_api::Sort::DateDesc),
    };

    let bill_endpoint = Endpoints::new_bill(
        Some(117),           // Congress number
        Some(BillType::Hr),  // Bill type
        Some("1".to_string()), // Bill number
        Some(cdg_api::BillOption::Text), // Bill option
        Some(params),
    );

    let url = bill_endpoint.to_url();
    println!("Generated URL: {}", url);
}
