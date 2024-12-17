use crate::env::Env;
use anyhow::{Context, Ok, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub pool: Arc<PgPool>,
}

impl Postgres {
    pub async fn new(env: Arc<Env>) -> Result<Self> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            &env.database_user, &env.database_password, &env.database_host, &env.database_port, &env.database_name
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .context("Failed to create Postgres pool")?;

        info!("Connected to the Postgres database");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}
