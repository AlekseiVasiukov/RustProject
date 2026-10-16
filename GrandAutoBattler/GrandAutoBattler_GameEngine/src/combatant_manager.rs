use crate::error_types::Result;
use rand::RngExt;
use rand::prelude::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

use crate::game_data_retriever::{self, Config, Faction, UnitType};

pub struct CombatantManager {}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
        let mut rng = rand::rngs::ThreadRng::default();
        let mut gender: usize = if (unittype.females_only) { 0 } else { 1 };
        let mut turn_by = *[0, 1].choose(&mut rng).unwrap_or(&0);

        let config = game_data_retriever::get_config()?;

        if gender == 0 {
            //female names
            first_name = faction.female_first_names.choose(&mut rng).cloned();
            last_name = faction.female_last_names.choose(&mut rng).cloned();
        } else {
            //male names
            first_name = faction.male_first_names.choose(&mut rng).cloned();
            last_name = faction.male_last_names.choose(&mut rng).cloned();
        }

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
        let number_of_applicable_parameters = parameters.len().saturating_sub(1);

        //Destributing bonus points
        let mut bonus_affected_indices = vec![0; number_of_applicable_parameters];
        for i in 0..(number_of_applicable_parameters) {
            bonus_affected_indices[i] = rand::rng().random_range(0..parameters.len());
        }

        let keys: Vec<_> = parameters.keys().cloned().collect();

        for &i in &bonus_affected_indices {
            if let Some(key) = keys.get(i) {
                if let Some(val) = parameters.get_mut(key) {
                    if *val < *maximum {
                        *val = val.saturating_add(1).min(*maximum);
                    }
                }
            }
        }

        let unaffected_indices: Vec<usize> = (0..parameters.len())
            .filter(|i| !bonus_affected_indices.contains(i))
            .collect();

        if let Some(&random_unaffected) = unaffected_indices.choose(&mut rng) {
            if let Some(key) = keys.get(random_unaffected) {
                if let Some(val) = parameters.get_mut(key) {
                    if *val < *minimum {
                        *val = val.saturating_sub(1).max(*minimum);
                    }
                }
            }
        }
    }

    pub(crate) fn generate_health_points(config: &Config, healthParameter: u8) -> usize {
        let health_bonus_points = healthParameter as f32 - config.HEALTH_MIDDLE_VALUE_PARAMETER;
        let health_bonus = config.HEALTH_PERCENTAGE_PER_POINT * health_bonus_points;
        let health_points = config.BASIC_HEALTH + (config.BASIC_HEALTH * health_bonus) / 100.0;

        if health_points < 1.0 {
            1
        } else {
            health_points.trunc() as usize
        }
    }
}
