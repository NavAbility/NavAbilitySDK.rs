
use regex::Regex;

use serde_json;
use serde::Serialize;

use reqwest_eventsource::{
  EventSource,
  Event,
};
use url::form_urlencoded;

use futures::stream::StreamExt;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Uuid,
  GraphQLQuery,
  QueryBody,
  Error,
  NavAbilityClient,
  post_to_nvaapi,
  StartWorker,
  to_console_debug, 
  to_console_error,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn start_worker_query(
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> QueryBody<crate::start_worker::Variables> {

  // let res = serde_json::from_str::<serde_json::Map<String,serde_json::Value>>(input).unwrap();
  return StartWorker::build_query(
    crate::start_worker::Variables {
      input,
      worker_label
    }
  );
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<Uuid, Box<dyn Error>> {
  
  let request_body = start_worker_query(input, worker_label);
  
  let bad_json_on_resp = post_to_nvaapi::<
    crate::start_worker::Variables,
    crate::start_worker::ResponseData,
    crate::start_worker::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;

  match bad_json_on_resp {
    Ok(res) => {
      let rstr = &res.start_worker.unwrap();
      let idstr = rstr["id"].as_str().expect(&format!("Unable to estract 'id' from post_start_worker response {:?}",&rstr["id"]));
      let id = Uuid::parse_str(&idstr).map_err(|e_| Box::new(e_) as Box<dyn Error>);
      return id;
    },
    Err(e) => {
      return Err(e);
    }
  }
}



#[cfg(any(feature = "tokio", feature = "thread"))]
pub fn startWorker(
  nvacl: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<Uuid, Box<dyn Error>> {
  return crate::execute(post_start_worker(
    nvacl,
    input,
    worker_label
  ));
}
    
#[cfg(feature = "wasm")]
pub fn startWorker(
  nvacl_: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label_: crate::start_worker::WorkerLabelEnum
) {

  let nvacl = nvacl_.clone();
  // let input = input_.to_string();
  let worker_label = worker_label_;
  return crate::execute(async move {
      let _ = post_start_worker(
      &nvacl,
      input,
      worker_label
    ).await;
  });
}



#[cfg(any(feature = "tokio", feature = "wasm"))]
pub fn subscription_listener(
    send_into: crate::Sender<crate::default_subscription::ResponseData>,
    nvacl: &NavAbilityClient,
) {
  // FIXME upgrade to eventsource_reqwest for sse
  let nvacl_e = NavAbilityClient::similar(
    nvacl,
    true
  );
  let query = crate::DefaultSubscription::build_query(
    crate::default_subscription::Variables {}
  );
  // https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/
  let query_string = form_urlencoded::byte_serialize(query.query.as_bytes())
    .collect::<String>();
  // let query_string = "subscription%7BworkerEvent%7Bpayload+id+status%7D%7D";
  let uri = format!("{}?query={}", nvacl.apiurl, query_string);

  let mut nvaes = EventSource::new(
    nvacl_e.client.get(&uri)
  ).expect("Failed to create EventSource");

  // wasmbindgen limitation?  overcome +'static requirement
  crate::execute(async move {
    while let Some(event) = nvaes.next().await {
        match event {
            Ok(Event::Open) => to_console_debug("SSE connection Open!"),
            Ok(Event::Message(message)) => {
              let msg_: Result<
                serde_json::Map<String, serde_json::Value>,
                serde_json::Error
              > = serde_json::from_str(&message.data);
              if let Err(e) = msg_ {
                to_console_error(&format!("Failed to parse message data: {}", e));
                continue;
              }
              let msg = msg_.unwrap();
              if let Some(jobj) = msg.get("data") { //msg.contains_key("data") {
                // to_console_debug(&format!("{:?}",&msg));
                let jstr = jobj.to_string();
                let jobj_: Result<crate::default_subscription::ResponseData, serde_json::Error> = serde_json::from_str(&jstr);
                if let Ok(subwe) = jobj_ {
                  send_into.send(
                    subwe
                  ).expect("Failed to send Event");
                } else {
                  to_console_error("Failed to parse 'data' from message data");
                }
              }
            },
            Err(err) => {
                to_console_error(&format!("Error: {}", err));
                nvaes.close();
            }
        }
    }
  });
}
