
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  GraphQLQuery,
  NavAbilityClient,
  Error,
  GQLRequestError,
  check_deser,
  check_query_response_data,
  to_console_error,
  StartWorker,
  // start_worker
};




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: &str,
  worker_label: crate::start_worker::mutationInput_post_startWorker_workerLabel
) -> Result<String, Box<dyn Error>> {
  
  use crate::GQLResponseEmptyError;
  let variables = crate::start_worker::Variables {
    input: input.to_string(),
    worker_label
  };
  
  let request_body = StartWorker::build_query(variables);
  
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
  let response_body = check_deser::<crate::start_worker::ResponseData>(
    req_res?.json().await
  );
  
  // unwrap ListAgents query response during error checks
  return check_query_response_data(response_body, |s| {
    return s.start_worker.unwrap_or("{}".to_owned());
  });
}







