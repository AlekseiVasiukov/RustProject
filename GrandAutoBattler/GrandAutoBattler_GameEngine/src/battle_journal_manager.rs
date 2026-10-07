//                            .Intention (Enum Intentions types)
//Battle Journal .-.. Turn -<
//                            .Outcome .-.. Effect ((Enum Effects types))
use crate::combatant_manager::Combatant;
use crate::error_types::{GameEngineError, Result};
use crate::game_data_retriever::{Config, IdRegistry};
use rand::Rng;
use rand::RngExt;
use rand::prelude::IndexedRandom;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Attack { src: String, dst: String, dmg: u32 },
    //Others shit goes here Heal, Hex, Poison
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Effect {
    Damage {
        src: String,
        dst: String,
        dmg: u32,
        dst_hp_before: f32,
        dst_hp_after: f32,
    },
    // Other effects goes here like Heal_received, Got_Poisoned
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BattleJournal {
    pub winner_side: usize,
    pub events: Vec<GameEvent>,
}

impl BattleJournal {
    pub fn new() -> Self {
        Self {
            winner_side: 0,
            events: Vec::new(),
        }
    }

    pub(crate) fn calculate_damage(
        config: &Config,
        attacker: &Combatant,
        victim: &Combatant,
    ) -> u32 {
        let attack_val = *attacker.parameters.get("attack").unwrap_or(&0) as f32;
        let defense_val = *victim.parameters.get("defense").unwrap_or(&0) as f32;

        let attack_bonus_points = attack_val - config.ATTACK_MIDDLE_VALUE_PARAMETER;
        let attack_bonus = config.ATTACK_PERCENTAGE_PER_POINT * attack_bonus_points;
        let incoming_damage = config.BASIC_ATTACK + (config.BASIC_ATTACK * attack_bonus) / 100.0;

        let defense_bonus_points = if defense_val > config.DEFENSE_MIDDLE_VALUE_PARAMETER {
            defense_val - config.DEFENSE_MIDDLE_VALUE_PARAMETER
        } else {
            0.0
        };
        let defense_bonus = config.DEFENSE_PERCENTAGE_PER_POINT * defense_bonus_points;
        let damage_taken = incoming_damage - (incoming_damage * defense_bonus) / 100.0;

        if damage_taken < 0.0 {
            0
        } else {
            damage_taken.trunc() as u32
        }
    }

    pub(crate) fn generate_based_on_data(sides: &BTreeMap<usize, Vec<Combatant>>) -> Self {
        // let mut rng = rand::rng();
        let mut rng = rand::rngs::ThreadRng::default();
        // let mut rng = ThreadRng::default();

        let mut game_events: Vec<GameEvent> = Vec::new();

        let mut our_fighters = sides.get(&0).cloned().unwrap_or_default();
        let mut their_fighters = sides.get(&1).cloned().unwrap_or_default();

        let config = crate::game_data_retriever::get_config().unwrap_or(Config {
            PARAMETER_MINIMUM: 0,
            PARAMETER_MAXIMUM: 9,
            BASIC_HEALTH: 100.0,
            BASIC_ATTACK: 10.0,
            ATTACK_PERCENTAGE_PER_POINT: 5.0,
            DEFENSE_PERCENTAGE_PER_POINT: 5.0,
            HEALTH_PERCENTAGE_PER_POINT: 5.0,
            ATTACK_MIDDLE_VALUE_PARAMETER: 4.0,
            DEFENSE_MIDDLE_VALUE_PARAMETER: 2.0,
            HEALTH_MIDDLE_VALUE_PARAMETER: 4.0,
        });

        let mut current_round = 1;
        let mut current_turn = 1;

        let mut turn_by = *[0, 1].choose(&mut rng).unwrap_or(&0);

        let mut our_not_taken_turn_yet: Vec<usize> = (0..our_fighters.len()).collect();
        let mut their_not_taken_turn_yet: Vec<usize> = (0..their_fighters.len()).collect();

        let mut winner_side = 0;

        loop {
            let our_alive_count = our_fighters
                .iter()
                .filter(|c| c.current_health_points > 0)
                .count();
            let their_alive_count = their_fighters
                .iter()
                .filter(|c| c.current_health_points > 0)
                .count();

            if our_alive_count == 0 {
                winner_side = 1;
                break;
            }
            if their_alive_count == 0 {
                winner_side = 0;
                break;
            }

            if our_not_taken_turn_yet.is_empty() && their_not_taken_turn_yet.is_empty() {
                current_round += 1;
                current_turn = 1;
                our_not_taken_turn_yet = (0..our_fighters.len())
                    .filter(|&i| our_fighters[i].current_health_points > 0)
                    .collect();
                their_not_taken_turn_yet = (0..their_fighters.len())
                    .filter(|&i| their_fighters[i].current_health_points > 0)
                    .collect();
            }

            our_not_taken_turn_yet.retain(|&i| our_fighters[i].current_health_points > 0);
            their_not_taken_turn_yet.retain(|&i| their_fighters[i].current_health_points > 0);

            let attacker_option = if turn_by == 0 {
                our_not_taken_turn_yet.choose(&mut rng).map(|&i| (i, 0))
            } else {
                their_not_taken_turn_yet.choose(&mut rng).map(|&i| (i, 1))
            };

            if let Some((idx, side)) = attacker_option {
                if side == 0 {
                    our_not_taken_turn_yet.retain(|&i| i != idx);
                } else {
                    their_not_taken_turn_yet.retain(|&i| i != idx);
                }

                let (attacker_id, attacker_name) = if side == 0 {
                    let a = &our_fighters[idx];
                    (
                        a.id.clone(),
                        format!(
                            "{} {}",
                            a.first_name.as_deref().unwrap_or(""),
                            a.last_name.as_deref().unwrap_or("")
                        )
                        .trim()
                        .to_string(),
                    )
                } else {
                    let a = &their_fighters[idx];
                    (
                        a.id.clone(),
                        format!(
                            "{} {}",
                            a.first_name.as_deref().unwrap_or(""),
                            a.last_name.as_deref().unwrap_or("")
                        )
                        .trim()
                        .to_string(),
                    )
                };

                let victims_indices: Vec<usize> = if side == 0 {
                    their_fighters
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| c.current_health_points > 0)
                        .map(|(i, _)| i)
                        .collect()
                } else {
                    our_fighters
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| c.current_health_points > 0)
                        .map(|(i, _)| i)
                        .collect()
                };

                if !victims_indices.is_empty() {
                    let mut targets_probs = Vec::new();
                    let mut total_prob: u32 = 0;
                    for &v_idx in &victims_indices {
                        let v = if side == 0 {
                            &their_fighters[v_idx]
                        } else {
                            &our_fighters[v_idx]
                        };
                        let range = *v.parameters.get("range").unwrap_or(&5) as u32;
                        let prob = 10u32.saturating_sub(range);
                        targets_probs.push((v_idx, prob));
                        total_prob += prob;
                    }

                    if total_prob > 0 {
                        let mut pick: u32 = rng.random_range(0..total_prob);
                        let mut chosen_v_idx = victims_indices[0];
                        for (v_idx, prob) in targets_probs {
                            if pick < prob {
                                chosen_v_idx = v_idx;
                                break;
                            }
                            pick = pick.saturating_sub(prob);
                        }

                        let (dmg, hp_before) = {
                            let attacker_ref = if side == 0 {
                                &our_fighters[idx]
                            } else {
                                &their_fighters[idx]
                            };
                            let victim_ref = if side == 0 {
                                &their_fighters[chosen_v_idx]
                            } else {
                                &our_fighters[chosen_v_idx]
                            };
                            let d = Self::calculate_damage(&config, attacker_ref, victim_ref);
                            (d, victim_ref.current_health_points as f32)
                        };

                        let (v_id, v_name) = if side == 0 {
                            let v = &their_fighters[chosen_v_idx];
                            (
                                v.id.clone(),
                                format!(
                                    "{} {}",
                                    v.first_name.as_deref().unwrap_or(""),
                                    v.last_name.as_deref().unwrap_or("")
                                )
                                .trim()
                                .to_string(),
                            )
                        } else {
                            let v = &our_fighters[chosen_v_idx];
                            (
                                v.id.clone(),
                                format!(
                                    "{} {}",
                                    v.first_name.as_deref().unwrap_or(""),
                                    v.last_name.as_deref().unwrap_or("")
                                )
                                .trim()
                                .to_string(),
                            )
                        };

                        if side == 0 {
                            let v = &mut their_fighters[chosen_v_idx];
                            v.current_health_points =
                                v.current_health_points.saturating_sub(dmg as usize);
                        } else {
                            let v = &mut our_fighters[chosen_v_idx];
                            v.current_health_points =
                                v.current_health_points.saturating_sub(dmg as usize);
                        }

                        let hp_after = if side == 0 {
                            their_fighters[chosen_v_idx].current_health_points as f32
                        } else {
                            our_fighters[chosen_v_idx].current_health_points as f32
                        };

                        let cmd = Command::Attack {
                            src: attacker_id.clone(),
                            dst: v_id.clone(),
                            dmg: dmg as u32,
                        };

                        let effect = Effect::Damage {
                            src: attacker_id,
                            dst: v_id,
                            dmg: dmg as u32,
                            dst_hp_before: hp_before,
                            dst_hp_after: hp_after,
                        };

                        game_events.push(GameEvent {
                            round: current_round,
                            turn: current_turn,
                            cmd,
                            effects: vec![effect],
                        });
                    }
                }

                current_turn += 1;
                turn_by = 1 - turn_by;
            } else {
                turn_by = 1 - turn_by;
            }

            let our_alive = our_fighters.iter().any(|c| c.current_health_points > 0);
            let their_alive = their_fighters.iter().any(|c| c.current_health_points > 0);

            if !our_alive {
                winner_side = 1;
                break;
            }
            if !their_alive {
                winner_side = 0;
                break;
            }
        }

        Self {
            winner_side,
            events: game_events,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEvent {
    pub round: u32,
    pub turn: u32,
    pub cmd: Command,
    pub effects: Vec<Effect>,
}
