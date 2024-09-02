

use std::error::Error;
use std::sync::mpsc;
use uuid::Uuid;
use chrono::{self, Utc};

use graphql_client::{
    GraphQLQuery,
    Response
};

#[cfg(feature="wasm")]
use reqwest::Client;

#[cfg(target_arch = "wasm32")]
use gloo_console::{__macro::JsValue, log};

// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
use tokio;
#[cfg(feature = "tokio")]
use reqwest::Client;

#[cfg(feature="blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature="blocking")]
use graphql_client::reqwest::post_graphql_blocking;


fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

type EmailAddress = String;
type DateTime = String;
type Metadata = String;

#[derive(GraphQLQuery, Clone)]
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/user_robot_session.graphql",
    response_derives = "Debug"
)]
pub struct GetURS;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "src/schema.json",
//     query_path = "src/add_robots.graphql",
//     response_derives = "Debug"
// )]
// pub struct AddRobot;

// unclear if manual definition of user robot session is necessary
pub struct User {
    id: Uuid,
    label: String,
    _version: String,
    created_timestamp: chrono::DateTime::<Utc>,
    last_updated_timestamp: chrono::DateTime::<Utc>,
}

pub struct Robot {
    pub id: Option<Uuid>,
    pub label: String,
    pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}

impl Robot {
    pub fn new(
        id: Option<Uuid>,
        label: String,
        _version: String,
        created_timestamp: chrono::DateTime<Utc>,
        last_updated_timestamp: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            label,
            _version,
            created_timestamp,
            last_updated_timestamp,
        }
    }
}

pub struct Session {
    id: Uuid,
    label: String,
    robot_label: String,
    user_label: String,
    _version: String,
    created_timestamp: chrono::DateTime::<Utc>,
    last_updated_timestamp: chrono::DateTime::<Utc>,
}

# [derive(Default)]
pub struct BlobEntry {
    id: Option<Uuid>,
    blobId: Option<Uuid>,
    originId: Uuid,
    label: String,
    blobstore: String,
    hash: String,
    origin: String,
    size: Option<i32>,
    description: String,
    mimeType: String,
    metadata: String,
    timestamp: chrono::DateTime<Utc>,
    createdTimestamp: Option<chrono::DateTime<Utc>>,
    lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
    _type: String,
    _version: String,
}

impl BlobEntry {
    fn new() -> Self {
        Default::default()
    }
}

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

#[derive(Debug, Clone)]
pub struct NavAbilityClient {
    client: Client,
    pub apiurl: String,
    pub user_label: String,
    pub nva_api_token: String,
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


pub async fn fetch_urs_async(
    nvacl: &NavAbilityClient,
    robot_label: String,
    session_label: String,
) -> Result<Response<get_urs::ResponseData>, Box<dyn Error>> {

    let variables = get_urs::Variables {
        user_label: nvacl.user_label.to_string(),
        robot_label: robot_label.to_string(),
        session_label: session_label.to_string(),
    };

    let request_body = GetURS::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;


    match req_res {
        Err(re) => {
            tracing::error!("Failed to get NavAbility API response {}", re);
            #[cfg(target_arch = "wasm32")]
            {
                gloo_console::log!("NvaSDK.rs, post to url ", format!("{:?}", &nvacl.apiurl));
                gloo_console::log!("NvaSDK.rs, client was ", format!("{:?}", &nvacl.client));
                gloo_console::log!("NvaSDK.rs, failed to get NavAbility API response", format!("{:?}", re));
            }
            return Err(Box::new(re));
        },
        Ok(res) => {
            let serde_res = res.json().await;
            match serde_res {
                Ok(response_body) => {
                    tracing::debug!("received and json deserialized GetURS");

                    #[cfg(target_arch = "wasm32")]
                    gloo_console::log!("NvaSDK.rs ", "received and json GetURS");

                    return Ok(response_body);
                },
                Err(e) => {
                    tracing::error!("failed to unpack json from GQL API response: {}", &e);

                    #[cfg(target_arch = "wasm32")]            
                    gloo_console::log!("NvaSDK.rs ", "failed to unpack json from GQL API response");

                    return Err(Box::new(e));
                }
            }
        }
    }
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

    // #[cfg(target_arch = "wasm32")] 
    // {
    //     gloo_console::log!("NvaSDK.rs nvacl.apiurl ", nvacl.apiurl.to_string());
    //     gloo_console::log!("NvaSDK.rs nvacl.user_label ", nvacl.user_label.to_string());
    //     gloo_console::log!("NvaSDK.rs nvacl.nva_api_token ", nvacl.nva_api_token.to_string());
    //     gloo_console::log!("NvaSDK.rs request_body.query ", request_body.query.to_string());
    // }

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;
    // .expect("Failed to get NavAbility API response");
    match req_res {
        Err(re) => {
            tracing::error!("Failed to get NavAbility API response {}", re);
            #[cfg(target_arch = "wasm32")]
            {
                gloo_console::log!("NvaSDK.rs, post to url ", format!("{:?}", &nvacl.apiurl));
                gloo_console::log!("NvaSDK.rs, client was ", format!("{:?}", &nvacl.client));
                gloo_console::log!("NvaSDK.rs, failed to get NavAbility API response", format!("{:?}", re));
            }
            return Err(Box::new(re));
        },
        Ok(res) => {
            // : Response<get_robots::ResponseData>
            let serde_res = res.json().await;
            // .expect("Failed to json unpack GQL response");
            match serde_res {
                Ok(response_body) => {
                    tracing::debug!("received and json deserialized user robots");

                    #[cfg(target_arch = "wasm32")]
                    gloo_console::log!("NvaSDK.rs ", "received and json deserialized user robots");

                    return Ok(response_body);
                },
                Err(e) => {
                    tracing::error!("failed to unpack json from GQL API response: {}", &e);

                    #[cfg(target_arch = "wasm32")]            
                    gloo_console::log!("NvaSDK.rs ", "failed to unpack json from GQL API response");

                    return Err(Box::new(e));
                }
            }
        }
    }
    // dbg!("{:?}", &response_body);

    // #[cfg(target_arch = "wasm32")]
    // gloo_console::log!("NvaSDK.rs ", type_of(&response_body));

    // Ok(response_body)
    // serde_res
}




#[cfg(target_arch = "wasm32")]
pub async fn fetch_context_web(
    send_into: mpsc::Sender<Vec<get_urs::GetUrsUsers>>, 
    client: &NavAbilityClient,
    robot_label: String,
    session_label: String,
) { // -> Vec<get_robots::GetRobotsUsers> {      
    if let Ok(response_body) = fetch_urs_async(&client, robot_label, session_label).await {
        let urs_data = response_body.data;
        match urs_data {
            None => gloo_console::log!("NvaSDK.rs ", JsValue::from("NvaSDK.rs, bad GQL response")),
            Some(resdata) => {
                let urs_data = resdata.users;
                let res_len = urs_data.len();
                gloo_console::log!("length of context send_into.send ", JsValue::from(res_len));  

                let resp = send_into.send(urs_data);
                if let Err(e) = resp {
                    tracing::error!("Error sending user robot list data: {}", e);
                }
            }
        }
    } else {
        tracing::error!("NvaSDK.rs Unable to fetch list from client connection");
        #[cfg(target_arch = "wasm32")]            
        gloo_console::log!("NvaSDK.rs ", "Unable to fetch list from client connection");
    }

}



#[cfg(target_arch = "wasm32")]
pub async fn fetch_ur_list_web(
    send_into: mpsc::Sender<Vec<get_robots::GetRobotsUsers>>, 
    client: &NavAbilityClient
) { // -> Vec<get_robots::GetRobotsUsers> {      
    if let Ok(response_body) = fetch_robots_async(&client).await {
        let ur_list_data = response_body.data;
        match ur_list_data {
            None => gloo_console::log!("NvaSDK.rs ", JsValue::from("NvaSDK.rs, bad GQL response")),
            Some(resdata) => {
                let ur_data = resdata.users;
                let res_len = ur_data.len();
                gloo_console::log!("length of data resp going send_into.send ", JsValue::from(res_len));                
                let resp = send_into.send(ur_data);
                if let Err(e) = resp {
                    tracing::error!("Error sending user robot list data: {}", e);
                }
            }
        }
    } else {
        tracing::error!("NvaSDK.rs Unable to fetch list from client connection");
        #[cfg(target_arch = "wasm32")]            
        gloo_console::log!("NvaSDK.rs ", "Unable to fetch list from client connection");
    }

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
    let ur_list_data = rt.block_on(async { 
        fetch_robots_async(&nvacl).await
    }).unwrap().data;

    match ur_list_data { // .data.expect("Problem with GQL response")
        Some(data) => {
            let ur_list = data.users;
            if let Err(e) = send_into.send(ur_list) {
                tracing::error!("Error sending user robot list data: {}", e);
            };
            return Ok(())
        },
        None => {
            return panic!("Problem with GQL response");
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

        #[cfg(feature = "blocking")]
        let robotrs = get_robots_blocking(&client);
        // println!("robots: {:?}", robotrs);

        #[cfg(feature = "blocking")]
        let robotlist = get_robots_blocking(&client);
        // println!("robot list: {:?}", robotlist);
    }
}
