use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::status::ServerStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ServerError {
    #[error("Server create error: {0}")]
    CreateError(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct Server {
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
