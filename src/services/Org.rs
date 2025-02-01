
use crate::{
    Error,
    Response,
    GraphQLQuery,
    QueryBody,
    GetOrg,
    get_org,
    to_console_debug,
    to_console_error,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    check_deser,
    NavAbilityClient,
};

pub fn get_org_query(
) -> QueryBody<get_org::Variables> { 
    GetOrg::build_query(get_org::Variables {}) 
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_org_id(
    nvacl: NavAbilityClient,
) -> Result<Response<get_org::ResponseData>, Box<dyn Error>> {
    
    let request_body = get_org_query();

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_org::ResponseData>(
        req_res?.json().await
    )
}