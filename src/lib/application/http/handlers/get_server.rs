use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::server::ports::ServerService;
use axum::http::StatusCode;
use axum::Extension;
use std::sync::Arc;

pub async fn get_server<S: ServerService>(
    Extension(_server_service): Extension<Arc<S>>,
) -> Result<ApiSuccess<String>, ApiError> {
    Ok(ApiSuccess::new(
        StatusCode::OK,
        "Server is running".to_string(),
    ))
}
