// mod types;

use GrandAutoBattler_GameEngine::game_instance_manager::GameInstancesManager;
// use types::HealthStatus;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::{StatusCode, request},
    routing::{get, post},
};
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;
// use axum::{};
use utoipa::{OpenApi, ToSchema};
// use utoipa_swagger_ui::SwaggerUi;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use utoipa_swagger_ui::SwaggerUi;

use crate::types::{
    GetAvailableFactionsResponse, GetAvailableUnitTypesResponse,
    GetGameResultByIdAndSelectionRequest, GetGameResultByIdAndSelectionResponse, HealthStatus,
    NewGameInstanceIdResponse,
};

#[derive(Clone)]
pub struct ApiState {
    pub game_manager: Arc<Mutex<GameInstancesManager>>,
}

// #[derive(OpenApi)]
// #[openapi(
//     info(
//         title = "Grand Auto Battler gameengine API",
//         description = "API for retrieving game_instance objects, the options to choose during gameplay and results of simulated battles",
//         version = "0.0.1",
//         contact(name = "https://github.com/AlekseiVasiukov")
//     ),
//     paths(health_check),
//     components(schemas(HealthStatus,))
// )]
// pub struct ApiDoc;

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

        .with_state(api_state)
)
    // let mut game_manager = GameInstancesManager::new();
    // // let game = game_manager.GetNewGameInstanceId();
    // // println!("{:?}", game);
    // // println!("YO");
    // let api_state = ApiState {
    //     game_manager: Arc::new(Mutex::new(game_manager)),
    // };

    // // let state = ApiState {api_state};

    // // let routes = create_routes();
    // return Router::new()
    //     .route("/", get(health_check))
    //     .route("/health", get(health_check))
    //     .route("/new_game_id", post(new_game_id_retrieve))
    //     .route("/games/:game_id/factions", get(available_factions_retrieve))
    //     .route(
    //         "/games/:game_id/unittypes",
    //         get(available_unittypes_retrieve),
    //     )
    //             .route("/games/:game_id/result", post(game_result_retrieve))

    //     //Add swagger for interactive api]
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    //     .layer(
    //         tower_http::cors::CorsLayer::permissive(), // .with_state(api_state)
    //     )
    //     .with_state(api_state);
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
        message: "Game Engine server is running. For API documentation use /swagger-ui path".to_string(),
        timestamp: chrono::Utc::now(),
    };
    (StatusCode::OK, Json(status))
}

#[utoipa::path(
    post,
    path = "/new_game_id",
    tag = "Game Id",
    responses(
        (status = 200, description = "Get the id for newly created game instance", body = NewGameInstanceIdResponse),
    )
)]
async fn new_game_id_retrieve(
    State(state): State<ApiState>,
) -> (StatusCode, Json<NewGameInstanceIdResponse>) {
    let mut game_manager = state.game_manager.lock().await;
    let generation_result = game_manager.GetNewGameInstanceId();
    drop(game_manager);
    let response = NewGameInstanceIdResponse {
        game_id: generation_result,
        timestamp: chrono::Utc::now(),
        success: true,
        error: None,
    };
    (StatusCode::OK, Json(response))
}

#[utoipa::path(
    get,
    path = "/games/{game_id}/factions",
    tag = "Game Data",
    params(("game_id"= String, Path, description = "Game Instance Id")),
    responses(
        (status = 200, description = "Get the available factions based on particular game_id of a game instance", body = GetAvailableFactionsResponse),
    )
)]
async fn available_factions_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
) -> (StatusCode, Json<GetAvailableFactionsResponse>) {
    let game_manager = state.game_manager.lock().await;
    let factions = game_manager.get_available_factions(&game_id);
    drop(game_manager);
    // let generation_result = state.game_manager.GetNewGameInstanceId();

    let response = GetAvailableFactionsResponse {
        factions,
        timestamp: chrono::Utc::now(),
        success: true,
        error: None,
    };
    (StatusCode::OK, Json(response))
}

#[utoipa::path(
    get,
    path = "/games/{game_id}/unittypes",
    tag = "Game Data",
    params(("game_id"= String, Path, description = "Game Instance Id")),
    responses(
        (status = 200, description = "Get the available unit types based on particular game_id of a game instance", body = GetAvailableUnitTypesResponse),
    )
)]
async fn available_unittypes_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
) -> (StatusCode, Json<GetAvailableUnitTypesResponse>) {
    let game_manager = state.game_manager.lock().await;
    let unittypes = game_manager.get_available_unit_types(&game_id);
    drop(game_manager);
    // let generation_result = state.game_manager.GetNewGameInstanceId();

    let response = GetAvailableUnitTypesResponse {
        unittypes,
        timestamp: chrono::Utc::now(),
        success: true,
        error: None,
    };
    (StatusCode::OK, Json(response))
}


//Example request
// {
//   "our_faction_index": 0,
//   "opponent_faction_index": 1,
//   "our_unittype_indices": [0, 2, null, 1]
// }


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
    )
    
)]
async fn game_result_retrieve(
    Path(game_id): Path<String>,
    State(state): State<ApiState>,
    Json(request): Json<GetGameResultByIdAndSelectionRequest>,
)->(StatusCode, Json<GetGameResultByIdAndSelectionResponse>){
    let mut selected_factions = std::collections::BTreeMap::new();
    selected_factions.insert(0, request.our_faction_index);
    selected_factions.insert(1, request.opponent_faction_index);

    let mut selected_unittypes: BTreeMap<usize, Option<usize>> = BTreeMap::new();
    for (index, value) in request.our_unittype_indices.iter().enumerate() {
        selected_unittypes.insert(index, *value);
    }

    let mut game_manager = state.game_manager.lock().await;
    let game_result = game_manager.GetGameResultByIdAndSelection(&game_id, &selected_factions, &selected_unittypes);
    drop(game_manager);
    let response = GetGameResultByIdAndSelectionResponse{
        result: game_result,
        success: true,
        timestamp: chrono::Utc::now(),
        error: None,

    };
 (StatusCode::OK, Json(response) )
}

// async fn available_factions_retrieve(
//     State(mut state): State<ApiState>, Json(request): Json<GetAvailableFactionsRequest)
// ) -> (StatusCode, Json<GetAvailableFactionsResponse>) {
//     // let available_factions = State.game_manager.
//     (StatusCode::OK, Json(response))
// }
