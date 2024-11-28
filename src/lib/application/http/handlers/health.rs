use crate::application::http::handlers::{ApiError, ApiSuccess};
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HealthResponseData {
    message: String,
}

pub async fn health_check() -> Result<ApiSuccess<HealthResponseData>, ApiError> {
    Ok(ApiSuccess::new(
        StatusCode::OK,
        HealthResponseData {
            message: "Server is running".to_string(),
        },
    ))
}
