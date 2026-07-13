use crate::game_data_retriever::{self, Config, Faction};

use std::error::Error;

#[derive(Debug, Clone)]
pub struct GameInstance {
    id: String,
    pub factions: Vec<Faction>,
    config: Config,
}

impl GameInstance {
    pub fn new(new_id: &String) -> Result<Self, Box<dyn Error>> {
        let config = game_data_retriever::get_config()?;
        let factions = game_data_retriever::get_factions()?;
        Ok(Self {
            id: new_id.clone(),
            factions,
            config,
        })
    }
}
