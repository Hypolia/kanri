use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::ports::ServerRepository;
use crate::infrastructure::db::postgres::Postgres;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PostgresServerRepository {
    postgres: Arc<Postgres>,
}

impl PostgresServerRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

impl ServerRepository for PostgresServerRepository {
    async fn create_server(&self, payload: CreateServer) -> Result<Server, ServerError> {
        let uuid: uuid::Uuid = uuid::Uuid::new_v4();

        let server = sqlx::query_as!(
            Server,
            r#"INSERT INTO servers (id, name, player_count, max_player_count, server_type, status, address)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, player_count, max_player_count, server_type, status, address"#,
            uuid,
            payload.name,
            payload.player_count,
            payload.max_player_count,
            payload.server_type.to_string(),
            payload.status.to_string(),
            payload.address
        )
        .fetch_one(&*self.postgres.get_pool())
        .await
            .map_err(|e| ServerError::CreateError(e.to_string()))?;

        Ok(server)
    }
}
