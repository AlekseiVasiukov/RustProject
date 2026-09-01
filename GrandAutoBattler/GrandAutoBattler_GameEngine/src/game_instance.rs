use crate::game_data_retriever::{self, Config, Faction};

use std::error::Error;

#[derive(Debug, Clone)]
pub struct GameInstance {
    id: String,
    pub factions: Vec<Faction>,
    config: Config,
    pub game_result: Option<GameResult>,
}

#[derive(Debug, Clone)]
pub struct GameResult {}

// pub impl GameResult {}

impl GameInstance {
    pub fn new(new_id: &String) -> Result<Self, Box<dyn Error>> {
        let config = game_data_retriever::get_config()?;
        let factions = game_data_retriever::get_factions()?;
        Ok(Self {
            id: new_id.clone(),
            factions,
            config,
            game_result: None,
        })
    }

    pub fn calculate_game_result(&mut self) -> Result<GameResult, Box<dyn Error>> {
        let result = GameResult {}; 
        self.game_result = Some(result.clone()); 
        Ok(result)
    }

    // pub fn calculate_game_result(&mut self) -> Result<GameResult, Box<dyn Error>> {
    //     Ok(GameResult {})
    // }
}
