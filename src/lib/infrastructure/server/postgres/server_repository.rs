use crate::domain::models::ServerStatus;
use crate::domain::ports::ServerRepository;
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
    async fn create_server(&self, address: String, status: ServerStatus) -> Result<(), anyhow::Error> {
        let uuid: uuid::Uuid = uuid::Uuid::new_v4();

        sqlx::query_as!(
            Server,
            r#"INSERT INTO servers (id, address, status) VALUES ($1, $2, $3)"#,
            uuid,
            address,
            status.to_string()
        )
        .execute(&*self.postgres.get_pool())
        .await?;

        Ok(())
    }
}
