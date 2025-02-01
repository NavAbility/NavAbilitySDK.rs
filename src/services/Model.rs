
use crate::{
    Utc,
    Uuid,
    Sender,
    GraphQLQuery,
    QueryBody,
    Response,
    Error,
    SDK_VERSION,
    NavAbilityClient,
    BlobEntry,
    ListModels,
    list_models,
    AddModel,
    add_model,
    AddBlobEntryModel,
    add_blob_entry_model,
    ListModelsGraphs,
    list_models_graphs,
    check_deser,
    to_console_debug,
    to_console_error,
};



pub fn list_models_query(
    model_label_contains: Option<&str>,
) -> QueryBody<list_models::Variables> {
    let mut model_lbl_contains = Some("".to_string());
    if let Some(mt) = model_label_contains {
        model_lbl_contains = Some(mt.to_string());
    }

    let variables = list_models::Variables {
        label_contains: model_lbl_contains,
    };
    ListModels::build_query(variables)
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_models(
    nvacl: NavAbilityClient,
    model_label_contains: Option<&str>,
) -> Result<Response<list_models::ResponseData>, Box<dyn Error>> {
    
    let request_body = list_models_query(model_label_contains);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<list_models::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_model_async(
    nvacl: NavAbilityClient,
    model_label: &String,
) -> Result<Response<add_model::ResponseData>,Box<dyn Error>> {
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}",&model_label).to_string();
    let agent_id = Uuid::new_v5(&org_id, name.as_bytes());

    let variables = add_model::Variables {
        org_id: org_id.to_string(),
        model_id: agent_id.to_string(),
        label: model_label.to_string(),
        tags: Some(Vec::new())
    };

    let request_body = AddModel::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<add_model::ResponseData>(
        req_res?.json().await
    )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_entry_model_async(
    nvacl: NavAbilityClient,
    model_label: &String,
    entry: &BlobEntry,
) -> Result<Response<add_blob_entry_model::ResponseData>, Box<dyn Error>> {
    
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}{}",&model_label,&entry.label).to_string();
    let entry_id = Uuid::new_v5(&org_id, name.as_bytes());

    let mut size_s: Option<String> = None;
    if let Some(sz) = entry.size {
        size_s = Some(format!("{}",sz));
    }
    let mut metadata = entry.metadata.to_string();
    if metadata.is_empty() {
        metadata = "e30=".to_string();
    }

    let variables = add_blob_entry_model::Variables {
        model_label: model_label.to_string(),
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

    let request_body = AddBlobEntryModel::build_query(variables);

    let req_res = nvacl.client
        .post(&nvacl.apiurl)
        .json(&request_body)
        .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<add_blob_entry_model::ResponseData>(
        req_res?.json().await
    )
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_model_graphs(
    nvacl: NavAbilityClient,
    model_label_contains: Option<&str>,
) -> Result<Response<list_models_graphs::ResponseData>, Box<dyn Error>> {
    
    let mut model_lbl_contains = Some("".to_string());
    if let Some(mt) = model_label_contains {
        model_lbl_contains = Some(mt.to_string());
    }

    let variables = list_models_graphs::Variables {
        label_contains: model_lbl_contains,
    };
    let request_body = ListModelsGraphs::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return check_deser::<list_models_graphs::ResponseData>(
        req_res?.json().await
    )
}