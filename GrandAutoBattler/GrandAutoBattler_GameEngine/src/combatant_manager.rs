use std::error::Error;

// use game_instance::Combatant;
// use game_data_retriever::{Faction, UnitType};
use crate::game_data_retriever::{self, Config, Faction, UnitType};


#[derive(Debug, Clone)]
pub struct Combatant {
    id: String,
    faction: String,
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    // parameters: BTreeMap<String, u8>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl Combatant {
    pub fn new(
        id: String,
        faction: String,
        unit_type_name: String,
        unit_type_pretty_name: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        Self {
            id: id,
            faction: faction,
            unit_type_name: unit_type_name,
            unit_type_pretty_name: unit_type_pretty_name,
            // parameters: BTreeMap<String, u8>,
            first_name: first_name,
            last_name: last_name,
        }
    }
}

// pub(crate) fn generate_combatant() 
    // pub(crate) combatants_ids: IdRegistry,
            // combatants_ids: IdRegistry::new(),
