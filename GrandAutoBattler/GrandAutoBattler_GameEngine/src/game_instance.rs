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
        // self.config = config;
        // let mut config = load_config()?;
        // Err(e.into())
        // Ok(())
        Ok(Self {
            id: 0,
            factions: Vec::new(),
            config,
        })
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Initializing unit types");
        // self.initialize_game_config()?;
        self.initialize_unit_types()?;

        Ok(())
    }

    // fn initialize_game_config(&mut self) -> Result<(), Box<dyn Error>> {
    //     self.id = 666;
    //     let config = load_config();
    //     self.config = config;
    //     // let mut config = load_config()?;
    //     // Err(e.into())
    //     Ok(())
    // }

    fn initialize_unit_types(&mut self) -> Result<(), Box<dyn Error>> {
        // let factions = load_available_factions()?;
        Ok(())
        // match factions {
        //     Err(e) => {
        //         println!("Couldn't initialize unit types: {}", e);
        //         Err(e)
        //     }
        //     Ok(result) => {
        //         self.factions = result;
        //         Ok(())
        //     }
        // }
    }
}
