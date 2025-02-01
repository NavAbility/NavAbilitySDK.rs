
use crate::{
    Uuid,
    Utc,
    Sender,
    GraphQLQuery,
    Response,
    Error,
    SDK_VERSION,
    Agent,
    BlobEntry,
    AddAgent,
    add_agent,
    GetAgents,
    get_agents,
    get_agent_entries_metadata,
    GetAgentEntriesMetadata,
    AddBlobEntries,
    add_blob_entries,
    to_console_debug,
    to_console_error,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    NavAbilityClient,
    check_deser,
    send_query_result,
};


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


// ===================== QUERIES ========================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_agent_async(
    nvacl: &NavAbilityClient,
    agent_label: &String,
) -> Result<Response<add_agent::ResponseData>,Box<dyn Error>> {
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}",&agent_label).to_string();
    let agent_id = Uuid::new_v5(&org_id, name.as_bytes());

    let variables = add_agent::Variables {
        agent_id: agent_id.to_string(),
        label: agent_label.to_string(),
        version: SDK_VERSION.to_string(),
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



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
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
    }); //.unwrap().data;

    // TODO use common send_query_result -- TBD data.agents changes type
    // send_query_result(send_into, ur_list_data);
    match ur_list_data {
        Ok(ur_data) => match ur_data.data { 
            Some(data) => {
                let ur_list = data.agents;
                if let Err(e) = send_into.send(ur_list) {
                    to_console_error(&format!("Error sending user robot list data: {:?}", e));
                };
                return Ok(())
            },
            None => {
                    let estr = format!("GQL errors {:?}\n",ur_data.errors);
                    to_console_error(&estr);
                    panic!("{}", estr);
                }
        },
        Err(e) => {
            panic!("Something went wrong {:?}", e);
        }
    }
}


// FIXME FIXME FIXME update to newer pattern without requiring separate wasm config
#[cfg(target_arch = "wasm32")]
pub async fn fetch_ur_list_web(
    send_into: Sender<Vec<get_agents::GetAgentsAgents>>, 
    nvacl: &NavAbilityClient
) { // -> Vec<get_robots::GetRobotsUsers> {      
    let result = fetch_robots_async(&nvacl).await;
    // FIXME use common send_query_result
    // send_query_result::<get_robots::GetAgentsAgents>(send_into, result);
    if let Ok(response_body) = result {
        let res_errs = response_body.errors;
        match res_errs {
            Some(ref err) => {
                to_console_error(&format!("NvaSDK.rs fetch_ur_list_web has response errors {:?}",&res_errs));
            },
            None => {
                let ur_list_data = response_body.data;
                match ur_list_data {
                    None => to_console_debug("NvaSDK.rs bad GQL response"),
                    Some(resdata) => {
                        let ur_list = resdata.agents;
                        let res_len = ur_list.len();
                        let resp = send_into.send(ur_list);
                        if let Err(e) = resp {
                            to_console_error(&format!("Error sending user robot list data: {}", e));
                        }
                    }
                }
            }
        }
    } else {
        to_console_error("fetch_robots_async(&nvacl).await");
    }

}


// ------------------------ Agent Entries Metadata ------------------------

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_agent_entries_metadata(
    nvacl: NavAbilityClient,
    agent_label: String,
    mime_type: Option<String>
) -> Result<Response<get_agent_entries_metadata::ResponseData>, Box<dyn Error>> {
    
    let mut mime_type_contains = Some("".to_string());
    if let Some(mt) = mime_type {
        mime_type_contains = Some(mt.to_string());
    }

    let variables = get_agent_entries_metadata::Variables {
        org_id: nvacl.user_label.to_string(),
        agent_label: agent_label.to_string(),
        mime_type_contains,
    };
    let request_body = GetAgentEntriesMetadata::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<get_agent_entries_metadata::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_entry_agent_async(
    nvacl: NavAbilityClient,
    agent_label: &String,
    entry: &BlobEntry,
    _legacy: Option<String>,
) -> Result<Response<add_blob_entries::ResponseData>, Box<dyn Error>> {
    
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}{}",&agent_label,&entry.label).to_string();
    let entry_id = Uuid::new_v5(&org_id, name.as_bytes());

    let mut size_s: Option<String> = None;
    if let Some(sz) = entry.size {
        size_s = Some(format!("{}",sz));
    }
    let mut metadata = entry.metadata.to_string();
    if metadata.is_empty() {
        metadata = "e30=".to_string();
    }

    let variables = add_blob_entries::Variables {
        agent_label: agent_label.to_string(),
        entry_id: entry_id.to_string(),
        entry_label: entry.label.to_string(),
        blob_id: entry.blobId.to_string(),
        blobstore: Some(entry.blobstore.to_string()),
        origin: Some(entry.origin.to_string()),
        mime_type: Some(entry.mimeType.to_string()),
        metadata: metadata,
        description: Some(entry.description.to_string()),
        hash: entry.hash.to_string(),
        size: size_s,
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


