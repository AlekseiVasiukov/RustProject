use serde::{Deserialize, Serialize};

use crate::battle_journal_manager::BattleJournal;
use crate::combatant_manager::Combatant;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub generated_combatants: BTreeMap<usize, Vec<Combatant>>,
    pub battle_journal: BattleJournal,
}

impl GameResult {
    pub fn new(
        generated_combatants: BTreeMap<usize, Vec<Combatant>>,
        battle_journal: BattleJournal,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            generated_combatants: generated_combatants,
            battle_journal: battle_journal,
        })
    }
}
