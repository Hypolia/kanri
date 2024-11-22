use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_validator::CreateServer;
use std::future::Future;

pub trait ServerService: Clone + Send + Sync + 'static {
    fn create_server(
        &self,
        server: CreateServer,
    ) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_by_id(&self, id: String) -> impl Future<Output = Result<Server, ServerError>> + Send;
}

pub trait ServerRepository: Clone + Send + Sync + 'static {
    fn create_server(
        &self,
        payload: CreateServer,
        name: String,
    ) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_by_id(&self, id: String) -> impl Future<Output = Result<Server, ServerError>> + Send;
}
