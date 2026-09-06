use rand::seq::SliceRandom;
use std::error::Error;

// use game_instance::Combatant;
// use game_data_retriever::{Faction, UnitType};
use crate::game_data_retriever::{self, Config, Faction, UnitType};

pub struct CombatantManager {}

#[derive(Debug, Clone)]
pub struct Combatant {
    id: String,
    faction_name: String,
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    // parameters: BTreeMap<String, u8>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl Combatant {
    pub fn generate_based_on_data(
        id: String,
        faction: Faction,
        unittype: UnitType,
        // unit_type_name: String,
        // unit_type_pretty_name: Option<String>,
        // first_name: Option<String>,
        // last_name: Option<String>,
    ) -> Self {
        //choosing name:

        let mut first_name: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut rng = rand::thread_rng();
        let mut gender = 0;

        // if !unittype.females_only {
        gender = *[0, 1].choose(&mut rng).unwrap();
        // }

        if gender == 0 {
            //female names
            if !faction.female_first_names.is_empty() {
                first_name = Some(faction.female_first_names.choose(&mut rng).unwrap().clone());
            }

            if !faction.female_last_names.is_empty() {
                last_name = Some(faction.female_last_names.choose(&mut rng).unwrap().clone());
            }
        } else {
            //male names
            if !faction.male_first_names.is_empty() {
                first_name = Some(faction.male_first_names.choose(&mut rng).unwrap().clone());
            }

            if !faction.male_last_names.is_empty() {
                last_name = Some(faction.male_last_names.choose(&mut rng).unwrap().clone());
            }
        }

        Self {
            id: id,
            faction_name: faction.name,
            unit_type_name: unittype.unit_type_name,
            unit_type_pretty_name: unittype.unit_type_pretty_name,
            // parameters: BTreeMap<String, u8>,
            first_name: first_name,
            last_name: last_name,
        }
    }
}

// pub(crate) fn generate_combatant()
// pub(crate) combatants_ids: IdRegistry,
// combatants_ids: IdRegistry::new(),
