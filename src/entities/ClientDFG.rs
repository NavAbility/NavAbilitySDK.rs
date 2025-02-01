
use std::collections::HashMap;

use crate::{
    Agent,
    NvaNode,
    Factorgraph,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    Client,
    NavAbilityBlobStore,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct NavAbilityClient {
    pub client: Client,
    pub apiurl: String,
    pub user_label: String,
    pub nva_api_token: String,
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct NavAbilityDFG<'a> {
    pub client: NavAbilityClient,
    pub fg: NvaNode<'a,Factorgraph>,
    pub agent: NvaNode<'a, Agent>,
    pub blobStores: HashMap<String, NavAbilityBlobStore>,
}