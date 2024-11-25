use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Type, Serialize)]
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