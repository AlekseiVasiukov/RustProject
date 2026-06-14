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
        let config = game_data_retriever::get_config()?;
        let factions = game_data_retriever::get_factions()?;
        Ok(Self {
            id: 0,
            factions,
            config,
        })
    }
}
