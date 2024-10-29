use cdg_api::{BillOption, BillType, CommonParams, Endpoints, NewEndpoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up common parameters
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        limit: Some(5),
        ..CommonParams::new()
    };

    // Create bill endpoint
    let bill_endpoint = Endpoints::new_bill(
        Some(117),
        Some(BillType::Hr),
        Some("1".to_string()),
        Some(BillOption::Text),
        Some(params),
    );

    // Generate URL
    let url = bill_endpoint.to_url();
    println!("Request URL: {}", url);

    // Make the HTTP request

    Ok(())
}
