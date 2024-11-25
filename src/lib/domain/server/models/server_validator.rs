use crate::domain::server::models::server_type::ServerType;
use crate::domain::server::models::status::ServerStatus;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Validate)]
pub struct CreateServer {
    /*    #[validate(length(min = 3, max = 50))]
    pub name: String,*/
    #[validate(range(min = 0, max = 100))]
    pub player_count: i32,
    #[validate(range(min = 0, max = 100))]
    pub max_player_count: i32,
    pub server_type: ServerType,
    pub status: ServerStatus,
    pub address: String,
}
