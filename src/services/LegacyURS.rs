
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::{
    // Utc,
    // Uuid,
    // Sender,
    GraphQLQuery,
    Response,
    Error,
    // SDK_VERSION,
    GetURS,
    get_urs,
    to_console_error,
};

#[cfg(feature = "wasm")]
use crate::to_console_debug;

#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::NavAbilityClient;


#[cfg(feature = "blocking")]
pub fn get_robots_blocking(client: &NavAbilityClient) -> get_robots::ResponseData {
    let variables = get_robots::Variables {
        user_label: client.user_label.clone(),
    };

    let response_body =
        post_graphql_blocking::<GetRobots, _>(&client.client, &client.apiurl, variables)
            .expect("Failure to post graphql");
    
    //debug print raw response body
    dbg!(&response_body);

    let response_data: get_robots::ResponseData =
        response_body.data.expect("missing response data");

    return response_data;
}



#[cfg(feature = "blocking")]
pub fn fetch_ur_list_blocking(
    send_into: &mut crate::Sender<Vec<get_robots::GetRobotsUsers>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> {

    // THIS IS THE LEGACY VERSION IN SDK FIXME TO NEW VERSION FOR WEB/WASM
    let ur_list = get_robots_blocking(&nvacl).users;
    // dbg!(&ur_list);

    if let Err(e) = send_into.send(ur_list) {
        tracing::error!("Error sending user robot list data: {}", e);
    };

    Ok(())
}


