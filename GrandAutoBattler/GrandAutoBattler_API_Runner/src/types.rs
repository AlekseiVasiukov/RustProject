use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, openapi::schema};

//Health response, useful for future conteinerization
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthStatus {
    //returns OK while server is running
    pub status: String,
    pub message: String,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewGameInstanceIdResponse {
    pub game_id: String,
    pub success: bool,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetAvailableFactionsRequest {
    pub game_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetAvailableFactionsResponse {
    pub factions: Vec<String>,
    pub success: bool,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}
