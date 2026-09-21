// mod types;

use GrandAutoBattler_GameEngine::game_instance_manager::GameInstancesManager;
// use types::HealthStatus;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::{StatusCode, request},
    routing::get,
};
// use axum::{};
use utoipa::{OpenApi, ToSchema};
// use utoipa_swagger_ui::SwaggerUi;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use utoipa_swagger_ui::SwaggerUi;

use crate::types::{GetAvailableFactionsResponse, HealthStatus, NewGameInstanceIdResponse};

#[derive(Clone)]
pub struct ApiState {
    pub game_manager: GameInstancesManager,
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
        title = "Grand Auto Battler gameengine API",
        description = "API for retrieving game_instance objects, the options to choose during gameplay and results of simulated battles.",
        version = "0.0.1",
        contact(name = "https://github.com/AlekseiVasiukov")
    ),
    paths(health_check, new_game_id_retrieve, available_factions_retrieve),
    components(schemas(HealthStatus, NewGameInstanceIdResponse, GetAvailableFactionsResponse,),)
)]
pub struct ApiDoc;

pub fn create_routes() -> Router {
    let mut game_manager = GameInstancesManager::new();
    // let game = game_manager.GetNewGameInstanceId();
    // println!("{:?}", game);
    // println!("YO");
    let api_state = ApiState { game_manager };

    // let state = ApiState {api_state};

    // let routes = create_routes();
    return Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/new_game_id", get(new_game_id_retrieve))
        .route("/games/:game_id/factions", get(available_factions_retrieve))
        //Add swagger for interactive api]
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            tower_http::cors::CorsLayer::permissive(), // .with_state(api_state)
        )
        .with_state(api_state);
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
        message: "Game Engine server is running".to_string(),
        timestamp: chrono::Utc::now(),
    };
    (StatusCode::OK, Json(status))
}

#[utoipa::path(
    get,
    path = "/new_game_id",
    tag = "Game Id",
    responses(
        (status = 200, description = "Get the id for newly created game instance", body = NewGameInstanceIdResponse),
    )
)]
async fn new_game_id_retrieve(
    State(mut state): State<ApiState>,
) -> (StatusCode, Json<NewGameInstanceIdResponse>) {
    let generation_result = state.game_manager.GetNewGameInstanceId();

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
    params(("game_id"= String, Path, description = "Game instance id")),
    responses(
        (status = 200, description = "Get the available factions based on particular game_id of a game instance", body = GetAvailableFactionsResponse),
    )
)]
async fn available_factions_retrieve(
    Path(game_id): Path<String>,
    State(mut state): State<ApiState>,
) -> (StatusCode, Json<GetAvailableFactionsResponse>) {
    let factions = state.game_manager.GetAvailableFactions(&game_id);

    // let generation_result = state.game_manager.GetNewGameInstanceId();

    let response = GetAvailableFactionsResponse {
        factions,
        timestamp: chrono::Utc::now(),
        success: true,
        error: None,
    };
    (StatusCode::OK, Json(response))
}

// async fn available_factions_retrieve(
//     State(mut state): State<ApiState>, Json(request): Json<GetAvailableFactionsRequest)
// ) -> (StatusCode, Json<GetAvailableFactionsResponse>) {
//     // let available_factions = State.game_manager.
//     (StatusCode::OK, Json(response))
// }
