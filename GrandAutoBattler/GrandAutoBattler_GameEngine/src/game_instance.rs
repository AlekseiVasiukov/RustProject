// mod game_data_retriever;

// mod game_data_retriever;
// mod toml_parser;
use crate::game_data_retriever::{self, Config, Faction};

use std::error::Error;

#[derive(Debug)]
pub struct GameInstance {
    id: i32,
    factions: Vec<Faction>,
    config: Config,
}

impl GameInstance {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        //    self.id = 666;
        let config = game_data_retriever::get_config()?;
        let factions = game_data_retriever::get_factions()?;
        // self.config = config;
        // let mut config = load_config()?;
        // Err(e.into())
        // Ok(())
        Ok(Self {
            id: 0,
            factions: factions,
            config,
        })
    }

}
