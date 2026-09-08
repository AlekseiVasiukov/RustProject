//                            .Intention (Enum Intentions types)
//Battle Journal .-.. Turn -<
//                            .Outcome .-.. Effect ((Enum Effects types))
use crate::combatant_manager::Combatant;
use crate::game_data_retriever::IdRegistry;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, btree_map::Keys};

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
    events: Vec<GameEvent>,
}

impl BattleJournal {
    pub(crate) fn new() -> Self {
        Self {
            winner_side: 0,
            events: Vec::new(),
        }
    }

    pub(crate) fn generate_based_on_data(sides: &BTreeMap<usize, Vec<Combatant>>) -> Self {
        let mut rng = rand::thread_rng();
        let mut registry = IdRegistry::new();

        let mut game_events: Vec<GameEvent> = Vec::new();
        let mut our_fighters = sides[&0].clone();
        let mut their_fighters = sides[&1].clone();

        // Initialize round and turn counters
        let mut current_round: u32 = 1;
        let mut current_turn: u32 = 1;

        // Determine who starts first
        let mut turn_by = *[0, 1].choose(&mut rng).unwrap();

        // Initialize not-taken-turn collections for both sides
        let mut our_not_taken_turn_yet: Vec<usize> = Vec::new();
        let mut their_not_taken_turn_yet: Vec<usize> = Vec::new();

        // Initialize collections for tracking living combatants
        let mut our_fighters_alive: Vec<&Combatant> = our_fighters
            .iter()
            .filter(|c| c.current_health_points > 0)
            .collect();
        let mut their_fighters_alive: Vec<&Combatant> = their_fighters
            .iter()
            .filter(|c| c.current_health_points > 0)
            .collect();

        // Fill not-taken-turn collections at the beginning of round 1
        for (index, fighter) in our_fighters_alive.iter().enumerate() {
            our_not_taken_turn_yet.push(index);
        }
        for (index, fighter) in their_fighters_alive.iter().enumerate() {
            their_not_taken_turn_yet.push(index);
        }

        let mut winner_side: usize = 0;

        loop {
            // Check if this is the beginning of a new round
            if our_not_taken_turn_yet.is_empty() && their_not_taken_turn_yet.is_empty() {
                // Start a new round
                current_round += 1;
                current_turn = 1;
                
                // Refill not-taken-turn collections with surviving combatants
                our_not_taken_turn_yet.clear();
                their_not_taken_turn_yet.clear();
                
                // Update living combatants lists
                our_fighters_alive = our_fighters
                    .iter()
                    .filter(|c| c.current_health_points > 0)
                    .collect();
                their_fighters_alive = their_fighters
                    .iter()
                    .filter(|c| c.current_health_points > 0)
                    .collect();
                
                // Fill not-taken-turn collections for surviving combatants
                for (index, _fighter) in our_fighters_alive.iter().enumerate() {
                    our_not_taken_turn_yet.push(index);
                }
                for (index, _fighter) in their_fighters_alive.iter().enumerate() {
                    their_not_taken_turn_yet.push(index);
                }
            }

            // Win condition check - if all combatants on one side are dead
            if our_fighters_alive.is_empty() {
                winner_side = 1;
                break;
            }

            if their_fighters_alive.is_empty() {
                winner_side = 0;
                break;
            }

            // Basic turn logic - this will be enhanced later with actual combat
            let attacker_list = if turn_by == 0 {
                &our_fighters_alive
            } else {
                &their_fighters_alive
            };
            let victims_list = if turn_by == 0 {
                &their_fighters_alive
            } else {
                &our_fighters_alive
            };

            // Remove the current unit from not-taken-turn collection
            if !attacker_list.is_empty() {
                // This is where we would implement the actual combat logic
                // For now, we'll just track the turn and create a placeholder event
                
                // Create a placeholder game event
                let game_event = GameEvent {
                    round: current_round,
                    turn: current_turn,
                    cmd: Command::Attack {
                        src: String::from("placeholder_src"),
                        dst: String::from("placeholder_dst"),
                        dmg: 0,
                    },
                    effects: Vec::new(),
                };
                
                game_events.push(game_event);
                current_turn += 1;
            }

            // Remove dead combatants from not-taken-turn collections
            our_not_taken_turn_yet.retain(|&index| {
                our_fighters_alive.get(index).map_or(false, |f| f.current_health_points > 0)
            });
            their_not_taken_turn_yet.retain(|&index| {
                their_fighters_alive.get(index).map_or(false, |f| f.current_health_points > 0)
            });

            // Switch turns
            if turn_by == 1 {
                turn_by = 0;
            } else {
                turn_by = 1;
            };
        }

        // Handle the end of round cleanup
        if our_not_taken_turn_yet.is_empty() && their_not_taken_turn_yet.is_empty() {
            // This ensures we properly end the round
        }

        Self {
            winner_side: winner_side,
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
