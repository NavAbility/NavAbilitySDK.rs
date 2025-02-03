
/// Utility functions and common tools for the NavAbility SDK.
///
/// This module provides various utility functions and common tools used throughout the NavAbility SDK,
/// including functions for type introspection, string parsing, console logging, and handling GraphQL query responses.

// use graphql_client::GraphQLQuery;
use crate::{
    Error, 
    Sender, 
    Response
};
use std::{
    fmt,
    convert::TryInto,
};

/// Returns the type name of a given value.
///
/// # Arguments
///
/// * `_: T` - A value of any type.
///
/// # Returns
///
/// * `&'static str` - The name of the type of the given value.
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

/// Parses a string into a `chrono::DateTime<Utc>` object.
///
/// # Arguments
///
/// * `text` - A string representing a date and time in UTC.
///
/// # Returns
///
/// * `Result<chrono::DateTime<chrono::Utc>, chrono::ParseError>` - A `Result` containing the parsed `DateTime` object or a `ParseError`.
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

fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Logs a debug message to the console.
///
/// # Arguments
///
/// * `text` - A string slice containing the debug message.
pub fn to_console_debug(
    text: &str
) {
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}",text);
    // tracing::debug!("{}",text);
    #[cfg(target_arch = "wasm32")]
    gloo_console::log!(text.to_string());
}

/// Logs an error message to the console.
///
/// # Arguments
///
/// * `text` - A string slice containing the error message.
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


/// Checks the response data of a GraphQL query.
///
/// # Arguments
///
/// * `response_body` - A `Result` containing the response body of the GraphQL query.
///
/// # Returns
///
/// * `Result<T, Box<dyn Error>>` - A `Result` containing the data of the response or an error.
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

/// Sends the result of a GraphQL query to a given sender.
///
/// # Arguments
///
/// * `send_into` - A sender to which the query result will be sent.
/// * `response_body` - A `Result` containing the response body of the GraphQL query.
pub fn send_query_result<F,T>(
    send_into: Sender<T>,
    response_body: Result<Response<F>,Box<dyn Error>>,
    fn_modifier: fn(F) -> T,
) -> Result<(),Box<dyn Error>> {
    match check_query_response_data::<>(response_body) {
        Ok(data) => {
            // let _ = send_into.send(data);
            if let Err(e) = send_into.send(fn_modifier(data)) {
                to_console_error(&format!("Error sending data on channel: {:?}", e));
            };
            return Ok(())
        },
        Err(e) => {
            return Err(e)
        }
    }
}

/// Checks the deserialization result of a GraphQL query response.
///
/// # Arguments
///
/// * `serde_res` - A `Result` containing the deserialization result of the response.
///
/// # Returns
///
/// * `Result<Response<T>, Box<dyn Error>>` - A `Result` containing the deserialized response or an error.
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