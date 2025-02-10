
use crate::{
    Utc,
    Uuid,
    Sender,
    GraphQLQuery,
    Response,
    Error,
    NavAbilityClient,
    ListGraphs,
    list_graphs,
    check_deser,
    to_console_debug,
    to_console_error,
};




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_graphs(
    nvacl: NavAbilityClient,
    id: &Uuid,
) -> Result<Response<list_graphs::ResponseData>, Box<dyn Error>> {
    
    let request_body = ListGraphs::build_query(list_graphs::Variables {
        id: id.to_string()
    });

    // TBD
    // let query = MyQuery::build_query(my_query::Variables {});
    // match query {
    //     Ok(q) => () // println!("Query: {:?}", q),
    //     Err(e) => eprintln!("Failed to build query: {:?}", e),
    // }

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<list_graphs::ResponseData>(
        req_res?.json().await
    )
}