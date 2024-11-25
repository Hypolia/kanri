use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::ports::{ServerRepository, ServerService};

#[derive(Debug, Clone)]
pub struct ServerServiceImpl<S>
where
    S: ServerRepository,
{
    server_repository: S,
}

impl<S> ServerServiceImpl<S>
where
    S: ServerRepository,
{
    pub fn new(server_repository: S) -> Self {
        ServerServiceImpl { server_repository }
    }
}

impl<S> ServerService for ServerServiceImpl<S>
where
    S: ServerRepository,
{
    async fn create_server(&self, server: CreateServer) -> Result<Server, ServerError> {
        self.server_repository.create_server(server).await
    }

    async fn find_by_id(&self, id: String) -> Result<Server, ServerError> {
        self.server_repository.find_by_id(id).await
    }
}
