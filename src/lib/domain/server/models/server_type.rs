use serde::Deserialize;
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize, Type)]
#[sqlx(type_name = "server_type")]
#[serde(rename_all = "snake_case")]
pub enum ServerType {
    Hikabrain,
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::Hikabrain => write!(f, "hikabrain"),
        }
    }
}

impl From<String> for ServerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hikabrain" => ServerType::Hikabrain,
            _ => ServerType::Hikabrain,
        }
    }
}

impl FromStr for ServerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hikabrain" => Ok(ServerType::Hikabrain),
            _ => Err(format!("Invalid server type: {}", s)),
        }
    }
}
