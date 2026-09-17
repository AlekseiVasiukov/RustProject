mod mirror_body_json;
mod mirror_body_string;

use crate::routes::mirror_body_json::Faction;
use axum::Json;
use axum::{
    Router,
    routing::{get, post},
};
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use serde::{Deserialize, Serialize};
pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/", post(|| async { format!("{:#?}", Faction::new()) }))
        .route("/mirror", post(mirror_body_string))
        .route("/json", post(mirror_body_json))
}

async fn hello_world() -> String {
    "Hello World".to_owned()
}
