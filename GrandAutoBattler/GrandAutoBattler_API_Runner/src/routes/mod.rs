// mod types;

// use types::HealthStatus;
use axum::{Json, Router, http::StatusCode, routing::get};
use utoipa::{OpenApi, ToSchema};
// use utoipa_swagger_ui::SwaggerUi;
use chrono::{DateTime, Utc};
use utoipa_swagger_ui::SwaggerUi;

use crate::types::HealthStatus;

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
    paths(health_check),
    components(schemas(HealthStatus))
)]
pub struct ApiDoc;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        //Add swagger for interactive api]
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(tower_http::cors::CorsLayer::permissive())
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Server is health", body = HealthStatus),
    )
)]
pub async fn health_check() -> (StatusCode, Json<HealthStatus>) {
    let status = HealthStatus {
        status: "ok".to_string(),
        message: "Game Engine server is running".to_string(),
        timestamp: chrono::Utc::now(),
    };
    (StatusCode::OK, Json(status))
}
