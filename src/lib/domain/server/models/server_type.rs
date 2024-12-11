use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Type, Serialize, ToSchema)]
#[sqlx(type_name = "server_type")]
#[serde(rename_all = "snake_case")]
pub enum ServerType {
    Lobby,
    Hikabrain,
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::Hikabrain => write!(f, "hikabrain"),
            ServerType::Lobby => write!(f, "lobby"),
        }
    }
}

impl From<String> for ServerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hikabrain" => ServerType::Hikabrain,
            "lobby" => ServerType::Lobby,
            _ => ServerType::Hikabrain,
        }
    }
}

impl FromStr for ServerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hikabrain" => Ok(ServerType::Hikabrain),
            "lobby" => Ok(ServerType::Lobby),
            _ => Err(format!("Unknown server_type: {}", s)),
        }
    }
}
