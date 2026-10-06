use rand::Rng;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use rand::RngExt;

use std::collections::BTreeMap;

// use std::error::Error;
use crate::error_types::{GameEngineError, Result};

use crate::game_data_retriever::{self, Config, Faction, UnitType};

pub struct CombatantManager {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    pub id: String,
    pub faction_name: String,
    pub unit_type_name: String,
    pub unit_type_pretty_name: Option<String>,
    pub parameters: BTreeMap<String, u8>,
    pub current_health_points: usize,
    pub health_points: usize,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Combatant {
    pub fn generate_based_on_data(
        id: String,
        faction: Faction,
        mut unittype: UnitType,
    ) -> Result<Self> {
        //choosing name:
        let mut first_name: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut rng = rand::rng();
        // let mut gender = 0;
        let gender = rand::seq::IndexedRandom::choose(&[0, 1], &mut rng)
            .copied()
            .unwrap_or(0);

        let config = game_data_retriever::get_config().unwrap();

        // if !unittype.females_only {
        // gender = *[0, 1].choose(&mut rng).unwrap();
        // gender = *[0, 1].choose(&mut rng).copied().uwrap_or(0);
        // }

        if gender == 0 {
            // Female names - use .copied() to avoid clone, and fallback with .copied().ok()

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
        //     first_name = faction.female_first_names.choose(&mut rng).copied();
        //     last_name = faction.female_last_names.choose(&mut rng).copied();
        // } else {
        //     // Male names
        //     first_name = faction.male_first_names.choose(&mut rng).copied();
        //     last_name = faction.male_last_names.choose(&mut rng).copied();
        // }

        // if gender == 0 {
        //     //female names
        //     if !faction.female_first_names.is_empty() {
        //         first_name = Some(faction.female_first_names.choose(&mut rng)).copied();
        //     }

        //     if !faction.female_last_names.is_empty() {
        //         last_name = Some(faction.female_last_names.choose(&mut rng).copied());
        //     }
        // } else {
        //     //male names
        //     if !faction.male_first_names.is_empty() {
        //         first_name = Some(faction.male_first_names.choose(&mut rng).copied());
        //     }

        //     if !faction.male_last_names.is_empty() {
        //         last_name = Some(faction.male_last_names.choose(&mut rng).copied());
        //     }
        // }

        Self::apply_random_variance_to_parameters(
            &mut unittype.parameters_set,
            &config.PARAMETER_MAXIMUM,
            &config.PARAMETER_MINIMUM,
        );

        let hp = Self::generate_health_points(&config, unittype.parameters_set["health"]);

        Ok(Self {
            id,
            faction_name: faction.name,
            unit_type_name: unittype.unit_type_name.clone(),
            unit_type_pretty_name: unittype.unit_type_pretty_name.clone(),
            parameters: unittype.parameters_set.clone(),
            health_points: hp.clone(),
            current_health_points: hp.clone(),
            first_name,
            last_name,
        })
    }

    pub(crate) fn apply_random_variance_to_parameters(
        parameters: &mut BTreeMap<String, u8>,
        maximum: &u8,
        minimum: &u8,
    ) {
        let mut rng = rand::rng();
        //can add bonus points to many parameter, but guarantying at least one will be unaffected.
        let number_of_applicable_parameters = parameters.len().saturating_sub(1);

        //Destributing bonus points
        let mut bonus_affected_indices = vec![0; number_of_applicable_parameters];
        for i in 0..(number_of_applicable_parameters) {
            bonus_affected_indices[i] = rand::rng().random_range(0..parameters.len());
        }

        let keys: Vec<_> = parameters.keys().cloned().collect();
        for &i in &bonus_affected_indices {
            if let Some(key) = keys.get(i) {
                let val = parameters.get_mut(key).unwrap();
                if *val < *maximum {
                    *val = val.saturating_add(1).min(*maximum);
                }
            }
        }
        //the indices withou bonuses, even if it's only one will be randomly chosen for decreasing by 1 as penalty.
        let mut unaffected_indices: Vec<usize> = Vec::new();
        for i in 0..parameters.len() {
            if !bonus_affected_indices.contains(&i) {
                unaffected_indices.push(i);
            }
        }
        //decreasing left parameter
        if !unaffected_indices.is_empty() {
            let random_unaffected = Some(unaffected_indices.choose(&mut rng).unwrap()).unwrap();

            if let Some(key) = keys.get(*random_unaffected) {
                let val = parameters.get_mut(key).unwrap();
                if *val < *maximum {
                    *val = val.saturating_sub(1).max(*minimum);
                }
            }
        }
    }

    pub(crate) fn generate_health_points(config: &Config, healthParameter: u8) -> usize {
        let health_bonus_points = healthParameter as f32 - config.HEALTH_MIDDLE_VALUE_PARAMETER;
        let health_bonus = config.HEALTH_PERCENTAGE_PER_POINT * health_bonus_points;
        let health_points = config.BASIC_HEALTH + (config.BASIC_HEALTH * health_bonus) / 100.0;

        if health_points < 0.0 {
            1
        } else {
            health_points.trunc() as usize
        }
    }
}
