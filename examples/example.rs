use std::process::exit;

use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, 
    param_models::{
        FormatType, MemberListParams
    },
    curl_and_jq
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        current_member: Some(true),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    println!("{}", get_endpoint_url(endpoint.clone()));

    match curl_and_jq(&get_endpoint_url(endpoint), ".members | to_entries[] | {name: .value.name, party: .value.partyName, state: .value.state}") {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
