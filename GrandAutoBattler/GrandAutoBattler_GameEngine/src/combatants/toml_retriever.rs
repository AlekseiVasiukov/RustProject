use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    file_path: String,
}

impl ConfigToml {
    pub fn new() -> Self {
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get parent dir");
        let settings_dir = exe_dir.join("assets").join("Settings");
        let config_file_path = settings_dir.join("GameSettings.toml");
        let config_file_path_string = config_file_path.to_str().expect("Failed to convert path to string").to_string();
        
        Self {
            file_path: config_file_path_string,
        }
    }
}

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

fn parse_config_from_str(toml_string: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(toml_string)
}

pub fn get_config() -> Config {
    let config_toml = ConfigToml::new();
    let config_path = PathBuf::from(&config_toml.file_path);

    let toml_content = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read config file at '{}'", config_toml.file_path));

    let config_result = parse_config_from_str(&toml_content);

    let config = config_result
        .expect("Failed to parse GameSettings.toml. Check for syntax errors or missing fields.");

    config
}