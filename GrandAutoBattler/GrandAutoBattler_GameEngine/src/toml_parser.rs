use crate::error_types::{GameEngineError, Result};
use rand::RngExt;
use serde::Deserialize;
use std::env;
// use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use rand::prelude::IndexedRandom;

use crate::game_data_retriever::*;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    file_path: String,
}

impl ConfigToml {
    pub fn new() -> Result<Self> {
        let exe_path = env::current_exe()?;
        // let exe_dir = exe_path.parent().ok_or("Failed to get parent dir");
        let exe_dir = exe_path.parent().ok_or_else(|| {
            GameEngineError::ConfigNotFound("Failed to get executable parent directory".to_string())
        });

        let settings_dir = exe_dir?.join("assets").join("Settings");
        let config_file_path = settings_dir.join("GameSettings.toml");

        let config_file_path_string = config_file_path
            .to_str()
            .ok_or_else(|| {
                GameEngineError::ConfigNotFound("Failed to convert path to string".to_string())
            })?
            // .ok_or("Failed to convert path to string (possibly invalid UTF-8)")?
            .to_string();

        Ok(Self {
            file_path: config_file_path_string,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
struct UnitTypesArrayToml {
    UnitTypes: Vec<UnitTypesToml>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct UnitTypesToml {
    pub(crate) unit_type_name: String,
    pub(crate) unit_type_pretty_name: Option<String>,
    pub(crate) females_only: Option<bool>,
    pub(crate) health: u8,
    pub(crate) attack: u8,
    pub(crate) range: u8,
    pub(crate) defense: u8,
}

#[derive(Debug, Deserialize)]
struct NameToml {
    names: Vec<String>,
}

fn parse_config_from_str(toml_string: &str) -> Result<Config> {
    Ok(toml::from_str(toml_string)?)
}

pub(crate) fn load_config() -> Result<Config> {
    let config_toml = ConfigToml::new()?;
    let config_path = PathBuf::from(&config_toml.file_path);

    let toml_content = fs::read_to_string(&config_path)?;

    let config = parse_config_from_str(&toml_content)?;
    // let config_result =
    //     parse_config_from_str(&toml_content).map_err(|e| Box::new(e) as Box<dyn Error>);
    // let config = config_result?;

    Ok(config)
}

pub(crate) fn load_factions() -> Result<Vec<Faction>> {
    // pub(crate) fn load_factions() -> Result<Vec<Faction>, Box<dyn Error>> {
    let mut factions: Vec<Faction> = Vec::new();

    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| {
        GameEngineError::ConfigNotFound("Failed to get executable parent directory".to_string())
    })?;
    // let exe_dir = exe_path.parent().ok_or("Failed to get parent dir");
    let assets_dir = exe_dir.join("assets");
    // let b = assets_dir.to_string_lossy();
    let factions_folder_path = assets_dir.join("Factions");

    // for entry in WalkDir::new(&factions_folder_path)
    //     .max_depth(1)
    //     .min_depth(1)
    //     .into_iter()
    //     .filter_map(Result::ok)
    //     .filter(|e| e.file_type().is_file())
    // {
    //     let f_name = String::from(entry.file_name().to_string_lossy());
    // }

    let mut factions_names: Vec<String> = Vec::new();

    // let factions_names: Vec<String> = WalkDir::new(&factions_folder_path)
    //     .max_depth(1)
    //     .min_depth(1)
    //     .into_iter()
    //     .filter_map(Result::ok)
    //     .filter(|e| e.file_type().is_dir())
    //     .map(|e| String::from(e.file_name()).to_string())
    //     .collect();

    for entry in WalkDir::new(&factions_folder_path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        factions_names.push(f_name);
    }

    if factions_names.is_empty() {
        return Err(GameEngineError::FactionNotFound(
            "Factions folder is empty or contains no subdirectories.".to_string(),
        ));
        // return Err("Factions folder is empty".into());

        //THis is boxing text for Err return 0_o

        // return Err("Factions folder is empty".into());
    }

    // let mut factions_list: Vec<Faction> = Vec::new();
    for faction_name in factions_names {
        //Checking if there are types
        let faction_folder_path = factions_folder_path.join(&faction_name);
        let faction_types_path = faction_folder_path.join("UnitTypes.toml");
        let toml_content = match fs::read_to_string(&faction_types_path) {
            Err(e) => {
                println!(
                    "Cound't read UnitTypes.toml for faction '{}': {}",
                    faction_name, e
                );
                // println!(
                //     "Couldn't get the faction's UnitType contents for: {:?},{}",
                //     &faction_name, e
                // );
                continue;
            }
            Ok(result) => result,
        };

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

        if unit_types_result.UnitTypes.is_empty() {
            println!(
                "No unittypes defined for faction '{}. Skipping.",
                faction_name
            );
            // eprintln!(
            //     "There is no usable Unit Type for {:?} faction loaded. Skipping faction",
            //     faction_name
            // );
            continue;
        }

        let mut faction: Faction = Faction::new(faction_name.clone());
        faction.unit_types = unit_types_result
            .UnitTypes
            .into_iter()
            .map(Into::into)
            .collect();

        //since there are types already parsed, trying to fetch the optional names
        let mut male_names_folder_location = faction_folder_path.join("MaleNames");
        let mut male_names_first_names_file = male_names_folder_location.join("FirstNames.toml");
        let mut male_names_last_names_file =
            male_names_folder_location.join("MiddleLastNames.toml");

        // if let Ok(result) pattern, allowing to continue logic and silence the errors.
        if let Ok(content) = fs::read_to_string(male_names_first_names_file)
            && let Ok(data) = toml::from_str::<NameToml>(&content)
        {
            //without first names, last names doesn't make any sense.
            faction.male_first_names = data.names;
            if let Ok(content) = fs::read_to_string(male_names_last_names_file)
                && let Ok(data) = toml::from_str::<NameToml>(&content)
            {
                faction.male_last_names = data.names;
            }
        }

        //female
        let mut female_names_folder_location = faction_folder_path.join("FemaleNames");
        let mut female_names_first_names_file =
            female_names_folder_location.join("FirstNames.toml");
        let mut female_names_last_names_file =
            female_names_folder_location.join("MiddleLastNames.toml");

        if let Ok(content) = fs::read_to_string(female_names_first_names_file)
            && let Ok(data) = toml::from_str::<NameToml>(&content)
        {
            //without first names, last names doesn't make any sense.
            faction.female_first_names = data.names;
            if let Ok(content) = fs::read_to_string(female_names_last_names_file)
                && let Ok(data) = toml::from_str::<NameToml>(&content)
            {
                faction.female_last_names = data.names;
            }
        }

        factions.push(faction);
    }

    if factions.is_empty() {
        return Err(GameEngineError::FactionNotFound(
            "No usable factions could be loaded from disk".to_string(),
        ));
        // return Err("There is no usable faction loaded.".into());
    }
    Ok(factions)
}
