use clap::Parser;
use kanri::application::http::{HttpServer, HttpServerConfig};
use kanri::application::messaging::start_subscriptions;
use kanri::application::ports::messaging_ports::MessagingTypeImpl;
use kanri::domain::server::service::ServerServiceImpl;
use kanri::env::Env;
use kanri::infrastructure::db::postgres::Postgres;
use kanri::infrastructure::server::postgres::server_repository::PostgresServerRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    topic: String,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let postgres = Arc::new(Postgres::new(Arc::clone(&env)).await?);

    let messaging_port =
        Arc::new(MessagingTypeImpl::new(&env.messaging_type, Arc::clone(&env)).await?);

    let server_repository = PostgresServerRepository::new(Arc::clone(&postgres));
    let server_service = Arc::new(ServerServiceImpl::new(
        server_repository,
        Arc::clone(&messaging_port),
    ));

    start_subscriptions(Arc::clone(&messaging_port), Arc::clone(&server_service)).await?;

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config, Arc::clone(&server_service)).await?;

    http_server.run().await
}
