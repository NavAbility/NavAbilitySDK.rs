

use std::error::Error;
use std::sync::mpsc::Sender;
use uuid::Uuid;
use chrono::{self, Utc};

use graphql_client::{
    GraphQLQuery, 
    Response
};

#[cfg(feature="wasm")]
use reqwest::Client;
// #[cfg(feature="wasm")]
// use reqwest::multipart::Part; // requires multipart


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
type BigInt = i64;
type B64JSON = String;
type Latitude = f64;
type Longitude = f64;
type UUID = String;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/robot_queries.graphql",
    response_derives = "Debug"
)]
pub struct GetAgents;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/user_robot_session.graphql",
    response_derives = "Debug"
)]
pub struct GetURS;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/CreateDownload.gql",
    response_derives = "Debug"
)]
pub struct CreateDownload;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/blob_store.graphql",
    response_derives = "Debug"
)]
pub struct CreateUpload;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/CompleteUpload.gql",
    response_derives = "Debug"
)]
pub struct CompleteUpload;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/AddBlobEntryAgent.gql",
    response_derives = "Debug"
)]
pub struct AddBlobEntries;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/blob_entry.graphql",
    response_derives = "Debug"
)]
pub struct GetBlobEntry;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/GetOrg.gql",
    response_derives = "Debug"
)]
pub struct GetOrg;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/AddAgent.gql",
    response_derives = "Debug"
)]
pub struct AddAgent;

// unclear if manual definition of user robot session is necessary
pub struct User {
    id: Uuid,
    label: String,
    _version: String,
    created_timestamp: chrono::DateTime::<Utc>,
    last_updated_timestamp: chrono::DateTime::<Utc>,
}

pub struct Agent {
    pub id: Option<Uuid>,
    pub label: String,
    // pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}

impl Agent {
    pub fn new(
        id: Option<Uuid>,
        label: String,
        // _version: String,
        created_timestamp: chrono::DateTime<Utc>,
        last_updated_timestamp: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            label,
            // _version,
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

/// A `BlobEntry` is a small about of structured data that holds reference information to find an actual blob. Many `BlobEntry`s 
/// can exist on different graph nodes spanning Robots, and Sessions which can all reference the same `Blob`.  A `BlobEntry` 
/// is also a equivalent to a bridging entry between local `.originId` and a remotely assigned `.blobIds`.
# [derive(Default, Clone)]
pub struct BlobEntry {
    /// Remotely assigned and globally unique identifier for the `BlobEntry` itself (not the `.blobId`).
    pub id: Option<Uuid>,
    /// Machine friendly and globally unique identifier of the 'Blob', usually assigned from a common point in the system.  This can be used to guarantee unique retrieval of the large data blob.
    pub blobId: Uuid,
    /// Machine friendly and locally assigned identifier of the 'Blob'.  `.originId`s are mandatory upon first creation at the origin regardless of network access.  Separate from `.blobId` since some architectures do not allow edge processes to assign a uuid4 to data store elements.
    pub originId: Option<Uuid>,
    /// Human friendly label of the `Blob` and also used as unique identifier per node on which a `BlobEntry` is added.  E.g. do "LEFTCAM_1", "LEFTCAM_2", ... of you need to repeat a label on the same variable.
    pub label: String,
    /// A hint about where the `Blob` itself might be stored.  Remember that a Blob may be duplicated over multiple blobstores.
    pub blobstore: String,
    /// A hash value to ensure data consistency which must correspond to the stored hash upon retrieval.  Use `bytes2hex(sha256(blob))`. [Legacy: some usage functions allow the check to be skipped if needed.]
    pub hash: String,
    /// Context from which a BlobEntry=>Blob was first created. E.g. user|robot|session|varlabel.
    pub origin: String,
    /// number of bytes in blob
    pub size: Option<i64>,
    /// Additional information that can help a different user of the Blob.
    pub description: String,
    /// MIME description describing the format of binary data in the `Blob`, e.g. 'image/png' or 'application/json; _type=CameraModel'.
    pub mimeType: String,
    /// Additional storage for functional metadata used in some scenarios, e.g. to support advanced features such as `parsejson(base64decode(entry.metadata))['time_sync']`.
    pub metadata: String,
    /// When the Blob itself was first created.
    pub timestamp: chrono::DateTime<Utc>,
    /// When the BlobEntry was created.
    pub createdTimestamp: Option<chrono::DateTime<Utc>>,
    /// Use carefully, but necessary to support advanced usage such as time synchronization over Blob data.
    pub lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
    /// Self type declaration for when duck-typing happens.
    pub _type: String,
    /// Type version of this BlobEntry. Consider upgrading to `::VersionNumber`.
    pub _version: String,
}



impl BlobEntry {
    pub fn new() -> Self {
        let mut be = BlobEntry::default();
        be.blobId = Uuid::new_v4();
        be.blobstore = "default".to_string();
        be.origin = "NvaSDK.rs".to_string();
        be.createdTimestamp = Some(Utc::now());
        be.lastUpdatedTimestamp = be.createdTimestamp.clone();
        be._type = "BlobEntry".to_string(); // for self assemply typed usage elsewhere
        be._version = "0.24.0".to_string(); // FIXME dont hardcode, pull from common source
        return be
    }

    pub fn try_from_receiver(
        rx: &std::sync::mpsc::Receiver<get_blob_entry::ResponseData>
    ) -> Option<Self> {
        
        match rx.try_recv() {
            Ok(gqle) => {
                let gety = &gqle.blob_entries[0];
                let mut be = BlobEntry::default();
                be.id = Some(Uuid::parse_str(&gety.id).expect("failed to parse entry id to uuid"));
                be.blobId = Uuid::parse_str(&gety.blob_id).expect("failed to parse entry blob_id to uuid");
                be.label = gety.label.to_string();
                if let Some(blobstore) = &gety.blobstore {
                    be.blobstore = blobstore.to_string();
                }
                if let Some(origin) = &gety.origin {
                    be.origin = origin.to_string();
                }
                if let Some(mime) = &gety.mime_type {
                    be.mimeType = mime.to_string();
                }
                if let Some(description) = &gety.description {
                    be.description = description.to_string();
                }
                if let Some(hash) = &gety.hash {
                    be.hash = hash.to_string();
                }
                if let Some(size) = &gety.size {
                    be.size = Some(*size);
                }
                if let Some(timestamp) = &gety.timestamp {
                    to_console_debug(&format!("BlobEntry from rx timestamp string {}",&timestamp));
                    // 2024-09-16T16:51:20.555Z
                    if let Ok(tms) = chrono::DateTime::parse_from_str(
                        &timestamp
                            .replace("Z"," +00")
                            .replace(" UTC", " +00"), 
                        "%Y-%m-%dT%H:%M:%S%.f %#z"
                    ) {
                        be.timestamp = tms.to_utc();
                    } else {
                        to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
                    }
                }
                // be.createdTimestamp = Some(Utc::now());
                // be.lastUpdatedTimestamp = be.createdTimestamp.clone();
                if let Some(_type) = &gety.type_ {
                    be._type = _type.to_string();
                }
                be._version = gety.version.to_string();
                return Some(be)
            }
            Err(e) => {
                // to_console_debug(&"BlobEntry::try_from_receive nothing in channel");
            }
        }

        return None
    }
}





#[derive(Debug, Clone)]
pub struct NavAbilityClient {
    client: Client,
    pub apiurl: String,
    pub user_label: String,
    pub nva_api_token: String,
}
impl NavAbilityClient {
    pub fn new(apiurl: &String, user_label: &String, nva_api_token: &String) -> Self {
        // FIXME good header.insert example: https://medium.com/@itsuki.enjoy/post-file-using-multipart-form-data-in-rust-5171ae57aeed
        //   or https://users.rust-lang.org/t/how-to-upload-a-file-using-rust-or-some-library/45423/4
        let client = Client::builder()
        .user_agent("graphql-rust/0.12.0")
        .default_headers(
                // TODO use HeaderMap: https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.headers
                // TODO use bearer auth: https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.bearer_auth
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", nva_api_token))
                        .unwrap(),
                )).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str(&apiurl)
                            .unwrap(),
                    ))
                ).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str(&apiurl.replace("api.","app.d"))
                            .unwrap(),
                    ))
                )
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



// get_org::ResponseData
pub fn send_query_result<T>(
    send_into: Sender<T>,
    response_body: Result<Response<T>,Box<dyn Error>>,
) {
    match response_body {
        Ok(resbody) => {
            if resbody.errors.is_none() {
                match resbody.data {
                    Some(data) => {
                        let _ = send_into.send(data);
                    }
                    None => to_console_error(&"API query response data is none."),
                }
            } else {
                to_console_error(&format!("API query responed with error: {:?}", &resbody.errors));
                // return Err(Box::new(response_body.errors));
            }
        }
        Err(e) => {
            to_console_error(&format!("API query failure: {:?}",&e));
        }
    }
}


// missing traits for generic serde on query types
// async fn post_query_serde<V,R>(
//     nvacl: NavAbilityClient,
//     request_body: QueryBody<V>,
// ) -> Result<R,Box<dyn Error>> {
//     let req_res = nvacl.client
//     .post(&nvacl.apiurl)
//     .json(&request_body)
//     .send().await;

//     match req_res {
//         Err(re) => {
//             to_console_error(&format!("API request error: {:?}", re));
//             return Err(Box::new(re));
//         },
//         Ok(res) => {
//             let serde_res = res.json().await;
//             match serde_res {
//                 Ok(response_body) => {
//                     return Ok(response_body)
//                 },
//                 Err(e) => {
//                     to_console_error(&format!("JSON unpack of API response failed: {:?}", &e));
//                     return Err(Box::new(e));
//                 }
//             }
//         }
//     }
// }




fn check_deser<T>(
    serde_res: Result<Response<T>,reqwest::Error>
) -> Result<Response<T>,Box<dyn Error>> {

    if let Err(ref e) = serde_res {
        to_console_error(&format!("JSON unpack of API response failed: {:?}", &e));
    }

    return Ok(serde_res?)
}


pub async fn fetch_org_id(
    nvacl: NavAbilityClient,
) -> Result<Response<get_org::ResponseData>, Box<dyn Error>> {
    
    let variables = get_org::Variables {};
    let request_body = GetOrg::build_query(variables);

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


pub async fn add_agent_async(
    nvacl: NavAbilityClient,
    agent_label: &String,
) -> Result<Response<add_agent::ResponseData>,Box<dyn Error>> {
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}",&agent_label).to_string();
    let agent_id = Uuid::new_v5(&org_id, name.as_bytes());

    let variables = add_agent::Variables {
        agent_id: agent_id.to_string(),
        label: agent_label.to_string(),
        version: "0.24".to_string(),
        org_id: org_id.to_string(),
    };

    let request_body = AddAgent::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<add_agent::ResponseData>(
        req_res?.json().await
    )
}

pub async fn add_entry_agent_async(
    nvacl: NavAbilityClient,
    agent_label: &String,
    entry: &BlobEntry,
    metadata: Option<String>,
)  -> Result<Response<add_blob_entries::ResponseData>, Box<dyn Error>> {
    
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}{}",&agent_label,&entry.label).to_string();
    let entry_id = Uuid::new_v5(&org_id, name.as_bytes());

    let variables = add_blob_entries::Variables {
        agent_label: agent_label.to_string(),
        entry_id: entry_id.to_string(),
        entry_label: entry.label.to_string(),
        blob_id: entry.blobId.to_string(),
        blobstore: Some(entry.blobstore.to_string()),
        origin: Some(entry.origin.to_string()),
        mime_type: Some(entry.mimeType.to_string()),
        metadata: metadata.unwrap_or("e30=".to_string()),
        description: Some(entry.description.to_string()),
        hash: entry.hash.to_string(),
        size: entry.size,
        // metadata: Some(entry.metadata.to_string()),
        timestamp: Some(entry.timestamp.to_string()),
    };

    let request_body = AddBlobEntries::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<add_blob_entries::ResponseData>(
        req_res?.json().await
    )
}
// let gqlentry = add_blob_entries::BlobEntryCreateInput::new(
//     org_id,
//     entry,
//     agent_label.to_string(),
// );
// let mut blob_entries = Vec::new();
// blob_entries.push(gqlentry);
// let variables = add_blob_entries::Variables {
//     blob_entries,
// };


pub async fn fetch_urs_async(
    nvacl: &NavAbilityClient,
    // robot_label: String,
    // session_label: String,
) -> Result<Response<get_urs::ResponseData>, Box<dyn Error>> {

    let variables = get_urs::Variables {
        org_id: nvacl.user_label.to_string(),
    };

    let request_body = GetURS::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_urs::ResponseData>(
        req_res?.json().await
    )
}


pub async fn fetch_robots_async(
    nvacl: &NavAbilityClient,
) -> Result<Response<get_agents::ResponseData>, Box<dyn Error>> {

    // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
    let variables = get_agents::Variables {
        org_id: nvacl.user_label.to_string(),
        // Uuid::new_v4().to_string() // FIXME uuid
    };

    let request_body = GetAgents::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_agents::ResponseData>(
        req_res?.json().await
    )
}



pub async fn fetch_blob_entry(
    nvacl: NavAbilityClient,
    id: Uuid
) -> Result<Response<get_blob_entry::ResponseData>, Box<dyn Error>> {

    let variables = get_blob_entry::Variables {
        entry_id: id.to_string(),
    };

    let request_body = GetBlobEntry::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_blob_entry::ResponseData>(
        req_res?.json().await
    )
}

pub async fn send_blob_entry(
    send_into: std::sync::mpsc::Sender<get_blob_entry::ResponseData>,
    nvacl: NavAbilityClient,
    id: Uuid
) {
    let resp = fetch_blob_entry(nvacl, id).await;
    send_query_result::<get_blob_entry::ResponseData>(send_into, resp);
}


#[cfg(target_arch = "wasm32")]
pub async fn fetch_context_web(
    send_into: Sender<Vec<get_urs::GetUrsOrgs>>, 
    client: &NavAbilityClient,
    robot_label: String,
    session_label: String,
) { // -> Vec<get_robots::GetRobotsUsers> {
    let result = fetch_urs_async(&client).await;
    // FIXME use send_query_result instead, refactor .orgs part
    // send_query_result(send_into, result);
    if let Ok(response_body) = result {
        let res_errs = response_body.errors;
        match res_errs {
            Some(ref err) => {
                to_console_error(&format!("fetch_context_web has response errors {:?}",&res_errs));
            },
            None => {
                let urs_data = response_body.data;
                match urs_data {
                    None => gloo_console::log!("NvaSDK.rs ", JsValue::from("NvaSDK.rs, GQL response_body.data is empty")),
                    Some(resdata) => {
                        let urs_data = resdata.orgs;
                        let res_len = urs_data.len();
                        to_console_debug(&format!("length of context send_into.send {}", res_len));  
        
                        let resp = send_into.send(urs_data);
                        if let Err(e) = resp {
                            to_console_error(&format!("Error sending user robot list data: {}", e));
                        }
                    }
                }
            }
        }
    } else {
        to_console_error(&"Unable to fetch list from client connection");
    }

}



#[cfg(target_arch = "wasm32")]
pub async fn fetch_ur_list_web(
    send_into: Sender<Vec<get_agents::GetAgentsAgents>>, 
    client: &NavAbilityClient
) { // -> Vec<get_robots::GetRobotsUsers> {      
    let result = fetch_robots_async(&client).await;
    if let Ok(response_body) = result {
        let res_errs = response_body.errors;
        match res_errs {
            Some(ref err) => {
                tracing::error!("NvaSDK.rs fetch_ur_list_web has response errors {:?}",&res_errs);
                #[cfg(target_arch = "wasm32")]
                gloo_console::log!(format!("NvaSDK.rs fetch_ur_list_web has response errors {:?}",&res_errs));
            },
            None => {
                let ur_list_data = response_body.data;
                match ur_list_data {
                    None => gloo_console::log!("NvaSDK.rs ", JsValue::from("NvaSDK.rs, bad GQL response")),
                    Some(resdata) => {
                        let ur_data = resdata.agents;
                        let res_len = ur_data.len();
                        gloo_console::log!("length of data resp going send_into.send ", JsValue::from(res_len));                
                        let resp = send_into.send(ur_data);
                        if let Err(e) = resp {
                            tracing::error!("Error sending user robot list data: {}", e);
                        }
                    }
                }
            }
        }
    } else {
        to_console_error("fetch_robots_async(&client).await");
    }

}

#[cfg(target_arch = "wasm32")]
pub async fn create_upload_web(
    send_into: Sender<create_upload::ResponseData>, 
    client: &NavAbilityClient,
    name: &String,
    blob_size: i64,
    nparts: Option<i64>,
    blob_id: Option<Uuid>, // doenst work yet, leave None
) {
    let result = create_upload_async(
        client.clone(), 
        blob_id.expect("Must provide blob_id to create_upload_web"),
        nparts,
    ).await;
    send_query_result(send_into, result);
}



// #[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
pub fn fetch_ur_list_tokio(
    send_into: &mut Sender<Vec<get_agents::GetAgentsAgents>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> { // -> Vec<get_agents::GetAgentsAgents> {

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let ur_list_data = rt.block_on(async { 
        fetch_robots_async(&nvacl).await
    }).unwrap().data;

    match ur_list_data { // .data.expect("Problem with GQL response")
        Some(data) => {
            let ur_list = data.agents;
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
    send_into: &mut Sender<Vec<get_robots::GetRobotsUsers>>, 
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


pub async fn create_download_async(
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    store: Option<String>,
) -> Result<Response<create_download::ResponseData>, Box<dyn Error>> {

    let variables = create_download::Variables {
        blob_id: blob_id.to_string(),
        store: store.unwrap_or("default".to_string()).to_string(),
    };

    let request_body = CreateDownload::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<create_download::ResponseData>(
        req_res?.json().await
    )
}

pub async fn send_create_download(
    send_into: Sender<create_download::ResponseData>,
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    store: Option<String>
) {
    let resp = create_download_async(nvacl,blob_id,store).await;
    send_query_result::<create_download::ResponseData>(send_into,resp)
}


pub async fn create_upload_async(
    nvacl: NavAbilityClient,
    // label: String,
    // blob_size: i64,
    blob_id: Uuid,
    parts: Option<i64>,
) -> Result<Response<create_upload::ResponseData>, Box<dyn Error>> {

    let variables = create_upload::Variables {
        // label: label.to_string(),
        blob_id: blob_id.to_string(),
        parts: parts.unwrap_or(1),
    };

    let request_body = CreateUpload::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<create_upload::ResponseData>(
        req_res?.json().await
    )
}


pub async fn complete_upload_async(
    nvacl: NavAbilityClient,
    blob_id: Uuid,
    upload_id: String,
    etags: Vec<String>,
    // completed_upload: complete_upload::CompletedUploadInput,
) -> Result<Response<complete_upload::ResponseData>, Box<dyn Error>> {
    let mut parts: Vec<Option<complete_upload::CompletedUploadPartInput>> = vec![];
    for (i,et) in etags.iter().enumerate() {
        parts.push(
            Some(
                complete_upload::CompletedUploadPartInput {
                    part_number: (i + 1) as i64,
                    e_tag: Some(et.to_string()),
                }
            )
        )
    }

    let cupl = complete_upload::CompletedUploadInput {
        upload_id: upload_id.to_string(),
        parts
    };

    let variables = complete_upload::Variables {
        blob_id: blob_id.to_string(),
        completed_upload: cupl,
    };

    let request_body = CompleteUpload::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<complete_upload::ResponseData>(
        req_res?.json().await
    )
}


#[derive(Debug,Clone)]
pub struct FileUploader<T> {
    nvacl: NavAbilityClient,
    pub file: T, // assume read and seek are available
    blobId: Uuid,
    chunk_size: u64,
    nbytes_uploaded: u64,
}


impl<T> FileUploader<T> {
    pub fn new(
        nvacl: NavAbilityClient,
        file: T,
        label: String,
        blobId: Uuid,
        chunk_size: Option<u64>,
    ) -> Self {

        // create the actual uploader object
        Self {
            nvacl,
            file,
            blobId,
            chunk_size: chunk_size.expect("FileUpload expects chunk_size as u64"),
            nbytes_uploaded: 0 as u64,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn upload_file(
        &mut self,
        content: Vec<u8>,
        url_endpoint: String
    ) -> Result<String, Box<dyn std::error::Error>> {

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_LENGTH, 
            reqwest::header::HeaderValue::from(content.len())
        );
        // headers.insert(
        //     reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        //     reqwest::header::HeaderValue::from("https://_mySendIP???_")
        // );
        
        // PUT POST OPTIONS CORS: https://aws.amazon.com/blogs/media/deep-dive-into-cors-configs-on-aws-s3-how-to/
        let response = Client::new()
            .put(url_endpoint)
            .headers(headers)
            .body(content)
            .send()
            .await?;
            
            // // .multipart(file)
            // gloo_console::log!(format!("inner header {:?}", &postclient));
            
            let status_code = response.status();
            if reqwest::StatusCode::OK == status_code {
                let res_head = response.headers();
                let etag = res_head["etag"].to_str().unwrap().replace("\"","");
                // gloo_console::log!(format!("Headers:\n{:#?}", response.headers()));
                // gloo_console::log!(format!("Body:\n{}", response.text().await?));
                return Ok(etag)
            } else {
                to_console_error(&format!("Status: {:?}", &status_code));
                return Err(format!("Upload file put returned Status: {:?}", status_code).into())
            }
    }
}

// type CompletedUploadPartInput {
//   partNumber: Int!
//   eTag: String!
// }
// type CompletedUploadInput {
//   uploadId: String!
//   parts: [CompletedUploadPartInput!]!
// }


fn to_console_debug(
    text: &str
) {
    #[cfg(not(target_arch = "wasm32"))]
    tracing::debug!("{}",text);
    #[cfg(target_arch = "wasm32")]
    gloo_console::log!(text.to_string());
}

fn to_console_error(
    text: &str
) {
    #[cfg(not(target_arch = "wasm32"))]
    tracing::error!("ERROR NvaSDK.rs {}",&text);
    #[cfg(target_arch = "wasm32")]
    gloo_console::log!(&format!("ERROR NvaSDK.rs {}",&text));
}


// impl add_blob_entries::BlobEntryCreateInput {
//     pub fn new(
//         org_id: Uuid,
//         entry: &BlobEntry,
//         agent_label: String,
//     ) -> Self {

//         let name = format!("{}{}",&agent_label,&entry.label).to_string();
//         let id = Uuid::new_v5(&org_id, name.as_bytes());

//         let node = add_blob_entries::AgentWhere::new_agent(Some(agent_label.to_string()));

//         let agent_connect_where = add_blob_entries::AgentConnectWhere {
//             node: Box::new(node),
//         };

//         let agent = add_blob_entries::BlobEntryParentAgentFieldInput {
//             create: None,
//             connect_or_create: None,
//             connect: Some(add_blob_entries::BlobEntryParentAgentConnectFieldInput {
//                 connect: None,
//                 where_: Some(agent_connect_where),
//             })
//         };

//         let parent = add_blob_entries::BlobEntryParentCreateInput {
//             agent: Some(agent),
//             model: None,
//             factorgraph: None,
//             variable: None,
//             factor: None
//         };
            

//         Self {
//             id: id.to_string(),
//             origin_id: Some(entry.blobId.to_string()),
//             blob_id: entry.blobId.to_string(),
//             label: entry.label.to_string(),
//             blobstore: Some(entry.blobstore.to_string()),
//             origin: Some(entry.origin.to_string()),
//             description: Some(entry.description.to_string()),
//             mime_type: Some(entry.mimeType.to_string()),
//             hash: Some(entry.hash.to_string()),
//             size: entry.size,
//             metadata: Some(entry.metadata.to_string()),
//             timestamp: Some(entry.timestamp.to_string()),
//             // type_: entry._type.to_string(),
//             version: entry._version.to_string(),
//             parent: Some(parent),
//         }
//     }
// }
// 
// impl add_blob_entries::AgentWhere {
//     pub fn new_agent(
//         agent_label: Option<String>
//     ) -> Self {
//         Self {
//             id: None,
//             id_in: None,
//             id_contains: None,
//             id_starts_with: None,
//             id_ends_with: None,
//             org: Box::new(None),
//             org_aggregate: Box::new(None),
//             org_connection: Box::new(None),
//             org_connection_not: Box::new(None),
//             org_not: Box::new(None),
//             tags: None,
//             tags_includes: None,
//             version_ends_with: None,
//             version_starts_with: None,
//             version: None,
//             version_contains: None,
//             version_in: None,
//             and: Box::new(None),
//             not: Box::new(None),
//             or: Box::new(None),
//             blob_entries_aggregate: Box::new(None),
//             blob_entries_all: Box::new(None),
//             blob_entries_connection_all: Box::new(None),
//             blob_entries_connection_none: Box::new(None),
//             blob_entries_connection_single: Box::new(None),
//             blob_entries_connection_some: Box::new(None),
//             blob_entries_none: Box::new(None),
//             blob_entries_single: Box::new(None),
//             blob_entries_some: Box::new(None),
//             created_timestamp: None,
//             created_timestamp_gt: None,
//             created_timestamp_gte: None,
//             created_timestamp_in: None,
//             created_timestamp_lt: None,
//             created_timestamp_lte: None,
//             last_updated_timestamp: None,
//             last_updated_timestamp_gt: None,
//             last_updated_timestamp_gte: None,
//             last_updated_timestamp_lt: None,
//             last_updated_timestamp_lte: None,
//             last_updated_timestamp_in: None,
//             description: None,
//             description_contains: None,
//             description_ends_with: None,
//             description_in: None,
//             description_starts_with: None,
//             fgs_aggregate: Box::new(None),
//             fgs_all: Box::new(None),
//             fgs_single: Box::new(None),
//             fgs_connection_all: Box::new(None),
//             fgs_connection_single: Box::new(None),
//             fgs_connection_none: Box::new(None),
//             fgs_connection_some: Box::new(None),
//             fgs_none: Box::new(None),
//             fgs_some: Box::new(None),
//             label: agent_label,
//             label_in: None,
//             label_contains: None,
//             label_starts_with: None,
//             label_ends_with: None,
//             metadata: None,
//             metadata_in: None,
//             models_aggregate: Box::new(None),
//             models_all: Box::new(None),
//             models_none: Box::new(None),
//             models_some: Box::new(None),
//             models_single: Box::new(None),
//             models_connection_all: Box::new(None),
//             models_connection_none: Box::new(None),
//             models_connection_some: Box::new(None),
//             models_connection_single: Box::new(None),
//         }
//     }
// }









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
