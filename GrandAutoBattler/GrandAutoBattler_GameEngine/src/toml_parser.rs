use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::game_data_retriever::*;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    file_path: String,
}

impl ConfigToml {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // pub fn new() -> Self {
        let exe_path = env::current_exe()?;
        // .expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().ok_or("Failed to get parent dir");
        // .expect("Failed to get parent dir");
        let settings_dir = exe_dir?.join("assets").join("Settings");
        let config_file_path = settings_dir.join("GameSettings.toml");
        // let config_file_path_string = config_file_path.to_str()?;
        // .expect("Failed to convert path to string").to_string();

        let config_file_path_string = config_file_path
            .to_str()
            .ok_or("Failed to convert path to string (possibly invalid UTF-8)")?
            .to_string();

        Ok(Self {
            file_path: config_file_path_string,
        })
    }
}

// #[derive(Debug,Deserialize)]
// pub(crate) struct  UnitTypesToml {
//         #[serde(rename = "name")]

//     UnitTypes: Vec<UnitType>,
// }

#[derive(Debug, Deserialize, Clone)]

struct UnitTypesArrayToml {
    UnitTypes: Vec<UnitTypesToml>,
}

#[derive(Debug, Deserialize, Clone)]
struct UnitTypesToml {
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    // #[serde(Option)]
    females_only: Option<bool>,
    health: u8,
    attack: u8,
    range: u8,
    defense: u8,
}

#[derive(Debug,Deserialize)]
struct NameToml {
    name: Vec<String>
}

fn parse_config_from_str(toml_string: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(toml_string)
}

// pub fn get_config() -> Config {
pub(crate) fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_toml = ConfigToml::new()?;
    let config_path = PathBuf::from(&config_toml.file_path);

    let toml_content = fs::read_to_string(&config_path)?;
    // .expect(&format!("Failed to read config file at '{}'", config_toml.file_path));

    let config_result =
        parse_config_from_str(&toml_content).map_err(|e| Box::new(e) as Box<dyn Error>);

    let config = config_result?;
    // .expect("Failed to parse GameSettings.toml. Check for syntax errors or missing fields.");

    Ok(config)
    // config
}

pub(crate) fn load_factions() -> Result<Vec<Faction>, Box<dyn Error>> {
    let mut factions: Vec<Faction> = Vec::new();

    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Failed to get parent dir");
    let assets_dir = exe_dir?.join("assets");
    let b = assets_dir.to_string_lossy();
    println!("{}", b);
    let factions_folder_path = assets_dir.join("Factions");
    // let dd = d.into_iter()

    for entry in WalkDir::new(&factions_folder_path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        println!("{}", f_name);
    }

    let mut factions_names: Vec<String> = Vec::new();
    for entry in WalkDir::new(&factions_folder_path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        println!("{}", f_name);
        factions_names.push(f_name);
    }

    if (factions_names.is_empty()) {
        //THis is boxing text for Err return 0_o
        return Err("Factions folder is empty".into());
    }

    let mut factions_list: Vec<Faction> = Vec::new();
    for faction_name in factions_names {
  
        //Checking if there are types
        let faction_folder_path = factions_folder_path.join(&faction_name);
        let faction_types_path = faction_folder_path.join("UnitTypes.toml");
        let toml_content = match fs::read_to_string(&faction_types_path) {
            Err(e) => {
                println!(
                    "Couldn't get the faction's UnitType contents for: {:?},{}",
                    &faction_name, e
                );
                continue;
            }
            Ok(result) => result,
        };

        println!("{}", &toml_content);

        let unit_types_result: UnitTypesArrayToml = match toml::from_str(&toml_content) {
            Err(e) => {
                println!(
                    "Couldn't parse the faction's UnitType contents for: {:?},{}",
                    &faction_name, e
                );
                continue;
            }
            Ok(result) => result,
        };

        println!("{:#?}", &unit_types_result);

        if unit_types_result.UnitTypes.is_empty(){
            eprintln!("There is no usable Unit Type for {:?} faction loaded. Skipping faction", faction_name);
            continue
        }

        let mut faction: Faction = Faction::new(faction_name);
        
        
        let mut male_names_list: Vec<String> = Vec::new();
        let mut female_names_list: Vec<String> = Vec::new();
        let mut male_names_folder_location =  faction_folder_path.join("MaleNames");
        let mut male_names_fist_names_file =  male_names_folder_location.join("FirstNames.toml");
        
            // let content = fs::read_to_string(path)?;
                        // if let Ok(content) = fs::read_to_string(path){
                        //         // let data: NameToml = toml::from_str(&content)
                            
                        // }



        




        for unit_type_data in unit_types_result.UnitTypes{
            
        }

        //since there are types, trying to fetch the names

        //building compact type for caller.
    }

    // println!("{}",dd);
    if factions.is_empty(){
        return Err("There is no usable faction loaded.".into());
    }
    Ok(factions)
}

pub(crate) fn get_folder() {}

// pub mod toml_parser;
// use crate::game_data_retriever::*;
