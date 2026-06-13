use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::collections::HashMap;



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

pub(crate) fn load_factions() ->  Result<Vec<Faction>, Box<dyn Error>>{
    let mut factions: Vec<Faction> = Vec::new();

        let exe_path = env::current_exe()?;
                let exe_dir = exe_path.parent().ok_or("Failed to get parent dir");
        let assets_dir = exe_dir?.join("assets");
        let  b = assets_dir.to_string_lossy();
        println!("{}",b);
        let d = assets_dir.join("Factions");
        // let dd = d.into_iter()

        for entry in WalkDir::new(&d)
        .into_iter()
        .filter_map(Result::ok)
                         .filter(|e| e.file_type().is_file()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
                        println!("{}", f_name);

        }

                for entry in WalkDir::new(&d)
        .into_iter()
        .filter_map(Result::ok)
                         .filter(|e| e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
                        println!("{}", f_name);

        }


                // println!("{}",dd);


    Ok(factions)
}

pub(crate) fn get_folder()  {

}

// pub mod toml_parser;
// use crate::game_data_retriever::*;
