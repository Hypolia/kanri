use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::server::models::server::Server;
use crate::domain::server::ports::ServerService;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GetServerResponseData(Server);

impl From<&Server> for GetServerResponseData {
    fn from(value: &Server) -> Self {
        GetServerResponseData(value.clone())
    }
}

pub async fn get_server<S: ServerService>(
    Extension(server_service): Extension<Arc<S>>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<GetServerResponseData>, ApiError> {
    server_service
        .find_by_id(id)
        .await
        .map_err(ApiError::from)
        .map(|ref server| ApiSuccess::new(StatusCode::OK, server.into()))
}
