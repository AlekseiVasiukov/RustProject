use std::collections::BTreeMap;

use GrandAutoBattler_GameEngine::game_result_manager::GameResult;
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
pub struct GetAvailableFactionsResponse {
    pub factions: Vec<String>,
    pub success: bool,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetAvailableUnitTypesResponse {
    pub unittypes: BTreeMap<usize, Vec<String>>,
    pub success: bool,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetGameResultByIdAndSelectionRequest {
    pub our_faction_index: usize,
    pub opponent_faction_index: usize,
    pub our_unittype_indices: Vec<Option<usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetGameResultByIdAndSelectionResponse {
    pub result: GameResult,
    pub success: bool,
    #[schema(value_type = String, format = DateTime)]
    pub timestamp: DateTime<Utc>,
    pub error: Option<String>,
}
