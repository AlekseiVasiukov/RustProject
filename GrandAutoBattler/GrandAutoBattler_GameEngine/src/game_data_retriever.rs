// mod ::toml_parser;

// use super::GameInstance::load_config;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;

use crate::{game_data_retriever, toml_parser};

// mod toml_retriever;

// use toml_retriever::ConfigToml;
#[derive(Debug, Deserialize)]
pub struct Combatant {
    id: String,
    faction: Faction,
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    parameters: BTreeMap<String, u8>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Faction {
    pub name: String,
    pub unit_types: Vec<UnitType>,
    pub female_first_names: Vec<String>,
    pub male_first_names: Vec<String>,
    pub female_last_names: Vec<String>,
    pub male_last_names: Vec<String>,
}

impl Faction {
    pub fn new(faction_name: String) -> Self {
        // let unit_types = Vec::new();

        // let female_first_names = Vec::new();
        // let male_first_names = Vec::new();
        // let female_last_names = Vec::new();
        // let male_last_names = Vec::new();
        Self {
            name: faction_name,
            unit_types: Vec::new(),

            female_first_names: Vec::new(),
            male_first_names: Vec::new(),
            female_last_names: Vec::new(),
            male_last_names: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UnitType {
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    females_only: bool,
    parameters_set: BTreeMap<String, u8>,
}

impl UnitType {
    pub fn new(
        unit_type_name: String,
        unit_type_pretty_name: Option<String>,
        females_only: bool,
        parameters_set: BTreeMap<String, u8>,
    ) -> Self {
        Self {
            unit_type_name,
            unit_type_pretty_name,
            females_only,
            parameters_set,
        }
    }
}

//From pattern, good shit
impl From<toml_parser::UnitTypesToml> for UnitType {
    fn from(item: toml_parser::UnitTypesToml) -> Self {
        let mut parameters_set = BTreeMap::new();
        parameters_set.insert("health".to_string(), item.health);
        parameters_set.insert("attack".to_string(), item.attack);
        parameters_set.insert("range".to_string(), item.range);
        parameters_set.insert("defense".to_string(), item.defense);

        UnitType::new(
            item.unit_type_name,
            item.unit_type_pretty_name,
            item.females_only.unwrap_or(false),
            parameters_set,
        )
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config = toml_parser::load_config()?;
    // let asd = get_config();
    // Err("Error".to_string())
    Ok(config)
}

pub fn get_factions() -> Result<Vec<Faction>, Box<dyn Error>> {
    let d = toml_parser::load_factions()?;
    // let asd = get_config();
    // Err("Error".to_string())
    Ok(d)
}

// pub(crate) fn load_available_factions() -> Result<Vec<Faction>, String> {
//     // Err("Error".to_string())
//     Ok(vec![0])
// }

#[derive(Debug, Deserialize)]
pub struct Config {
    pub BASIC_HEALTH: f32,
    pub BASIC_ATTACK: f32,
    pub ATTACK_PERCENTAGE_PER_POINT: f32,
    pub DEFENSE_PERCENTAGE_PER_POINT: f32,
    pub HEALTH_PERCENTAGE_PER_POINT: f32,
    pub ATTACK_MIDDLE_VALUE_PARAMETER: f32,
    pub DEFENSE_MIDDLE_VALUE_PARAMETER: f32,
    pub HEALTH_MIDDLE_VALUE_PARAMETER: f32,
}

// pub(crate) fn get_config() -> _ {
//     todo!()
// }

// pub(crate) mod game_data_retriever;
