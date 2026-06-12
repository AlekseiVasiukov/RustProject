use serde::Deserialize;
use std::{env, error::Error};

pub struct ConfigToml {
    file_path: String,
}

impl ConfigToml {
    pub fn new() -> Self {
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir_option = exe_path.parent().expect("Failed to get parent dir");
        let settings_dir = exe_dir_option.join("assets").join("Settings");
        let config_file_path = settings_dir.join("GameSettings.toml");
        let config_file_path_string = config_file_path.into_os_string().into_string().unwrap();
        println!("{config_file_path_string}");
        Self {
            file_path: config_file_path_string,
        }
    }
}
