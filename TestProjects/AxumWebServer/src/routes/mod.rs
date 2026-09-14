use axum::{
    Router,
    routing::{get, post},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/", post(|| async { format!("{:#?}", Faction::new()) }))
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
