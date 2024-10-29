use std::process::exit;

use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, 
    param_models::{
        FormatType, MemberListParams
    },
    response_models::*,
    get_congress_data
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    let url = get_endpoint_url(endpoint);

    println!("URL: {}", url);

    let response: MembersResponse = match get_congress_data(&url) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    for member in response.members {
        println!("{}, {}, {}\n", member.name, member.state, member.party_name);
    }
}
