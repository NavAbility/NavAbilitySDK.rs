

use std::error::Error;
use std::sync::mpsc;
// use log::*;

use graphql_client::{
    // reqwest,
    GraphQLQuery,
    Response
};

// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
use tokio;
#[cfg(feature="tokio")]
use reqwest::Client;

#[cfg(feature="blocking")]
use ::reqwest::blocking::Client;
// #[cfg(feature="blocking")]
// use graphql_client::reqwest;
#[cfg(feature="blocking")]
use graphql_client::reqwest::post_graphql_blocking;
// #[cfg(not(target_arch = "wasm32"))]




type EmailAddress = String;
type DateTime = String;
type Metadata = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/robot_queries.graphql",
    response_derives = "Debug"
)]
pub struct GetRobots;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/robot_queries.graphql",
    response_derives = "Debug"
)]
pub struct ListRobots;

// likely in an earlier compile step via build.rs
// Schema can maybe be generated with something like:
// graphql-client introspect-schema https://api.d1.navability.io/graphql --output=schema.json

// async fn perform_my_query(url: &str, variables: get_robots::Variables) -> Result<(), Box<dyn Error>> {
//     // this is the important line
//     let request_body = GetRobots::build_query(variables);

//     let client = reqwest::Client::new();
//     let mut res = client.post(url).json(&request_body).send().await?;
//     let response_body: Response<get_robots::ResponseData> = res.json().await?;
//     println!("{:#?}", response_body);
//     Ok(())
// }

#[derive(Debug)]
pub struct NavAbilityClient {
    client: Client,
    apiurl: String,
    user_label: String,
    nva_api_token: String,
}
impl NavAbilityClient {
    pub fn new(apiurl: &String, user_label: &String, nva_api_token: &String) -> Self {
        let client = Client::builder()
            .user_agent("graphql-rust/0.12.0")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", nva_api_token))
                        .unwrap(),
                ))
                .collect(),
            )
            .build()
            .expect("Failure to create client");

        NavAbilityClient {
            client,
            apiurl: apiurl.to_string(),
            user_label: user_label.to_string(),
            nva_api_token: nva_api_token.to_string(),
        }
    }
}

#[cfg(feature = "blocking")]
pub fn get_robots_blocking(client: &NavAbilityClient) -> get_robots::ResponseData {
    let variables = get_robots::Variables {
        user_label: client.user_label.clone(),
    };

    let response_body =
        post_graphql_blocking::<GetRobots, _>(&client.client, &client.apiurl, variables)
            .expect("Failure to post graphql");
    
    //debug print raw response body
    dbg!(&response_body);

    let response_data: get_robots::ResponseData =
        response_body.data.expect("missing response data");

    return response_data;
}


// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "blocking")]
pub fn fetch_ur_list_blocking(
    send_into: &mut mpsc::Sender<Vec<get_robots::GetRobotsUsers>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> {

    // THIS IS THE LEGACY VERSION IN SDK FIXME TO NEW VERSION FOR WEB/WASM
    let ur_list = get_robots_blocking(&nvacl).users;
    // dbg!(&ur_list);

    if let Err(e) = send_into.send(ur_list) {
        tracing::error!("Error sending user robot list data: {}", e);
    };

    Ok(())
}


pub async fn fetch_robots_async(
    nvacl: &NavAbilityClient,
) -> Result<Response<get_robots::ResponseData>, Box<dyn Error>> {

    // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
    let variables = get_robots::Variables {
        user_label: nvacl.user_label.to_string()
    };

    // this is the important line
    let request_body = GetRobots::build_query(variables);

    let res = nvacl.client.post(&nvacl.apiurl).json(&request_body).send().await?;
    let response_body: Response<get_robots::ResponseData> = res.json().await?;
    dbg!("{:?}", &response_body);
    Ok(response_body)
}



// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
pub fn fetch_ur_list_tokio(
    send_into: &mut mpsc::Sender<Vec<get_robots::GetRobotsUsers>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> { // -> Vec<get_robots::GetRobotsUsers> {

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let ur_list = rt.block_on(async { 
        fetch_robots_async(&nvacl).await
    }).unwrap()
        .data
        .expect("Problem with GQL response")
        .users;

    if let Err(e) = send_into.send(ur_list) {
        tracing::error!("Error sending user robot list data: {}", e);
    };

    Ok(())
}



#[cfg(target_arch = "wasm32")]
pub async fn fetch_ur_list_web(
    send_into: &mut mpsc::Sender<Vec<get_robots::GetRobotsUsers>>, 
    api_url: &String, 
    auth_token: &String
) { // -> Vec<get_robots::GetRobotsUsers> {
    // FIXME this internally grabs api_key from env
    let ur_list = 
        fetch_robots_async(&api_url, &auth_token)
        .await
        .unwrap()
        .data
        .expect("Problem with GQL response")
        .users;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_robots() {
        let nva_userlabel: String =
            std::env::var("NAVABILITY_USERLABEL").expect("Missing NAVABILITY_USERLABEL env var");

        let nva_api_token: String =
            std::env::var("NAVABILITY_API_TOKEN").expect("Missing NAVABILITY_API_TOKEN env var");

        let api_url: &str = "https://api.d1.navability.io/graphql";
        let client = NavAbilityClient::new(&api_url.to_string(), &nva_userlabel, &nva_api_token);
        println!("client: {:?}", client);
        let robotrs = get_robots_blocking(&client);
        println!("robots: {:?}", robotrs);

        let robotlist = get_robots_blocking(&client);
        println!("robot list: {:?}", robotlist);
    }
}
