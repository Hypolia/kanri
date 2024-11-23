use clap::Parser;
use kanri::application::http::{HttpServer, HttpServerConfig};
use kanri::domain::server::service::ServerServiceImpl;
use kanri::env::Env;
use kanri::infrastructure::db::postgres::Postgres;
use kanri::infrastructure::server::postgres::server_repository::PostgresServerRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let postgres = Arc::new(Postgres::new(Arc::clone(&env)).await?);

    let server_repository = PostgresServerRepository::new(Arc::clone(&postgres));
    let server_service = Arc::new(ServerServiceImpl::new(server_repository));

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config, Arc::clone(&server_service)).await?;

    http_server.run().await
}
