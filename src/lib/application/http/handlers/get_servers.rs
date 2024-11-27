use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::server::models::server::Server;
use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::status::ServerStatus;
use crate::domain::server::ports::ServerService;
use crate::utils::parse_enum_optional;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GetServersResponseData(Vec<Server>);

#[derive(Deserialize, Debug)]
pub struct Params {
    #[serde(deserialize_with = "parse_enum_optional")]
    status: Option<ServerStatus>,
    #[serde(deserialize_with = "parse_enum_optional")]
    server_type: Option<ServerType>,
}

impl From<Vec<Server>> for GetServersResponseData {
    fn from(value: Vec<Server>) -> Self {
        GetServersResponseData(value)
    }
}

pub async fn get_servers<S: ServerService>(
    Extension(server_service): Extension<Arc<S>>,
    Query(params): Query<Params>,
) -> Result<ApiSuccess<GetServersResponseData>, ApiError> {
    server_service
        .find_all(params.status, params.server_type)
        .await
        .map_err(ApiError::from)
        .map(|servers| ApiSuccess::new(StatusCode::OK, servers.into()))
}
