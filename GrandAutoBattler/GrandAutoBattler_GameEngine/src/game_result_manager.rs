use crate::combatant_manager::Combatant;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GameResult {
    pub generated_combatants: BTreeMap<usize, BTreeMap<usize, Combatant>>,
}

impl GameResult {
    pub fn new(
        generated_combatants: BTreeMap<usize, BTreeMap<usize, Combatant>>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            generated_combatants: generated_combatants,
        })
    }
}
