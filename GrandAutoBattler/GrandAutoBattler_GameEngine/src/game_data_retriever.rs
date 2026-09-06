use rand::Rng;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::toml_parser;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Faction {
    pub(crate) name: String,
    pub unit_types: Vec<UnitType>,
    pub female_first_names: Vec<String>,
    pub male_first_names: Vec<String>,
    pub female_last_names: Vec<String>,
    pub male_last_names: Vec<String>,
}

impl Faction {
    pub fn new(faction_name: String) -> Self {
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
pub(crate) struct UnitType {
    pub unit_type_name: String,
    pub unit_type_pretty_name: Option<String>,
    pub females_only: bool,
    pub parameters_set: BTreeMap<String, u8>,
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
            item.females_only.unwrap_or(false),  //amazing fallback, need to remember that
            parameters_set,
        )
    }
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config = toml_parser::load_config()?;
    Ok(config)
}

pub fn get_factions() -> Result<Vec<Faction>, Box<dyn Error>> {
    let factions = toml_parser::load_factions()?;
    Ok(factions)
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Clone, Debug)]
pub(crate) struct IdRegistry {
    used_ids: BTreeSet<String>,
}

impl IdRegistry {
    pub(crate) fn new() -> Self {
        Self {
            used_ids: BTreeSet::new(),
        }
    }

    pub(crate) fn generate_id(&mut self) -> String {
        loop {
            let id = rand::random::<u32>();
            let id_str = format!("{:08x}", id);

            //If not exist - save for future and return
            if self.used_ids.insert(id_str.clone()) {
                return id_str;
            }
        }
    }
}
