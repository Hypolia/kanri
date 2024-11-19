use anyhow::Context;
use tokio::net;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
struct AppState {}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new(config: HttpServerConfig) -> anyhow::Result<Self> {
        let router = axum::Router::new();

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}
