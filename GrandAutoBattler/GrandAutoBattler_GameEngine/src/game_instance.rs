use crate::game_data_retriever::{self, Config, Faction, UnitType};
use crate::combatant_manager::Combatant;
use crate::game_result_manager::GameResult;

use std::{
    collections::{BTreeMap},
    error::Error,
};

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
