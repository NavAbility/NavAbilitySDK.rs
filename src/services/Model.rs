
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    // Utc,
    Uuid,
    // Sender,
    GraphQLQuery,
    QueryBody,
    // Response,
    Error,
    // SDK_VERSION,
    NavAbilityClient,
    BlobEntry,
    ListModels,
    list_models,
    GetModel,
    AddModel,
    add_model,
    AddModelBlobEntry,
    add_model_blob_entry,
    ListModelsGraphs,
    list_models_graphs,
    GetId,
    // check_deser,
    // to_console_debug,
    // to_console_error,
    post_to_nvaapi,
    BlobEntrySummaryImporters,
    BlobEntryFieldsImporters
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
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
pub async fn post_list_models(
    nvacl: &NavAbilityClient,
    model_label_contains: Option<&str>,
) -> Result<list_models::ResponseData, Box<dyn Error>> {
    
    let request_body = list_models_query(model_label_contains);

    return post_to_nvaapi::<
        list_models::Variables,
        list_models::ResponseData,
        list_models::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}





#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_model_async(
    nvacl: &NavAbilityClient,
    model_label: &String,
) -> Result<add_model::ResponseData,Box<dyn Error>> {
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

    return post_to_nvaapi::<
        add_model::Variables,
        add_model::ResponseData,
        add_model::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(1)
    ).await;
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_list_model_graphs(
    nvacl: NavAbilityClient,
    mlabel: Option<&str>, // FIXME must exist
) -> Result<list_models_graphs::ResponseData, Box<dyn Error>> {
    
    // let label = mlabel.unwrap_or("").to_string();
    
    let variables = list_models_graphs::Variables {
        id: nvacl.getId(mlabel.unwrap_or("")).to_string(),
    };
    let request_body = ListModelsGraphs::build_query(variables);

    return post_to_nvaapi::<
        list_models_graphs::Variables,
        list_models_graphs::ResponseData,
        list_models_graphs::ResponseData
    >(
        &nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}


#[cfg(feature = "wasm")]
pub fn q_listModelGraphs(
  send_into: crate::Sender<list_models_graphs::ResponseData>, 
  nvacl: &NavAbilityClient,
  model: String,
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = (*nvacl).clone();
  // let send_into_ = send_into.clone();
  let model_ = model.to_string();
  crate::execute(async move {
    let _ = crate::send_api_result(
      send_into, 
      post_list_model_graphs(
        nvacl_, 
        Some(&model_)
      ).await,
    );
  });
}