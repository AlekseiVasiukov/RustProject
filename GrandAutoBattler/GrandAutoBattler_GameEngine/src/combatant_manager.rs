use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use serde::de::value;
use std::collections::BTreeMap;
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
    parameters: BTreeMap<String, u8>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl Combatant {
    pub fn generate_based_on_data(
        id: String,
        faction: Faction,
        mut unittype: UnitType,
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

        let config = game_data_retriever::get_config().unwrap();

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

        Self::apply_random_variance_to_parameters(
            &mut unittype.parameters_set,
            &config.PARAMETER_MAXIMUM,
            &config.PARAMETER_MINIMUM,
        );

        Self {
            id: id,
            faction_name: faction.name,
            unit_type_name: unittype.unit_type_name.clone(),
            unit_type_pretty_name: unittype.unit_type_pretty_name.clone(),
            parameters: unittype.parameters_set.clone(),
            first_name: first_name,
            last_name: last_name,
        }
    }

    pub(crate) fn apply_random_variance_to_parameters(
        parameters: &mut BTreeMap<String, u8>,
        maximum: &u8,
        minimum: &u8,
    ) {
        let mut rng = rand::thread_rng();
        //can add bonus points to many parameter, but guarantying at least one will be unaffected.
        let number_of_applicable_parameters = parameters.len().saturating_sub(1);

        //Destributing bonus points
        let mut bonus_affected_indices = vec![0; number_of_applicable_parameters];
        for i in 0..(number_of_applicable_parameters) {
            bonus_affected_indices[i] = rand::thread_rng().gen_range(0..parameters.len());
        }

        // println!("Indices with bonus points{:?}", bonus_affected_indices);

        let keys: Vec<_> = parameters.keys().cloned().collect();
        for &i in &bonus_affected_indices {
            if let Some(key) = keys.get(i) {
                let val = parameters.get_mut(key).unwrap();
                if *val < *maximum {
                    *val = val.saturating_add(1).min(*maximum);
                }
            }
        }
        // for &i in &bonus_affected_indices {
        //     if let Some(parameter_value) = parameters.iter_mut().nth(i) {
        //         if *parameter_value.1 < *maximum {
        //             *parameter_value.1 += 1u8;
        //         }
        //     }
        // }

        // //the indices withou bonuses, even if it's only one will be randomly chosen for decreasing by 1 as penalty.
        // let mut unaffected_indices: Vec<usize> = Vec::new();
        // for i in 0..parameters.len() {
        //     if !bonus_affected_indices.contains(&i) {
        //         unaffected_indices.push(i);
        //     }
        // }

        // // println!(
        // //     "Indices left for random one point of penalty{:?}",
        // //     unaffected_indices
        // // );

        // if !unaffected_indices.is_empty() {
        //     let random_unaffected = Some(unaffected_indices.choose(&mut rng).unwrap()).unwrap();
        //     // let random_index_in_list = rand::thread_rng().gen_range(0..unaffected_indices.len());
        //     // let actual_index_to_decrease = unaffected_indices[random_index_in_list];

        //     // println!("Chosen index for decrease is {}", actual_index_to_decrease);
        //     if parameters[*random_unaffected] > minimum {
        //         parameters[*random_unaffected] -= 1;
        //     }
        // }
    }
}

// pub(crate) fn generate_combatant()
// pub(crate) combatants_ids: IdRegistry,
// combatants_ids: IdRegistry::new(),
