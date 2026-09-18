use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//Health response, useful for future conteinerization
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthStatus {
    //returns OK while server is running
    pub status: String,
    pub message: String,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
}
