#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct Server {
    pub id: uuid::Uuid,
    pub address: String,
    pub status: ServerStatus,
}
