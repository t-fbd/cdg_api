use cdg_api::{
    url_builders::get_endpoint_url,
    Endpoints, NewEndpoint, 
    param_models::{
        FormatType, MemberListParams
    }
};

fn main() {
    let params = MemberListParams {
        format: Some(FormatType::Json),
        limit: Some(10),
        ..MemberListParams::default()
    };

    let endpoint = Endpoints::new_member_list(params);

    println!("{}", get_endpoint_url(endpoint));
}
