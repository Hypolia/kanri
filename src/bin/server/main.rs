use clap::Parser;
use kanri::application::http::{HttpServer, HttpServerConfig};
use kanri::env::Env;
use kanri::infrastructure::db::postgres::Postgres;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let _postgres = Postgres::new(Arc::clone(&env)).await?;

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await
}
