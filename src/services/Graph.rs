

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Utc,
  Uuid,
  Sender,
  GraphQLQuery,
  Response,
  Error,
  NavAbilityClient,
  post_to_nvaapi,
  ListGraphs,
  list_graphs,
  AddFactorgraph,
  // add_factorgraph,
  check_deser,
  to_console_debug,
  to_console_error,
  SDK_VERSION
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_graphs(
  nvacl: &NavAbilityClient,
  id: &Uuid,
) -> Result<list_graphs::ResponseData, Box<dyn Error>> {
  
  let request_body = ListGraphs::build_query(list_graphs::Variables {
    id: id.to_string()
  });
  
  return post_to_nvaapi::<
    list_graphs::Variables,
    list_graphs::ResponseData,
    list_graphs::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_factorgraph(
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<crate::add_factorgraph::ResponseData, Box<dyn Error>> {
  
  let oid = Uuid::parse_str(&nvacl.user_label).expect("cannot parse org_id");
  let request_body = AddFactorgraph::build_query(crate::add_factorgraph::Variables {
    org_id: Some(nvacl.user_label.to_string()),
    id: Some(Uuid::new_v5(&oid, label.as_bytes()).to_string()),
    label: Some(label.to_string()),
    description: Some(description.to_string()),
    metadata: Some(metadata.to_string()),
    version: Some(SDK_VERSION.to_string()),
  });
  
  return post_to_nvaapi::<
    crate::add_factorgraph::Variables,
    crate::add_factorgraph::ResponseData,
    crate::add_factorgraph::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_factorgraph_send(
  send_into: std::sync::mpsc::Sender<crate::add_factorgraph::ResponseData>, //get_blob_entry::ResponseData>,
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<(), Box<dyn Error>> {
  return crate::send_api_result(
    send_into, 
    post_add_factorgraph(nvacl, label, description, metadata).await,
  );
}



#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn addFactorgraph(
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<crate::add_factorgraph::ResponseData, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(
    post_add_factorgraph(
      nvacl,
      label,
      description,
      metadata
    )
  );
}