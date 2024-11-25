use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::ports::{ServerRepository, ServerService};
use rand::{Rng, thread_rng};

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
                let letters: String = (0..2)
                    .map(|_| rng.gen_range(b'A'..=b'Z') as char)
                    .collect();
                format!("m{}{}", digits, letters)
            }
        }
    }
}

impl<S> ServerService for ServerServiceImpl<S>
where
    S: ServerRepository,
{
    async fn create_server(&self, server: CreateServer) -> Result<Server, ServerError> {
        let name = self.generate_name(&server.server_type);
        self.server_repository.create_server(server, name).await
    }

    async fn find_by_id(&self, id: String) -> Result<Server, ServerError> {
        self.server_repository.find_by_id(id).await
    }


}
