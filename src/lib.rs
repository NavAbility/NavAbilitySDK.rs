

use std::{
    error::Error, 
    sync::mpsc::Sender,
    // collections::HashMap,
};
use serde::Serialize;
use uuid::Uuid;
use chrono::{
    self, 
    Utc
};

use graphql_client::{
    GraphQLQuery, 
    QueryBody, 
    Response
};


#[cfg(any(feature = "tokio", feature = "wasm"))]
use reqwest::Client;
#[cfg(feature="blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature="blocking")]
use graphql_client::reqwest::post_graphql_blocking;


#[cfg(target_arch = "wasm32")]
use gloo_console::{
    __macro::JsValue, 
    log
};
// #[cfg(feature="wasm")]
// use reqwest::multipart::Part; // requires multipart

#[cfg(feature = "tokio")]
use tokio;

pub mod entities;
pub use crate::entities::*;
// type and file name are the same and requires precision import
pub use crate::Agent::Agent;
pub use crate::BlobEntry::BlobEntry;

pub mod utils;
pub use crate::utils::*;


pub mod services;
pub use crate::services::{
    fetch_robots_async,
    fetch_blob_entry,
    fetch_delete_blob,
    send_blob_entry,
    create_upload_async,
    create_download_async,
    send_create_download,
    complete_upload_async,
    fetch_org_id,
    fetch_delete_blobentry,
    add_agent_async,
    update_blobentry_metadata_async,
    fetch_agent_entries_metadata,
    add_entry_agent_async,
    // fetch_robots_async,
    list_models_query,
    fetch_list_models,
    add_model_async,
    add_entry_model_async,
    fetch_list_model_graphs,
    fetch_list_graphs,
};

// #[cfg(feature = "wasm")]
// pub use crate::services::{
//     fetch_ur_list_web,
//     create_upload_web,
//     fetch_context_web,
// };

#[cfg(feature = "tokio")]
pub use crate::services::{
    fetch_ur_list_tokio,
};

pub mod deprecated;
pub use crate::deprecated::*;

// FFI for publishing C compliant library 
#[cfg(feature = "tokio")]
mod capi;
#[cfg(feature = "tokio")]
use crate::capi::*;


const SDK_VERSION: &str = "0.25";


type EmailAddress = String;
type DateTime = String;
type Metadata = String;
type BigInt = String;
type B64JSON = String;
type Latitude = f64;
type Longitude = f64;
type UUID = String;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/GetAgents.gql",
    response_derives = "Debug"
)]
pub struct GetAgents;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/GetURS.gql",
    response_derives = "Debug"
)]
pub struct GetURS;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/ListModels.gql",
    response_derives = "Debug"
)]
pub struct ListModels;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/ListModelsGraphs.gql",
    response_derives = "Debug"
)]
pub struct ListModelsGraphs;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/GetAgentEntriesMetadata.gql",
    response_derives = "Debug"
)]
pub struct GetAgentEntriesMetadata;

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
    query_path = "src/gql/CreateUpload.gql",
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
    query_path = "src/gql/GetBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct GetBlobEntry;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/GetVariable.gql",
    response_derives = "Debug"
)]
pub struct GetVariable;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/ListVariables.gql",
    response_derives = "Debug"
)]
pub struct ListVariables;
// Implicit ListWhere due to graphql-client limitation: https://github.com/graphql-rust/graphql-client/issues/508

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
    query_path = "src/gql/AddBlobEntryModel.gql",
    response_derives = "Debug"
)]
pub struct AddBlobEntryModel;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/DeleteBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct DeleteBlobEntry;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/DeleteBlob.gql",
    response_derives = "Debug"
)]
pub struct DeleteBlob;

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
    query_path = "src/gql/ListGraphs.gql",
    response_derives = "Debug"
)]
pub struct ListGraphs;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/AddAgent.gql",
    response_derives = "Debug"
)]
pub struct AddAgent;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/AddModel.gql",
    response_derives = "Debug"
)]
pub struct AddModel;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/gql/UpdateBlobentryMetadata.gql",
    response_derives = "Debug"
)]
pub struct UpdateBlobentryMetadata;


// ===================== traits =========================

pub trait QueryDetails<Q: Serialize> {
    fn operation_name(&self) -> &str;
    fn query(&self) -> String;
    fn variables_jstr(&self) -> Result<String,serde_json::Error>;
    fn to_jstr(&self) -> String;
}

impl<Q: Serialize> QueryDetails<Q> for graphql_client::QueryBody<Q> {
    fn operation_name(&self) -> &str { 
        self.operation_name 
    }

    fn query(&self) -> String { 
        self.query.replace("\n","").replace("\"","\\\"") 
    }
    
    fn variables_jstr(&self) -> Result<String,serde_json::Error> { 
        serde_json::to_string(&self.variables)
    }

    fn to_jstr(&self) -> String {
        format!(
            r#"{{"extensions": {}, "operationName": "{}", "query": "{}", "variables": {}}}"#, 
            "{}",
            self.operation_name(),
            self.query(),
            self.variables_jstr().unwrap_or("".to_owned()),
        )
    }
}



#[allow(non_snake_case)]
pub trait GetLabel {
    fn getLabel(&self) -> &String;
}

// helper macro to avoid repetition of "basic" impl Coordinates
// #[macro_export]
macro_rules! genGetLabel { 
    ($T:ident) => {
        impl GetLabel for $T {
            fn getLabel(&self) -> &String { &self.label }
        }
    }
}


genGetLabel!(User);
// getLabel!(Graph);
genGetLabel!(BlobEntry);
// TO BE DEPRECATED
genGetLabel!(Session);
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
genGetLabel!(NavAbilityBlobStore);

// move to services
genGetLabel!(Agent);


pub trait GetId {
    /// Get the deterministic identifier (uuid v5) for a node.
    #[allow(non_snake_case)]
    fn getId(
        &self,
        label: &str,
    ) -> Uuid;
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl<T> GetId for NvaNode<'_, T> {
    fn getId(
        &self, 
        label: &str
    ) -> Uuid {
        let ostr = self.label.clone();
        return Uuid::new_v5(
            &self.namespace, 
            (&(ostr + label)).as_bytes()
        )
    }
}





// ================================= TESTS ===============================



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_robots() {
        // parse datetime example 1
        let text = "2024-09-03 02:31:39.367 UTC";
        let _res = parse_str_utc(text.to_owned());
        println!("parse_str_utc {:?}",_res);
        let _ = _res.unwrap(); // make sure the conversion worked
        // parse datetime example 2
        let text = "2024-10-28T20:56:35.270Z";
        let _res = parse_str_utc(text.to_owned());
        println!("parse_str_utc {:?}",_res);
        let _ = _res.unwrap(); // make sure the conversion worked


        let nva_userlabel: String = "test@wherewhen.ai".to_owned();
            // std::env::var("NAVABILITY_USERLABEL").expect("Missing NAVABILITY_USERLABEL env var");

        let nva_api_token: String = "".to_owned();
            // std::env::var("NAVABILITY_API_TOKEN").expect("Missing NAVABILITY_API_TOKEN env var");

        let api_url: &str = "https://api.navability.io/graphql";
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
