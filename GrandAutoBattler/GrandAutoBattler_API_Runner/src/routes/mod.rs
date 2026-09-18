// mod types;

// use types::HealthStatus;
use axum::{Json, Router, http::StatusCode, routing::get};

use crate::types::HealthStatus;

pub fn create_routes() -> Router {
    Router::new().route("/health", get(health_check()))
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Server is health", body = HealthStatus),
    )
)]
pub fn health_check() -> (StatusCode, Json<HealthStatus>) {
    let status = HealthStatus {
        status: "ok".to_string(),
        message: "Game Engine server is running".to_string(),
        timestamp: chrono::Utc::now(),
    };
    (StatusCode::OK, Json(status))
}
