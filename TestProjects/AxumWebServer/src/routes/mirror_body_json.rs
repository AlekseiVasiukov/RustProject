// use crate::routes::Faction;
// use axum::{Json,Ser};
use serde::{Deserialize, Serialize};
// use serde_json::{self, json};
use axum::Json;

pub async fn mirror_body_json(Json(body): Json<Faction>) {
    // body
    dbg!(body);
    todo!();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitType {
    names: Vec<String>,
}

impl UnitType {
    pub fn new() -> Self {
        Self {
            names: vec!["Knight".to_string(), "Archer".to_string()],
        }
    }
}
