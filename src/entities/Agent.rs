
use crate::{
    Uuid,
    Utc,
};


#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Option<Uuid>,
    pub label: String,
    // pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}

