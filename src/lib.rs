// use graphql_client::{GraphQLQuery, Response};

use ::reqwest::blocking::Client;
use graphql_client::{
    // reqwest,
    reqwest::post_graphql_blocking,
    GraphQLQuery,
};
// use std::error::Error;
// use log::*;

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
struct NavAbilityClient {
    client: Client,
    apiurl: String,
    user_label: String,
    nva_api_token: String,
}
impl NavAbilityClient {
    fn new(apiurl: &str, user_label: &str, nva_api_token: &str) -> Self {
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

fn get_robots(client: &NavAbilityClient) -> get_robots::ResponseData {
    let variables = get_robots::Variables {
        user_label: client.user_label.clone(),
    };

    let response_body =
        post_graphql_blocking::<GetRobots, _>(&client.client, &client.apiurl, variables)
            .expect("Failure to post graphql");
    
    //debug print raw response body
    // println!("responce body {:?}", response_body);

    let response_data: get_robots::ResponseData =
        response_body.data.expect("missing response data");

    return response_data;
}

fn list_robots(client: &NavAbilityClient) -> list_robots::ResponseData {
    let variables = list_robots::Variables {
        user_label: client.user_label.clone(),
    };
    let response_body =
        post_graphql_blocking::<ListRobots, _>(&client.client, &client.apiurl, variables)
            .expect("Failure to post graphql");

    let response_data: list_robots::ResponseData =
        response_body.data.expect("missing response data");

    return response_data;
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
        let client = NavAbilityClient::new(&api_url, &nva_userlabel, &nva_api_token);
        println!("client: {:?}", client);
        let robotrs = get_robots(&client);
        println!("robots: {:?}", robotrs);

        let robotlist = list_robots(&client);
        println!("robot list: {:?}", robotlist);
    }
}
