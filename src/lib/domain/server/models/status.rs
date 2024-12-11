use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, PartialOrd, Ord, Type, Serialize, ToSchema)]
#[sqlx(type_name = "status")]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
    Offline,
    Waiting,
    Scheduled,
    Error,
    Running,
    Finished,
    Created,
    #[default]
    Unknown,
}

impl std::fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Offline => write!(f, "offline"),
            ServerStatus::Waiting => write!(f, "waiting"),
            ServerStatus::Error => write!(f, "error"),
            ServerStatus::Running => write!(f, "running"),
            ServerStatus::Finished => write!(f, "finished"),
            ServerStatus::Unknown => write!(f, "unknown"),
            ServerStatus::Scheduled => write!(f, "scheduled"),
            ServerStatus::Created => write!(f, "created"),
        }
    }
}

impl From<String> for ServerStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "offline" => ServerStatus::Offline,
            "waiting" => ServerStatus::Waiting,
            "error" => ServerStatus::Error,
            "running" => ServerStatus::Running,
            "finished" => ServerStatus::Finished,
            "scheduled" => ServerStatus::Scheduled,
            "created" => ServerStatus::Created,
            _ => ServerStatus::Unknown,
        }
    }
}

impl FromStr for ServerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "offline" => Ok(ServerStatus::Offline),
            "waiting" => Ok(ServerStatus::Waiting),
            "error" => Ok(ServerStatus::Error),
            "running" => Ok(ServerStatus::Running),
            "finished" => Ok(ServerStatus::Finished),
            "scheduled" => Ok(ServerStatus::Scheduled),
            "created" => Ok(ServerStatus::Created),
            _ => Err(format!("Unknown server_status: {}", s)),
        }
    }
}
