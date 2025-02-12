
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Serialize,
  GraphQLQuery,
  QueryBody,
  NavAbilityClient,
  Error,
  GQLRequestError,
  GQLResponseEmptyError,
  check_deser,
  check_query_response_data,
  to_console_error,
  print_type_of,
  StartWorker,
  // start_worker
};

use serde::Deserialize;



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_to_nvaapi<
  Q: Serialize,
  R: for<'de> Deserialize<'de>,
  T
>(
  nvacl: &NavAbilityClient,
  request_body: Q,
  fn_modifier: fn(R) -> T
) -> Result<T, Box<dyn Error>> {
    use serde::{Deserialize, Serialize};

  let req_res = nvacl.client
  .post(&nvacl.apiurl)
  .json(&request_body)
  .send().await;
  
  if let Err(ref re) = req_res {
    let erm = format!("API request error for {:?}: {:?}",  print_type_of(&request_body), &re);
    to_console_error(&erm);
    return Err(Box::new(GQLRequestError { details: erm }));
  }
  
  // generic transport and serde error checks
  let response_body = check_deser::<R>(
    req_res?.json().await
  );
  
  // unwrap StartWorker query response during error checks
  return check_query_response_data(response_body, fn_modifier);
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: &str,
  worker_label: crate::start_worker::mutationInput_post_startWorker_workerLabel
) -> Result<String, Box<dyn Error>> {
  
  let variables = crate::start_worker::Variables {
    input: input.to_string(),
    worker_label
  };
  
  let request_body = StartWorker::build_query(variables);
  
  return post_to_nvaapi::<
    crate::QueryBody<crate::start_worker::Variables>,
    crate::start_worker::ResponseData,
    String
  >(
    nvacl,
    request_body, 
    |s|{
      return s.start_worker.unwrap_or("{}".to_owned());
    }
  ).await;
}







