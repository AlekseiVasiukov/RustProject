use crate::battle_journal_manager::{self, *};
use crate::combatant_manager::{self, Combatant};
use crate::error_types::{GameEngineError, Result};
use crate::game_data_retriever::{self, Config, Faction, UnitType};
use crate::game_result_manager::GameResult;
use crate::game_result_manager::*;

use crate::game_data_retriever::IdRegistry;
use rand::RngExt;
use rand::seq::IndexedRandom;
// use rand::seq::SliceRandom;
use std::collections::BTreeMap;
use std::io::{Write, stdout};

#[derive(Debug, Clone)]
pub struct GameInstance {
    id: String,

    config: Config,
    pub factions: Vec<Faction>,
    pub game_result: Option<GameResult>,
    pub(crate) combatants_register_id: IdRegistry,
    pub(crate) combatants_by_id: BTreeMap<String, Combatant>,
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
    pub fn new(new_id: &String) -> Result<Self> {
        let config = game_data_retriever::get_config()?;
        let available_factions = game_data_retriever::get_factions()?;

        Ok(Self {
            id: new_id.clone(),
            config,
            factions: available_factions,
            game_result: None,

            combatants_register_id: IdRegistry::new(),
            combatants_by_id: BTreeMap::new(),
        })
    }

    pub fn calculate_game_result(
        &mut self,
        selected_factions: &BTreeMap<usize, usize>,
        selected_unittypes: &BTreeMap<usize, Option<usize>>,
    ) -> Result<GameResult> {
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

        //calculating how much fighters
        let number_of_our_non_null_combatants: usize =
            selected_unittypes.values().filter(|v| v.is_some()).count();
        let mut our_units: Vec<Combatant> = Vec::new();

        // let mut combatant_manager = combatant_manager::CombatantManager::new();

        for (_key, value) in selected_unittypes.iter().filter(|(_, v)| v.is_some()) {
            let new_id = self.combatants_register_id.generate_id();

            let faction_index = *selected_factions.get(&0).ok_or_else(|| {
                GameEngineError::FactionNotFound(
                    "No faction selected for player (index 0)".to_string(),
                )
            })?;

            let faction = self.factions.get(faction_index).ok_or_else(|| {
                GameEngineError::FactionNotFound(format!(
                    "Faction index {} out of range",
                    faction_index
                ))
            })?;

            let unittype_index = value.unwrap_or_default();
            let unittype = faction
                .unit_types
                .get(unittype_index)
                .ok_or_else(|| GameEngineError::UnitTypeNotFound(unittype_index))?
                .clone();

            let new_combatant =
                Combatant::generate_based_on_data(new_id, faction.clone(), unittype)?;

            our_units.push(new_combatant);
        }

        let mut their_units: Vec<Combatant> = Vec::new();
        for n in 0..number_of_our_non_null_combatants {
            let new_id = self.combatants_register_id.generate_id();

            let mut rng = rand::rngs::ThreadRng::default();

            let opponent_faction_index = *selected_factions.get(&1).ok_or_else(|| {
                GameEngineError::FactionNotFound(
                    "No factions selected for opponent (index 1)".to_string(),
                )
            })?;

            let faction = self.factions.get(opponent_faction_index).ok_or_else(|| {
                GameEngineError::FactionNotFound(format!(
                    "Faction index {} out of range",
                    opponent_faction_index
                ))
            })?;

            if let Some(unittype) = faction.unit_types.choose(&mut rng) {
                let unittype_clone = unittype.clone();
                let new_combatant =
                    Combatant::generate_based_on_data(new_id, faction.clone(), unittype_clone)?;

                their_units.push(new_combatant);
            }
        }

        let mut sides: BTreeMap<usize, Vec<Combatant>> = BTreeMap::new();
        sides.insert(0, our_units);
        sides.insert(1, their_units);

        let battle_journal = battle_journal_manager::BattleJournal::generate_based_on_data(&sides);
        // let battle_journal = battle_journal_manager::BattleJournal::generate_based_on_data(&sides);

        Ok(GameResult::new(sides, battle_journal)?)
    }
}

#[derive(Debug, Clone)]
pub struct GameInstancesManager {
    pub(crate) game_instances_ids: IdRegistry,
    pub(crate) games_by_ids: BTreeMap<String, GameInstance>,
}

impl GameInstancesManager {
    pub fn new() -> Self {
        Self {
            game_instances_ids: IdRegistry::new(),
            games_by_ids: BTreeMap::new(),
        }
    }

    pub fn create_new_game_instance(&mut self) -> Result<String> {
        tracing::info!("Trying to initialize new Game Instance");
        let new_game_id = self.game_instances_ids.generate_id();

        match GameInstance::new(&new_game_id) {
            Err(e) => {
                tracing::error!("Initialization failed. Error: {} ", e);
                Err(e)
            }
            Ok(game_instance) => {
                self.games_by_ids.insert(new_game_id.clone(), game_instance);
                tracing::info!("Initialization succeeded with ID: {}", new_game_id);
                Ok(new_game_id)
            }
        }
    }

    pub fn get_game_data_by_id(&self, game_id: &str) -> Result<GameInstance> {
        self.games_by_ids
            .get(game_id)
            .cloned()
            .ok_or_else(|| GameEngineError::GameInstanceNotFound(game_id.to_string()))
    }

    pub fn get_game_result_by_id_and_selection(
        &mut self,
        game_id: &str,
        selected_factions: &BTreeMap<usize, usize>,
        selected_unittypes: &BTreeMap<usize, Option<usize>>,
    ) -> Result<GameResult> {
        let game_instance_ref = self
            .games_by_ids
            .get_mut(game_id)
            .ok_or_else(|| GameEngineError::GameInstanceNotFound(game_id.to_string()))?;

        let result =
            game_instance_ref.calculate_game_result(selected_factions, selected_unittypes)?;
        Ok(result)
    }

    pub fn get_available_factions(&self, game_id: &String) -> Result<Vec<String>> {
        let mut faction_names: Vec<String> = Vec::new();
        let game_instance = self.get_game_data_by_id(&game_id)?;

        for (i, faction) in game_instance.factions.iter().enumerate() {
            faction_names.push(faction.name.clone());
        }

        Ok(faction_names)
    }

    pub fn get_available_unit_types(
        &self,
        game_id: &String,
    ) -> Result<BTreeMap<usize, Vec<String>>> {
        let game_instance = self.get_game_data_by_id(&game_id);

        let mut unittypes_names: BTreeMap<usize, Vec<String>> = BTreeMap::new();

        for (i, faction) in game_instance?.factions.iter().enumerate() {
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

        Ok(unittypes_names)
    }
}
