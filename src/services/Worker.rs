
use serde_json;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  // Serialize,
  GraphQLQuery,
  QueryBody,
  Error,
  // GQLRequestError,
  // GQLResponseEmptyError,
  // check_deser,
  // check_query_response_data,
  // to_console_error,
  NavAbilityClient,
  post_to_nvaapi,
  StartWorker, // start_worker
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: &str,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<String, Box<dyn Error>> {
    use crate::to_console_error;

  
  let variables = crate::start_worker::Variables {
    input: input.to_string(),
    worker_label
  };
  
  let request_body = StartWorker::build_query(variables);
  
  return post_to_nvaapi::<
    crate::start_worker::Variables,
    crate::start_worker::ResponseData,
    String
  >(
    nvacl,
    request_body, 
    |s| {
      // let sm = serde_json::from_str(s).unwrap();
      if let Some(sw) = &s.start_worker {
        match serde_json::from_str::<serde_json::Map<String,serde_json::Value>>(sw) {
          Ok(res) => {
            //FIXME, make more robust -- ensure fields are present etc.
            return res["id"].to_string();
          },
          Err(e) => {
            to_console_error(&format!("start worker parse response error: {:?}", e));
            return "".to_string();
          }
        }
      }
      return "".to_string();
    },
    Some(1)
  ).await;
}







