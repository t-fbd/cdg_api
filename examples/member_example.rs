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
    let params = MemberListParams::default()
        .format(FormatType::Json)
        .limit(10)
        .current_member(true);

    // Create the endpoint
    let endpoint = Endpoints::MemberList(params);

    // Fetch the data, casting it to the appropriate response type
    let response: MembersResponse = client.fetch(endpoint)?;

    // Process the response
    for member in response.members {
        println!(
            "{}, {}, {}\n", 
            member.name.unwrap_or("".to_string()),
            member.state.unwrap_or("".to_string()),
            member.party_name.unwrap_or("".to_string())
        );
    }

    Ok(())
}
