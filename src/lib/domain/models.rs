#[derive(Debug, Clone)]
pub enum ServerStatus {
    Offline,
    Waiting,
    Error,
    Running,
    Finished,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub id: String,
    pub address: String,
    pub status: ServerStatus,
}
