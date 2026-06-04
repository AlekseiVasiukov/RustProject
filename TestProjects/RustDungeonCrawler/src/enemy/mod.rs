use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: u32,
    pub base: EnemyBase,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnemyBase {
    Rat,
    Bat,
}

impl EnemyBase {
    pub fn try_from_name(name: &str) -> Result<Self, String> {
        let lower = name.to_lowercase();
        match lower.as_str() {
            "rat" => Ok(Self::Rat),
            "bat" => Ok(Self::Bat),
            _ => Err(format!("unsupported enemy type: {name}")),
        }
    }
}

pub trait EnemyRepository {
    fn load_all_possible_enemies(&self) -> Result<Vec<Enemy>, Box<dyn Error>>;
}

pub mod toml_repo;
pub use toml_repo::TomlEnemyRepository;