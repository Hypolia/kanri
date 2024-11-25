use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::status::ServerStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ServerError {
    #[error("Server create error: {0}")]
    CreateError(String),
    #[error("Server id is invalid: {0}")]
    InvalidId(String),
    #[error("Server not found")]
    NotFound,
}

#[derive(Debug, Clone, FromRow, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct Server {
    #[serde(serialize_with = "uuid::serde::simple::serialize")]
    pub id: uuid::Uuid,
    pub name: String,
    pub player_count: i32,
    pub max_player_count: i32,
    pub server_type: ServerType,
    pub status: ServerStatus,
    pub address: String,
    //pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub description: Option<String>,
    pub region: Option<String>,
    pub created_by: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerCreatedMessage {
    pub server_id: String,
    pub name: String,
    pub server_type: String,
}
