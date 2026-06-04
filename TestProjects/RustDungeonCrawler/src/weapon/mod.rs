use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Weapon {
    pub id: u32,
    pub base: WeaponBase,
    pub condition: Option<f32>,
    pub range: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeaponBase {
    Sword,
    Bow,
}

impl WeaponBase {
    pub fn try_from_name(name: &str) -> Result<Self, String> {
        let lower = name.to_lowercase();
        match lower.as_str() {
            "sword" => Ok(Self::Sword),
            "bow" => Ok(Self::Bow),
            _ => Err(format!("unsupported weapon type: {name}")),
        }
    }
}

pub trait WeaponRepository {
    fn load_all_possible_weapons(&self) -> Result<Vec<Weapon>, Box<dyn Error>>;
}

pub mod toml_repo;
pub use toml_repo::TomlWeaponRepository;