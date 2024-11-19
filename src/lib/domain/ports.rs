use crate::domain::models::ServerStatus;
use std::future::Future;

pub trait ServerRepository {
    fn create_server(&self, address: String, status: ServerStatus) -> impl Future<Output = Result<(), anyhow::Error>>;
}
