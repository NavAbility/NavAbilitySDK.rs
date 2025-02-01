
use crate::{
    Uuid,
    Utc,
};

// unclear if manual definition of user robot session is necessary
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub label: String,
    pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}



// TBD: TO BE DEPRECATED? with SDK.jl v0.8
#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub label: String,
    pub robot_label: String,
    pub user_label: String,
    pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}
