use crate::combatant_manager::Combatant;
use crate::game_data_retriever::{self, Config, Faction, UnitType};
use crate::game_result_manager::GameResult;

use crate::game_result_manager::*;

use std::io::{Write, stdout};

use crate::game_data_retriever::IdRegistry;
use std::{collections::BTreeMap, error::Error};

#[derive(Debug, Clone)]
pub struct GameInstance {
    id: String,
    pub factions: Vec<Faction>,
    config: Config,
    pub game_result: Option<GameResult>,
}

// #[derive(Debug, Clone)]
// pub struct Combatant {
//     pub id: String,
//     pub faction: String,
//     pub unit_type: String,
//     pub first_name: String,
//     pub last_name: Option<String>,
//     // pub parameters: BTreeMap<String, u8>,
// }

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

    pub fn calculate_game_result(
        &mut self,
        selected_factions: &BTreeMap<usize, usize>,
        selected_unittypes: &BTreeMap<usize, Option<usize>>,
    ) -> Result<GameResult, Box<dyn Error>> {
        //Plan

        //1) Get our faction
        //2) Get number of our fighters
        //3) generate number of combatants with their
        //id
        //name
        //unittype
        //max healthpoints
        //current healthpoints
        //parameters
        //4) depending on number of our fighters, generate opposing combatants

        //5) Genarate Battle journal...

        let character1 = Combatant::new(
            "42".to_string(),
            "Humans".to_string(),
            "knight".to_string(),
            Some("Knight".to_string()),
            Some("Gorge".to_string()),
            Some("From Jungle".to_string()),
        );
        let mut our_units: BTreeMap<usize, Combatant> = BTreeMap::new();
        our_units.insert(0, character1);

        let character2 = Combatant::new(
            "666".to_string(),
            "Demons".to_string(),
            "imp".to_string(),
            Some("Imp".to_string()),
            Some("Jujuka".to_string()),
            Some("Minoris".to_string()),
        );
        let mut opponents_units: BTreeMap<usize, Combatant> = BTreeMap::new();
        opponents_units.insert(0, character2);
        let mut sides: BTreeMap<usize, BTreeMap<usize, Combatant>> = BTreeMap::new();
        sides.insert(0, our_units);
        sides.insert(1, opponents_units);

        let generated_combatants: BTreeMap<usize, BTreeMap<usize, Combatant>> = BTreeMap::new();
        match GameResult::new(sides) {
            Ok(result) => {
                return Ok(result);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    //    match GameResult::new(generated_combatants){
    //         Ok(game_result)=>{
    //              self.result = Some(result.clone());
    //              return result;

    // }

    //     }
    //     Err(e)=>{return e}
    //         // Ok(result.unwrap().clone());

    // pub fn calculate_game_result(&mut self) -> Result<GameResult, Box<dyn Error>> {
    //     Ok(GameResult {})
    // }
}

pub struct GameInstancesManager {
    pub(crate) game_instances_ids: IdRegistry,
    pub games_by_ids: BTreeMap<String, GameInstance>,
}

impl GameInstancesManager {
    pub fn new() -> Self {
        Self {
            game_instances_ids: IdRegistry::new(),
            games_by_ids: BTreeMap::new(),
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

    pub fn GetAvailableFactions(&self, game_id: &String) -> Vec<String> {
        let mut faction_names: Vec<String> = Vec::new();
        let game_instance = self.GetGameDataById(&game_id);

        for (i, faction) in game_instance.factions.iter().enumerate() {
            faction_names.push(faction.name.clone());
        }

        faction_names
    }

    pub fn GetAvailableUnitTypes(&self, game_id: &String) -> BTreeMap<usize, Vec<String>> {
        // let mut faction_names: Vec<String> = Vec::new();
        let game_instance = self.GetGameDataById(&game_id);

        let mut unittypes_names: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for (i, faction) in game_instance.factions.iter().enumerate() {
            // faction_names.push(faction.name.clone());

            let mut unittypes_names_of_faction: Vec<String> = Vec::new();
            for unit_type in faction.unit_types.clone() {
                if let Some(pretty_name) = unit_type.unit_type_pretty_name {
                    unittypes_names_of_faction.push(pretty_name);
                } else {
                    unittypes_names_of_faction.push(unit_type.unit_type_name);
                }
            }
            unittypes_names.insert(i, unittypes_names_of_faction);
        }

        unittypes_names
    }
}
