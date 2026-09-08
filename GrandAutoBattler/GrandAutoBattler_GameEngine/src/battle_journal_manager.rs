//                            .Intention (Enum Intentions types)
//Battle Journal .-.. Turn -<
//                            .Outcome .-.. Effect ((Enum Effects types))
use crate::combatant_manager::Combatant;
use std::collections::{BTreeMap, btree_map::Keys};
use rand::seq::SliceRandom;


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

    pub(crate) fn generate_based_on_data(sides: &BTreeMap<usize, Vec<Combatant>>) -> Self{
        let mut rng = rand::thread_rng();
        let turns: Vec<Turn> = Vec::new();
        let mut opposite_side: usize;
        let mut turn_by = *[0,1].choose (&mut rng).unwrap();
        loop{
            //setting opposing side
            if turn_by == 1 {opposite_side = 0} else {opposite_side = 1};

            //calculating health 

            //switching side
            if turn_by == 1 {turn_by = 0} else {turn_by = 1};
        }

        Self{
            winner_side: 0,
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
