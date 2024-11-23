use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::server::models::server::{Server, ServerError};
use crate::domain::server::models::server_validator::CreateServer;
use crate::domain::server::ports::ServerService;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateServerResponseData {
    id: String,
}

impl From<&Server> for CreateServerResponseData {
    fn from(value: &Server) -> Self {
        CreateServerResponseData {
            id: value.id.to_string(),
        }
    }
}

impl From<ServerError> for ApiError {
    fn from(e: ServerError) -> Self {
        match e {
            ServerError::CreateError(e) => Self::InternalServerError(e.to_string()),
        }
    }
}

pub async fn create_server<S: ServerService>(
    Extension(server_service): Extension<Arc<S>>,
    Json(body): Json<CreateServer>,
) -> Result<ApiSuccess<CreateServerResponseData>, ApiError> {
    server_service
        .create_server(body)
        .await
        .map_err(ApiError::from)
        .map(|ref server| ApiSuccess::new(StatusCode::CREATED, server.into()))
}
