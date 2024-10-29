use cdg_api::{ChamberType, CommitteeOption, CommonParams, Endpoints, NewEndpoint, curl_and_jq};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up common parameters
    let params = CommonParams {
        format: Some(cdg_api::Format::Json),
        limit: Some(10),
        ..CommonParams::new()
    };

    // Create committee endpoint
    let committee_endpoint = Endpoints::new_committee(
        Some(ChamberType::Senate),
        Some(118),
        None,
        Some(CommitteeOption::Bills),
        Some(params),
    );

    // Generate URL
    let url = committee_endpoint.to_url();

    // Make the HTTP request
    curl_and_jq(&url)?;

    Ok(())
}
