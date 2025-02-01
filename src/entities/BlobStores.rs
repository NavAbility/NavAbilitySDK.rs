
use crate::{
    NavAbilityClient
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(Debug, Clone)]
pub struct NavAbilityBlobStore {
    pub client: NavAbilityClient,
    pub label: String,
}