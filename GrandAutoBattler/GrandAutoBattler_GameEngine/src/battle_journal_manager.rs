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

        let game_events: Vec<GameEvent> = Vec::new();
        let mut our_fighters = sides[&0].clone();
        let mut their_fighters = sides[&1].clone();

        let mut turn_by = *[0, 1].choose(&mut rng).unwrap();

        let our_fighters_alive: Vec<&Combatant> = our_fighters
            .iter()
            .filter(|c| c.current_health_points > 0)
            .collect();
        let their_fighters_alive: Vec<&Combatant> = their_fighters
            .iter()
            .filter(|c| c.current_health_points > 0)
            .collect();
        let our_not_taken_turn_yet: Vec<usize> = Vec::new();
        let their_not_taken_turn_yet: Vec<usize> = Vec::new();

        let mut winner_side: usize = 0;
        loop {
            //checking win conditions
            //calculating party health
            // let turn_side = sides[tu]

            //giving turn to another side
            if (our_fighters_alive.len() == 0) {
                winner_side = 1;
                break;
            }

            if (their_fighters_alive.len() == 0) {
                winner_side = 0;
                break;
            }

            let attacker_list = if (turn_by == 0) {
                &our_fighters_alive
            } else {
                &their_fighters_alive
            };
            let victims_list = if (!turn_by == 0) {
                &our_fighters_alive
            } else {
                &their_fighters_alive
            };
            let mut random_attacker = attacker_list.choose(&mut rng).unwrap();
            // let mut random_victim = victims_list.choose(&mut rng).unwrap();

            //Attack action
            // let mut random_victim =

            if turn_by == 1 {
                turn_by = 0
            } else {
                turn_by = 1
            };
        }

        Self {
            winner_side: winner_side,
            events: Vec::new(),
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
