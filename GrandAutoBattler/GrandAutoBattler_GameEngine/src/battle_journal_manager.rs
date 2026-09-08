//                            .Intention (Enum Intentions types)
//Battle Journal .-.. Turn -<
//                            .Outcome .-.. Effect ((Enum Effects types))
use crate::combatant_manager::Combatant;
use rand::seq::SliceRandom;
use std::collections::{BTreeMap, btree_map::Keys};

#[derive(Clone, Debug)]
enum IntentionType {
    Attack,
}

#[derive(Clone, Debug)]
enum EffectType {
    Damage_to_HP,
}

#[derive(Clone, Debug)]
pub struct BattleJournal {
    pub winner_side: usize,
    pub turns: Vec<Turn>,
}

impl BattleJournal {
    pub(crate) fn new(winner_side: &usize, turns: Vec<Turn>) -> Self {
        Self {
            winner_side: winner_side.clone(),
            turns: turns.clone(),
        }
    }

    pub(crate) fn generate_based_on_data(sides: &BTreeMap<usize, Vec<Combatant>>) -> Self {
        let mut rng = rand::thread_rng();
        let turns: Vec<Turn> = Vec::new();
        let mut our_fighters = sides[&0].clone();
        let mut their_fighters = sides[&1].clone();
        let mut turn_by = *[0, 1].choose(&mut rng).unwrap();

        let our_fighters_alive: Vec<_> = our_fighters.iter().filter(|c| c.current_health_points > 0).collect();
        let their_fighters_alive: Vec<_> = their_fighters.iter().filter(|c| c.current_health_points > 0).collect();

        let mut winner_side: usize = 0;
        loop {
            //checking win conditions
            //calculating party health
            // let turn_side = sides[tu]

            //giving turn to another side
            if (our_fighters_alive.len() == 0){
                winner_side = 1;
            }

            if (their_fighters_alive.len() == 0){
                winner_side = 0;
            }

            if turn_by == 1 {
                turn_by = 0
            } else {
                turn_by = 1
            };
        }

        Self {
            winner_side: winner_side,
            turns: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Turn {
    pub id: String,
    pub turn_number: usize,
    pub intention: Intention,
    pub outcome: Outcome,
}

impl Turn {
    pub(crate) fn new(
        id: &String,
        turn_number: &usize,
        intention: &Intention,
        outcome: &Outcome,
    ) -> Self {
        Self {
            id: id.clone(),
            turn_number: turn_number.clone(),
            intention: intention.clone(),
            outcome: outcome.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Intention {
    pub entity_id: String,
    pub intention: IntentionType,
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub effects: Vec<Effect>,
}

impl Outcome {
    pub(crate) fn new(effects: &Vec<Effect>) -> Self {
        Self {
            effects: effects.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Effect {
    pub entity_id: String,
    pub effect_type: EffectType,
}
impl Effect {
    pub(crate) fn new(entity_id: &String, effect_type: &EffectType) -> Self {
        Self {
            entity_id: entity_id.clone(),
            effect_type: effect_type.clone(),
        }
    }
}
