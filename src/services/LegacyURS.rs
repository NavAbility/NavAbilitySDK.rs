
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::{
    GraphQLQuery,
    Response,
    Error,
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




