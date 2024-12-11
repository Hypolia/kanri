mod handlers;
mod responses;

use crate::application::http::handlers::create_server::create_server;
use crate::application::http::handlers::get_server::get_server;
use crate::application::http::handlers::get_servers::get_servers;
use crate::application::http::handlers::health::health_check;
use crate::domain::server::ports::ServerService;
use anyhow::Context;
use axum::routing::{get, post};
use axum::Extension;
use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::application::http::handlers::get_servers::__path_get_servers;
use crate::application::http::handlers::get_servers::GetServersResponseData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone)]
struct AppState<S>
where
    S: ServerService,
{
    server_service: Arc<S>,
}

#[derive(OpenApi)]
#[openapi(
    paths(get_servers),
    components(
        schemas(GetServersResponseData)
    ),
)]
struct ApiDoc;

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<S>(config: HttpServerConfig, server_service: Arc<S>) -> anyhow::Result<Self>
    where
        S: ServerService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState { server_service };

        let router = axum::Router::new()
            .nest("/v1", api_routes())
            .merge(SwaggerUi::new("/swagger")
                .url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(trace_layer)
            .layer(Extension(Arc::clone(&state.server_service)))
            .with_state(state);

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

fn api_routes<S>() -> axum::Router<AppState<S>>
where
    S: ServerService + Send + Sync + 'static,
{
    axum::Router::new()
        .route("/servers/health", get(health_check))
        .route("/servers", post(create_server::<S>))
        .route("/servers", get(get_servers::<S>))
        .route("/servers/:id", get(get_server::<S>))

}
