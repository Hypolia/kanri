use crate::application::ports::messaging_ports::MessagingPort;
use crate::domain::server::models::server::{Server, ServerCreatedMessage, ServerError};
use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::ports::{ServerRepository, ServerService};
use rand::{thread_rng, Rng};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ServerServiceImpl<S, M>
where
    S: ServerRepository,
    M: MessagingPort,
{
    server_repository: S,
    messaging: Arc<M>,
}

impl<S, M> ServerServiceImpl<S, M>
where
    S: ServerRepository,
    M: MessagingPort,
{
    pub fn new(server_repository: S, messaging: Arc<M>) -> Self {
        ServerServiceImpl {
            server_repository,
            messaging,
        }
    }

    fn generate_name(&self, server_type: &ServerType) -> String {
        let mut rng = thread_rng();

        match server_type {
            ServerType::Lobby => {
                let digits: String = format!("{:02}", rng.gen_range(0..100));
                let letter: char = rng.gen_range(b'A'..=b'Z') as char;
                format!("L{}{}", digits, letter)
            }
            _ => {
                let digits: String = format!("{:03}", rng.gen_range(0..1000)); // Trois chiffres
                let letters: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
                format!("m{}{}", digits, letters)
            }
        }
    }
}

impl<S, M> ServerService for ServerServiceImpl<S, M>
where
    S: ServerRepository,
    M: MessagingPort,
{
    async fn create_server(&self, server: CreateServer) -> Result<Server, ServerError> {
        let name = self.generate_name(&server.server_type);
        let server = self.server_repository.create_server(server, name).await?;

        info!("Creating server: {:?}", server);

        let message = ServerCreatedMessage {
            server_id: server.id.to_string(),
            name: server.name.clone(),
            server_type: server.server_type.to_string(),
        };

        let message_str = serde_json::to_string(&message).unwrap();

        self.messaging
            .publish_message(String::from("scheduler.request.create"), message_str)
            .await
            .map_err(|e| ServerError::CreateError(e.to_string()))?;

        Ok(server)
    }

    async fn find_by_id(&self, id: String) -> Result<Server, ServerError> {
        self.server_repository.find_by_id(id).await
    }
}
