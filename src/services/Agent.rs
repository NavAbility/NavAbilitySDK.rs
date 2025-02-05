
use crate::{
    Uuid,
    Utc,
    Agent,
};

use std::collections::HashMap;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[macro_use]
use crate::{
    Sender,
    GraphQLQuery,
    Response,
    Error,
    SDK_VERSION,
    BlobEntry,
    to_console_error,
    parse_str_utc,
    NavAbilityClient,
    NvaNode,
    Factorgraph,
    Model,
    check_deser,
    send_query_result,
    send_api_response,
    check_query_response_data,
    AddAgent,
    add_agent,
    GetAgents, // query vs fn, unique crate::get_agents,
    ListAgents, // query vs fn, unique crate::post_list_agents,
    AgentFieldImportersSummary,
    Agent_importers_summary,
    AgentFieldImportersFull,
    Agent_importers_full,
    get_agent_entries_metadata,
    GetAgentEntriesMetadata,
    AddBlobEntryAgent,
    // add_blob_entry_agent,
    GQLRequestError,
};

#[cfg(feature = "wasm")]
use crate::to_console_debug;


// ===================== HELPERS ========================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::get_agents::agent_fields_summary as GA_AgentFieldsSummary;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
Agent_importers_summary!(GA_AgentFieldsSummary);
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::get_agents::agent_fields_full as GA_AgentFieldsFull;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
Agent_importers_full!(GA_AgentFieldsFull);


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl Agent {
    pub fn from_gql_summary(
        aggql: &impl AgentFieldImportersSummary,
    ) -> Self {
        let mut ag = Agent::default();
        ag.id = aggql.id();
        ag.label = aggql.label();
        ag.description = aggql.description();
        ag._version = aggql._version();
        ag.createdTimestamp = aggql.createdTimestamp();
        ag.lastUpdatedTimestamp = aggql.lastUpdatedTimestamp();

        return ag
    }

    pub fn from_gql_full(
        aggql: &impl AgentFieldImportersFull,
        ag: &mut Self,
    ) {
        ag.metadata = aggql.metadata();
        ag.blobEntries = aggql.blobEntries();
        ag.models = aggql.models();
        ag.fgs = aggql.fgs();
        
        return ();
    }
}


// ===================== QUERIES ========================


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_list_agents(
    nvacl: &NavAbilityClient,
) -> Result<Vec<String>, Box<dyn Error>> {
    // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
    let variables = crate::list_agents::Variables {
        org_id: nvacl.user_label.to_string(),
    };

    let request_body = ListAgents::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        let erm = format!("API request error: {:?}", &re);
        to_console_error(&erm);
        return Err(Box::new(GQLRequestError { details: erm }));
    }

    // generic transport and serde error checks
    let response_body = check_deser::<crate::list_agents::ResponseData>(
        req_res?.json().await
    );

    // unwrap ListAgents query response during error checks
    return check_query_response_data(response_body, |s| {
        let mut ags = Vec::new();
        for oa in s.orgs {
            for a in oa.agents {
                ags.push(a.label);
            }
        }
        return ags;
    });
}


#[cfg(feature = "tokio")]
pub fn listAgents(
    nvacl: &NavAbilityClient,
) -> Result<Vec<String>, Box<dyn Error>> {
    return tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(post_list_agents(nvacl));
}

// FIXME update to newer pattern without requiring separate wasm config
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[allow(non_snake_case)]
pub async fn listAgents_send(
    send_into: Sender<Vec<String>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> {
    // use common send_query_result
    return send_api_response(
        send_into, 
        post_list_agents(&nvacl).await?,
    );
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_get_agents(
    nvacl: &NavAbilityClient,
) -> Result<Vec<Agent>, Box<dyn Error>> {

    // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
    let variables = crate::get_agents::Variables {
        org_id: nvacl.user_label.to_string(),
        full: Some(true)
    };

    let request_body = GetAgents::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    // FIXME change to use common error handling before attempting to json deserialize
    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    let response_body = check_deser::<crate::get_agents::ResponseData>(
        req_res?.json().await
    );

    // unwrap ListAgents query response during error checks
    return check_query_response_data::<
        crate::get_agents::ResponseData,
        Vec<Agent>
    >(
        response_body, 
        |s| {
            let mut ags = Vec::new();
            for a in s.agents {
                let mut agent = Agent::from_gql_summary(&a.agent_fields_summary);
                Agent::from_gql_full(&a.agent_fields_full, &mut agent);
                ags.push(agent);
            };
            return ags;
        }
    );
}


#[cfg(feature = "tokio")]
pub fn getAgents(
    nvacl: &NavAbilityClient
) -> Result<Vec<Agent>, Box<dyn Error>> {
    return tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(post_get_agents(nvacl));
}


#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn getAgents_send(
    send_into: Sender<Vec<Agent>>,
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> {
    return send_api_response(
        send_into, 
        post_get_agents(nvacl).await?,
    );
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_agent(
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



// ------------------------ Agent Entries Metadata ------------------------

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_get_agent_entries_metadata(
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



// FIXME return Uuid (not string)
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_entry_agent(
    nvacl: &NavAbilityClient,
    agent_label: &String,
    entry: &BlobEntry,
    _legacy: Option<String>,
) -> Result<String,Box<dyn Error>> {
    //) -> Result<Response<crate::add_blob_entry_agent::ResponseData>, Box<dyn Error>> {
    
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

    let variables = crate::add_blob_entry_agent::Variables {
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

    let request_body = AddBlobEntryAgent::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    let response_body = check_deser::<crate::add_blob_entry_agent::ResponseData>(
        req_res?.json().await
    );

    return check_query_response_data(response_body, |s| {
        s.add_blob_entries.blob_entries[0].id.clone()
    });
}

// let gqlentry = add_blob_entries_agent::BlobEntryCreateInput::new(
//     org_id,
//     entry,
//     agent_label.to_string(),
// );
// let mut blob_entries = Vec::new();
// blob_entries.push(gqlentry);
// let variables = add_blob_entries_agent::Variables {
//     blob_entries,
// };


#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn post_get_blob_entry_send(
    send_into: std::sync::mpsc::Sender<String>,
    nvacl: &NavAbilityClient,
    agent_label: &String,
    entry: &BlobEntry,
) -> Result<(),Box<dyn Error>> {
    
    return send_api_response(
        send_into, 
        post_add_entry_agent(
            nvacl, 
            agent_label,
            entry,
            None
        ).await?,
    );
}


#[cfg(feature = "tokio")]
pub fn addAgentBlobEntry(
    nvacl: &NavAbilityClient,
    agent_label: &String,
    entry: &BlobEntry,
) -> Result<String, Box<dyn Error>> {
    return tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(post_add_entry_agent(
            nvacl,
            agent_label,
            entry,
            None // legacy
        ));
}

