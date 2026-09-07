//                            .Intention (Enum Intentions types)
//Battle Journal .-.. Turn -<
//                            .Outcome .-.. Effect ((Enum Effects types))

enum IntentionType {
    Attack,
}

enum EffectType {
    Damage_to_HP,
}

#[derive(Clone, Debug)]
pub struct BattleJournal {
    pub winner_side: usize,
    pub turns: Vec<Turn>,
}

impl BattleJournal {
    pub(crate) fn new(winner_side: &usize, &turns: Vec<Turn>) -> Self {
        Self {
            winner: winner.clone(),
            turns: turns.clone(),
        }
    }
}

#[derive(Clone, Debug)]

pub struct Turn {
    pub id: String,
    pub turn_number: usize,
    pub intention: Instention,
    pub outcome: Outcome,
}

impl Turn {
    pub(crate) fn new(
        id: &String,
        turn_number: &usize,
        intention: &Instention,
        outcome: &Outcome,
    ) -> Self {
        Self {
            id: id.clone(),
            turn_number: turn_number.clone(),
            intention: Instention.clone(),
            outcome: Outcome.clone(),
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
    pub(crate) fn new(&effects: Vec<Effect>) -> Self {
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
    pub(crate) fn new(entity_id: &String, effects: &effect_type) -> Self {
        Self {
            entity_id: entity_id.clone(),
            effects: effect_type.clone(),
        }
    }
}
