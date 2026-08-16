mod game_data_retriever;
mod game_instance;
mod toml_parser;

use game_instance::*;

use std::collections::BTreeMap;
use std::io::{Write, stdout};

use std::error::Error;

use crate::game_data_retriever::IdRegistry;

pub struct GameInstancesManager {
    pub(crate) game_instances_id: IdRegistry,
    pub games_by_ids: BTreeMap<String, GameInstance>,
}

impl GameInstancesManager {
    pub fn new() -> Self {
        Self {
            game_instances_id: IdRegistry::new(),
            games_by_ids: BTreeMap::new(),
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
                self.games_by_ids.insert(new_game_id.clone(), result.clone());
                println!("Initialization succeed. {:#?}", &result);
                return new_game_id;
            }
        };
    }

    pub fn GetGameDataById(&self, key: &String) -> GameInstance{
        let game_instance = self.games_by_ids.get(key);
        return game_instance.unwrap().clone();
    }
}
