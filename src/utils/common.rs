
use graphql_client::GraphQLQuery;

use crate::{
    // Uuid,
    // Utc,
    Error, 
    Sender,
    Response,
};

use std::fmt;

pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn parse_str_utc(
    text: String
) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
    let tmstr = text
        .replace(" UTC"," +00")
        .replace("Z", " +00")
        .replace("T", " ");
    match chrono::DateTime::parse_from_str(&tmstr, "%Y-%m-%d %H:%M:%S%.f %#z") {
        Ok(tmsp) => {
            return Ok(tmsp.to_utc());
        }
        Err(e) => {
            to_console_error(&format!("Unable parse UTC datetime {} with error {:?}",text.to_string(),e));
            return Err(e)
        }
    }
}


pub fn to_console_debug(
    text: &str
) {
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}",text);
    // tracing::debug!("{}",text);
    #[cfg(target_arch = "wasm32")]
    gloo_console::log!(text.to_string());
}

pub fn to_console_error(
    text: &str
) {
    #[cfg(not(target_arch = "wasm32"))]
    println!("ERROR NvaSDK.rs {}",&text);
    // tracing::error!("ERROR NvaSDK.rs {}",&text);
    #[cfg(target_arch = "wasm32")]
    gloo_console::log!(&format!("ERROR NvaSDK.rs {}",&text));
}


// ===================== COMMON QUERY TOOLS ======================


#[derive(Debug)]
struct GQLResponseEmptyError {
    details: String,
}

impl fmt::Display for GQLResponseEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API response empty error {:?}", self.details)
    }
}

impl Error for GQLResponseEmptyError {}

#[derive(Debug)]
struct GQLResponseErrors {
    details: Vec<graphql_client::Error>,
}

impl fmt::Display for GQLResponseErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API response has error {:?}", self.details)
    }
}

impl Error for GQLResponseErrors {}


pub fn check_query_response_data<T>(
    response_body: Result<Response<T>,Box<dyn Error>>,
) -> Result<T,Box<dyn Error>> {
    match response_body {
        Ok(resbody) => {
            if resbody.errors.is_none() {
                match resbody.data {
                    Some(data) => {
                        return Ok(data)
                        // let _ = send_into.send(data);
                    }
                    None => {
                        to_console_error(&"API query response data is empty.");
                        return Err(Box::new(GQLResponseEmptyError {
                            details: "API post query error: response data is empty.".to_owned(),
                    }))
                    },
                }
            } else {
                to_console_error(&format!("API post query error: response errors exist: {:?}", &resbody.errors));
                return Err(Box::new(
                    GQLResponseErrors {
                        details: resbody.errors.unwrap()
                    }
                ));
            }
        }
        Err(e) => {
            to_console_error(&format!("Earlier query handling failure, unable to send: {:?}",&e));
            return Err(e);
        }
    }
}

// get_org::ResponseData
pub fn send_query_result<T>(
    send_into: Sender<T>,
    response_body: Result<Response<T>,Box<dyn Error>>,
) {
    match check_query_response_data::<T>(response_body) {
        Ok(data) => {
            let _ = send_into.send(data);
        },
        Err(e) => {
            // suppress panic
        }
    }
    // match response_body {
    //     Ok(resbody) => {
    //         if resbody.errors.is_none() {
    //             match resbody.data {
    //                 Some(data) => {
    //                     let _ = send_into.send(data);
    //                 }
    //                 None => to_console_error(&"API query response data is none."),
    //             }
    //         } else {
    //             to_console_error(&format!("API query responed with error: {:?}", &resbody.errors));
    //             // return Err(Box::new(response_body.errors));
    //         }
    //     }
    //     Err(e) => {
    //         to_console_error(&format!("Earlier query handling failure, unable to send: {:?}",&e));
    //     }
    // }
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn check_deser<T>(
    serde_res: Result<Response<T>,reqwest::Error>
) -> Result<Response<T>,Box<dyn Error>> {

    if let Err(ref e) = serde_res {
        to_console_error(&format!("JSON unpack failure from possibly good API response: {:?}", &e));
    }

    return Ok(serde_res?)
}


// ====================== FUTURE IDEAS ======================

// missing traits for generic serde on query types
// async fn post_query_serde<V,R>(
//     nvacl: NavAbilityClient,
//     request_body: QueryBody<V>,
// ) -> Result<R,Box<dyn Error>> {
//     let req_res = nvacl.client
//     .post(&nvacl.apiurl)
//     .json(&request_body)
//     .send().await;

//     match req_res {
//         Err(re) => {
//             to_console_error(&format!("API request error: {:?}", re));
//             return Err(Box::new(re));
//         },
//         Ok(res) => {
//             let serde_res = res.json().await;
//             match serde_res {
//                 Ok(response_body) => {
//                     return Ok(response_body)
//                 },
//                 Err(e) => {
//                     to_console_error(&format!("JSON unpack of API response failed: {:?}", &e));
//                     return Err(Box::new(e));
//                 }
//             }
//         }
//     }
// }