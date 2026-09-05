mod game_data_retriever;
pub mod game_instance;
mod toml_parser;

use game_instance::*;

use std::collections::BTreeMap;
use std::io::{Write, stdout};

use std::error::Error;

use crate::game_data_retriever::IdRegistry;

pub struct GameInstancesManager {
    pub(crate) game_instances_ids: IdRegistry,
    pub games_by_ids: BTreeMap<String, GameInstance>,

    pub(crate) combatants_ids: IdRegistry,
}

impl GameInstancesManager {
    pub fn new() -> Self {
        Self {
            game_instances_ids: IdRegistry::new(),
            games_by_ids: BTreeMap::new(),

            combatants_ids: IdRegistry::new(),
        }
    }

    pub fn GetNewGameInstanceId(&mut self) -> String {
        println!("Trying to initialize new Game Instance");

        let new_game_id = self.game_instances_ids.generate_id();
        let result = match GameInstance::new(&new_game_id) {
            Err(e) => {
                println!(
                    "Initialization failed. Game continuation is not possible. Error: {}",
                    e
                );

                return String::new();
            }

            Ok(result) => {
                self.games_by_ids
                    .insert(new_game_id.clone(), result.clone());
                println!("Initialization succeed. ");
                // println!("Initialization succeed. {:#?}", &result);
                return new_game_id;
            }
        };
    }

    pub fn GetGameDataById(&self, key: &String) -> GameInstance {
        let game_instance = self.games_by_ids.get(key);
        return game_instance.unwrap().clone();
    }

    pub fn GetGameResultByIdAndSelection(
        &mut self,
        key: &String,
        selected_factions: &BTreeMap<usize, usize>,
        selected_unittypes: &BTreeMap<usize, Option<usize>>,
    ) -> GameResult {
        let game_instance_ref = self
            .games_by_ids
            .get_mut(key)
            .expect("Game instance not found");
        // let mut game_instance_clone = game_instance_ref.clone();
        // let game_instance: Option<&GameInstance> = self.games_by_ids.get_mut(key).as_deref();
        // let result = GameInstance::calculate_game_result(&mut (game_instance.unwrap()));
        let result = game_instance_ref
            .calculate_game_result(selected_factions, selected_unittypes)
            .expect("Failed to calculate game result");
        result
    }
}
