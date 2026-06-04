use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Consumable {
    pub id: u32,
    pub base: ConsumableBase,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsumableBase {
    Apple,
    Salve,
}

impl ConsumableBase {
    pub fn try_from_name(name: &str) -> Result<Self, String> {
        let lower = name.to_lowercase();
        match lower.as_str() {
            "apple" => Ok(Self::Apple),
            "salve" => Ok(Self::Salve),
            _ => Err(format!("unsupported consumable type: {name}")),
        }
    }
}

pub trait ConsumableRepository {
    fn load_all_possible_consumables(&self) -> Result<Vec<Consumable>, Box<dyn Error>>;
}

pub mod toml_repo;
pub use toml_repo::TomlConsumableRepository;