mod mirror_body_json;
mod mirror_body_string;
use axum::{
    Router,
    routing::{get, post},
};
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/", post(|| async { format!("{:#?}", Faction::new()) }))
        .route("/mirror", post(mirror_body_string))
        .route("/json", post(mirror_body_json(body)))
}

async fn hello_world() -> String {
    "Hello World".to_owned()
}

#[derive(Debug, Clone)]
struct Faction {
    unittypes: Vec<UnitType>,
}

impl Faction {
    pub fn new() -> Self {
        let a = UnitType::new();
        let b = UnitType::new();
        Self {
            unittypes: vec![a, b],
        }
    }
}

#[derive(Debug, Clone)]
struct UnitType {
    names: Vec<String>,
}

impl UnitType {
    pub fn new() -> Self {
        Self {
            names: vec!["Knight".to_string(), "Archer".to_string()],
        }
    }
}
