use serde::Deserialize;
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, PartialOrd, Ord, Type)]
#[sqlx(type_name = "status")]
pub enum ServerStatus {
    Offline,
    Waiting,
    Error,
    Running,
    Finished,
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
            _ => Err(format!("Invalid server status: {}", s)),
        }
    }
}
