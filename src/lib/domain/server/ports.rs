use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::models::status::ServerStatus;
use std::future::Future;

pub trait ServerService: Clone + Send + Sync + 'static {
    fn create_server(
        &self,
        server: CreateServer,
    ) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_by_id(&self, id: String) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_all(
        &self,
        status: Option<ServerStatus>,
        server_type: Option<ServerType>,
    ) -> impl Future<Output = Result<Vec<Server>, ServerError>> + Send;
}

pub trait ServerRepository: Clone + Send + Sync + 'static {
    fn create_server(
        &self,
        payload: CreateServer,
        name: String,
    ) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_by_id(&self, id: String) -> impl Future<Output = Result<Server, ServerError>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<Server>, ServerError>> + Send;
    fn find_by_status(
        &self,
        status: ServerStatus,
    ) -> impl Future<Output = Result<Vec<Server>, ServerError>> + Send;
    fn find_by_type(
        &self,
        server_type: ServerType,
    ) -> impl Future<Output = Result<Vec<Server>, ServerError>> + Send;
    fn find_by_status_and_type(
        &self,
        status: ServerStatus,
        server_type: ServerType,
    ) -> impl Future<Output = Result<Vec<Server>, ServerError>> + Send;
}
