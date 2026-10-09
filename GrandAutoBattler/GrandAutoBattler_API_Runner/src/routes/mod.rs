use GrandAutoBattler_GameEngine::game_instance_manager::GameInstancesManager;
use axum::{
    Extension, Json, Router, extract::{Path, State}, http::{StatusCode, request, response}, routing::{get, post},
};
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;
use utoipa::{OpenApi};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use utoipa_swagger_ui::SwaggerUi;

use GrandAutoBattler_GameEngine::game_result_manager::GameResult;
use GrandAutoBattler_GameEngine::battle_journal_manager::{BattleJournal, GameEvent, Command, Effect};
use GrandAutoBattler_GameEngine::combatant_manager::Combatant;
use crate::types::{
    GetAvailableFactionsResponse, GetAvailableUnitTypesResponse,
    GetGameResultByIdAndSelectionRequest, GetGameResultByIdAndSelectionResponse, HealthStatus,
    NewGameInstanceIdResponse,
};

#[derive(Clone)]
pub struct ApiState {
    pub game_manager: Arc<Mutex<GameInstancesManager>>,
}


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Grand Auto-Battler gameengine API",
        description = "API for retrieving game_instance objects, the options to choose during gameplay and results of simulated battles.",
        version = "0.0.2",
        contact(name = "https://github.com/AlekseiVasiukov")
    ),
    paths(
        health_check,
        new_game_id_retrieve,
        available_factions_retrieve,
        available_unittypes_retrieve,
        game_result_retrieve,
    ),
    components(schemas(
        HealthStatus,
        NewGameInstanceIdResponse,
        GetAvailableFactionsResponse,
        GetAvailableUnitTypesResponse,
        GetGameResultByIdAndSelectionRequest,
        GetGameResultByIdAndSelectionResponse,
        GameResult,
        BattleJournal,
        Combatant,
        GameEvent,
        Command,
        Effect,
    ),)
)]
pub struct ApiDoc;

pub fn create_routes() -> anyhow::Result<Router> {
    let game_manager = GameInstancesManager::new();
    let api_state = ApiState {
        game_manager: Arc::new(Mutex::new(game_manager)),

    };


    Ok(Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/new_game_id", post(new_game_id_retrieve))
        .route("/games/:game_id/factions", get(available_factions_retrieve))
        .route("/games/:game_id/unittypes", get(available_factions_retrieve))
        .route("/games/:game_id/result", post(game_result_retrieve))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(api_state)
)
  
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Server is health", body = HealthStatus),
    )
)]
async fn health_check() -> (StatusCode, Json<HealthStatus>) {
    let status = HealthStatus {
        status: "ok".to_string(),
        message: "Game Engine server is running. Use /swagger-ui for API documentation.".to_string(),
        timestamp: chrono::Utc::now(),
    };
    (StatusCode::OK, Json(status))
}

#[utoipa::path(
    post,
    path = "/new_game_id",
    tag = "Game Id",
    responses(
        (status = 200, description = "ID for newly created game instance", body = NewGameInstanceIdResponse),
        (status = 500, description = "Failed to create game instance", body = NewGameInstanceIdResponse),
    )
)]
async fn new_game_id_retrieve(
    State(state): State<ApiState>,
) -> (StatusCode, Json<NewGameInstanceIdResponse>) {
    let mut game_manager = state.game_manager.lock().await;

    match game_manager.create_new_game_instance(){
        Ok(game_id)=>{
            drop(game_manager);
            let response = NewGameInstanceIdResponse{
                game_id,
                timestamp: chrono::Utc::now(),
                success: true,
                error: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) =>{
            drop(game_manager);
            tracing::error!("Failed to create game instance: {}", e);
            let response = NewGameInstanceIdResponse{
                game_id: String::new(),
                timestamp: chrono::Utc::now(),
                success: false,
                error: Some(e.to_string()),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }

    }
   
}

#[utoipa::path(
    get,
    path = "/games/{game_id}/factions",
    tag = "Game Data",
    params(("game_id"= String, Path, description = "Game Instance Id")),
    responses(
        (status = 200, description = "Get the available factions based on particular game_id of a game instance", body = GetAvailableFactionsResponse),
        (status = 404, description = "Game instance not found", body = GetAvailableFactionsResponse),
    )
)]
async fn available_factions_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
) -> (StatusCode, Json<GetAvailableFactionsResponse>) {
    let game_manager = state.game_manager.lock().await;
    match game_manager.get_available_factions(&game_id){
        Ok(factions) =>{
            drop(game_manager);
            let response = GetAvailableFactionsResponse{
                factions,
                timestamp: chrono::Utc::now(),
                success: true,
                error: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e)=>{
            drop(game_manager);
            tracing::error!("Failed to get factions for game '{}':{}", game_id, e);
            let response = GetAvailableFactionsResponse{
                factions: Vec::new(),
                                timestamp: chrono::Utc::now(),
success:false,
error: Some(e.to_string()),
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
  
}

#[utoipa::path(
    get,
    path = "/games/{game_id}/unittypes",
    tag = "Game Data",
    params(("game_id"= String, Path, description = "Game Instance Id")),
    responses(
        (status = 200, description = "Get the available unit types based on particular game_id of a game instance", body = GetAvailableUnitTypesResponse),
                (status = 404, description = "Game instance not found", body = GetAvailableUnitTypesResponse),

    )
)]
async fn available_unittypes_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
) -> (StatusCode, Json<GetAvailableUnitTypesResponse>) {
    let game_manager = state.game_manager.lock().await;
    match game_manager.get_available_unit_types(&game_id){
        Ok(unittypes) => {
            drop(game_manager);
            let response = GetAvailableUnitTypesResponse{
                unittypes: unittypes,
                                timestamp: chrono::Utc::now(),
success: true,
error: None,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            drop(game_manager);
            tracing::error!("Not successful unit types retrieval for game '{}': {}", game_id, e
        );
        let response = GetAvailableUnitTypesResponse{
            unittypes: BTreeMap::new(),
                timestamp: chrono::Utc::now(),
                success:false,
                error: Some(e.to_string()),
        };
        (StatusCode::NOT_FOUND, Json(response))
        }
    }
   
}




#[utoipa::path(
    post,
    path = "/games/{game_id}/result",
    tag = "Game Data",
    params(
        ("game_id" = String, Path, description = "Game Instance Id")
    ),
    request_body = GetGameResultByIdAndSelectionRequest,
    responses(
        (
            status = 200, 
            description = "Calculated game result",
            body = GetGameResultByIdAndSelectionResponse

        ),
               (
            status = 500, 
            description = "Failed to calculate game result",
            body = GetGameResultByIdAndSelectionResponse

        ),
    )
    
)]
async fn game_result_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
    Json(request): Json<GetGameResultByIdAndSelectionRequest>,
)->(StatusCode, Json<GetGameResultByIdAndSelectionResponse>){
    let mut selected_factions = BTreeMap::new();
    selected_factions.insert(0, request.our_faction_index);
    selected_factions.insert(1, request.opponent_faction_index);

    let mut selected_unittypes: BTreeMap<usize, Option<usize>> = BTreeMap::new();
    for (index, value) in request.our_unittype_indices.iter().enumerate()
    {
        selected_unittypes.insert(index, *value);
    }

    let mut game_manager = state.game_manager.lock().await;

    match game_manager.get_game_result_by_id_and_selection(&game_id, &selected_factions, &selected_unittypes){
        Ok(game_result)=>{
            drop(game_manager);
            let response = GetGameResultByIdAndSelectionResponse{
                result: game_result,
                success: true,
                                timestamp: chrono::Utc::now(),
error:None
            };
            (StatusCode::OK, Json(response))
        }
        Err(e)=>{
            drop(game_manager);
            tracing::error!("Failed to calculate result for '{}': {}",game_id,e);
            let response = GetGameResultByIdAndSelectionResponse{
                result: GrandAutoBattler_GameEngine::game_result_manager::GameResult::new(BTreeMap::new(), GrandAutoBattler_GameEngine::battle_journal_manager::BattleJournal::new(),)
                .unwrap_or_else(|_|{
                    GrandAutoBattler_GameEngine::game_result_manager::GameResult{
                        generated_combatants: BTreeMap::new(),
                        battle_journal: GrandAutoBattler_GameEngine::battle_journal_manager::BattleJournal::new(),
                    }
                }),
                success: false,
                                timestamp: chrono::Utc::now(),
error: Some(e.to_string()),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
   
}

