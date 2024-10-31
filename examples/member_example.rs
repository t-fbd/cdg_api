use cdg_api::CongressApiClient;
use cdg_api::endpoints::Endpoints;
use cdg_api::param_models::MemberListParams;
use cdg_api::cdg_types::FormatType;
use cdg_api::response_models::MembersResponse;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let client = CongressApiClient::new(None)?; // Use default API key
                                                // Set the API key explicitly with:
                                                // CongressApiClient::new(Some("api_key_here"))?;
    // Define parameters
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    // Create the endpoint
    let endpoint = Endpoints::MemberList(params);

    // Fetch the data, casting it to the appropriate response type
    let response: MembersResponse = client.fetch(endpoint)?;

    // Process the response
    for member in response.members {
        println!("{}, {}, {}\n", member.name, member.state, member.party_name);
    }

    Ok(())
}
