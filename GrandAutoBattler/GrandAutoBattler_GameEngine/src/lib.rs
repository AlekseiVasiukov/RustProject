mod game_data_retriever;
mod game_instance;
mod toml_parser;

use game_instance::*;

use std::io::{Write, stdout};

use std::error::Error;

use crate::game_data_retriever::IdRegistry;

pub struct GameInstancesManager {
    pub(crate) game_instances_id: IdRegistry,
}

impl GameInstancesManager {
    pub fn new() -> Self {
        Self {
            game_instances_id: IdRegistry::new(),
        }
    }

    pub fn GetNewGameInstanceId(&mut self) -> String {
        println!("Trying to initialize new Game Instance");

        let new_game_id = self.game_instances_id.generate_id();
        let result = match GameInstance::new(&new_game_id) {
            Err(e) => {
                println!(
                    "Initialization failed. Game continuation is not possible. Error: {}",
                    e
                );

                return String::new();
            }

            Ok(result) => {
                println!("Initialization succeed. {:#?}", result);
                return new_game_id;
            }
        };
    }
}
