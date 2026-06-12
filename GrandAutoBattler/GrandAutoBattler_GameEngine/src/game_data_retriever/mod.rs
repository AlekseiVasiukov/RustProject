use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;

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
pub(crate) struct Faction {
    name: String,
    unit_types: Vec<UnitType>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UnitType {
    name: String,
    pretty_name: Option<String>,
    females_only: bool,
    parameters_set: BTreeMap<String, u8>,
    first_names: Option<Vec<String>>,
    last_names: Option<Vec<String>>,
}

pub(crate) fn load_config() -> Result<String, Box <dyn Error>> {
    // let a = ConfigToml::new();
    // let d = toml_retriever::get_config()?;
    // println!("{:#?}", d);
    // let asd = get_config();
    // Err("Error".to_string())
    Ok("".to_string())
}

// pub(crate) fn load_available_factions() -> Result<Vec<Faction>, String> {
//     // Err("Error".to_string())
//     Ok(vec![0])
// }





#[derive(Debug, Deserialize)]
pub struct Config {
    pub BASIC_HEALTH: f32,
    pub BASIC_ATTACK: f32,
    pub ATTACK_PERCENT_PER_POINT: f32,
    pub DEFENSE_PERCENT_PER_POINT: f32,
    pub HEALTH_PERCENT_PER_POINT: f32,
    pub ATTACK_MIDDLE_VALUE_PARAMETER: f32,
    pub DEFENSE_MIDDLE_VALUE_PARAMETER: f32,
    pub HEALTH_MIDDLE_VALUE_PARAMETER: f32,
}


