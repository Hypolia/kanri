use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::models::status::ServerStatus;
use crate::domain::server::ports::ServerRepository;
use crate::infrastructure::db::postgres::Postgres;
use std::sync::Arc;
use tracing::info;

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
    async fn create_server(
        &self,
        payload: CreateServer,
        name: String,
    ) -> Result<Server, ServerError> {
        let uuid: uuid::Uuid = uuid::Uuid::new_v4();

        let server = sqlx::query_as!(
            Server,
            r#"INSERT INTO servers (id, name, player_count, max_player_count, server_type, status, address)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, player_count, max_player_count, server_type, status, address"#,
            uuid,
            name,
            payload.player_count,
            payload.max_player_count,
            payload.server_type.to_string(),
            ServerStatus::Created.to_string(),
            payload.address
        )
        .fetch_one(&*self.postgres.get_pool())
        .await
            .map_err(|e| ServerError::CreateError(e.to_string()))?;

        Ok(server)
    }

    async fn find_by_id(&self, id: String) -> Result<Server, ServerError> {
        let uuid = uuid::Uuid::parse_str(&id).map_err(|_e| ServerError::NotFound)?;

        let server = sqlx::query_as!(
            Server,
            r#"SELECT id, name, player_count, max_player_count, server_type, status, address
            FROM servers
            WHERE id = $1"#,
            uuid
        )
        .fetch_one(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(server)
    }

    async fn find_all(&self) -> Result<Vec<Server>, ServerError> {
        info!("Finding all servers");
        let servers = sqlx::query_as!(
            Server,
            r#"SELECT id, name, player_count, max_player_count, server_type, status, address
            FROM servers"#,
        )
        .fetch_all(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(servers)
    }

    async fn find_by_status(&self, status: ServerStatus) -> Result<Vec<Server>, ServerError> {
        info!("Finding servers by status: {:?}", status);
        let servers = sqlx::query_as!(
            Server,
            r#"SELECT id, name, player_count, max_player_count, server_type, status, address
            FROM servers
            WHERE status = $1"#,
            status.to_string()
        )
        .fetch_all(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(servers)
    }

    async fn find_by_type(&self, server_type: ServerType) -> Result<Vec<Server>, ServerError> {
        info!("Finding servers by type: {:?}", server_type);
        let servers = sqlx::query_as!(
            Server,
            r#"SELECT id, name, player_count, max_player_count, server_type, status, address
            FROM servers
            WHERE server_type = $1"#,
            server_type.to_string()
        )
        .fetch_all(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(servers)
    }

    async fn find_by_status_and_type(
        &self,
        status: ServerStatus,
        server_type: ServerType,
    ) -> Result<Vec<Server>, ServerError> {
        info!(
            "Finding servers by status: {:?} and type: {:?}",
            status, server_type
        );
        let servers = sqlx::query_as!(
            Server,
            r#"SELECT id, name, player_count, max_player_count, server_type, status, address
            FROM servers
            WHERE status = $1 AND server_type = $2"#,
            status.to_string(),
            server_type.to_string()
        )
        .fetch_all(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(servers)
    }

    async fn update_status_by_id(
        &self,
        id: String,
        status: ServerStatus,
    ) -> Result<(), ServerError> {
        let uuid = uuid::Uuid::parse_str(&id).map_err(|_e| ServerError::NotFound)?;

        sqlx::query!(
            r#"UPDATE servers
            SET status = $1
            WHERE id = $2"#,
            status.to_string(),
            uuid
        )
        .execute(&*self.postgres.get_pool())
        .await
        .map_err(|_| ServerError::NotFound)?;

        Ok(())
    }
}
