use crate::game_data_retriever::{self, Combatant, Config, Faction};

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
};

#[derive(Debug, Clone)]
pub struct GameInstance {
    id: String,
    pub factions: Vec<Faction>,
    config: Config,
    pub game_result: Option<GameResult>,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    generated_combatants: BTreeMap<usize, BTreeMap<usize, Combatant>>,
}

impl GameResult {
    pub fn new(
        generated_combatants: BTreeMap<usize, BTreeMap<usize, Combatant>>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            generated_combatants: generated_combatants,
        })
    }
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

    pub fn calculate_game_result(&mut self) -> Result<GameResult, Box<dyn Error>> {
        //    let mut units: BTreeMap<usize, Option<usize>> = BTreeMap::new();
        // units.insert(0, Some(0));
        // units.insert(1, None);
        // units.insert(2, None);
        // units.insert(3, None);

        //         pub id: String,
        // pub faction: String,
        // pub unittype: String,
        // pub firstName: String,
        // pub lastName: Option<String>,
        // pub parameters: BTreeMap<String, u8>,

        // let character1 = Combatant::new(
        // //     id: "42",
        // //     faction: Humans,
        // //     unittype

        //         let character1 = Combatant::new(
        //             id

        // // );

        let character1 = Combatant::new(
            "42".to_string(),
            "Humans".to_string(),
            "knight".to_string(),
            Some("Knight".to_string()),
            Some("Gorge".to_string()),
            Some("From Jungle".to_string()),
        );
        let our_units: BTreeMap<usize, Combatant> = BTreeMap::new();
        our_units.insert(0, character1);

        let character2 = Combatant::new(
            "666".to_string(),
            "Demons".to_string(),
            "imp".to_string(),
            Some("Imp".to_string()),
            Some("Jujuka".to_string()),
            Some("Minoris".to_string()),
        );        
        let opponents_units: BTreeMap<usize, Combatant> = BTreeMap::new();
        opponents_units.insert(0, character2);
        let sides: BTreeMap<usize, BTreeMap<usize, Combatant>> = BTreeMap::new();
        sides.insert(0, our_units);
        sides.insert(1, opponents_units);

        let generated_combatants:BTreeMap<usize,BTreeMap<usize,Combatant>> = BTreeMap::new();
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
