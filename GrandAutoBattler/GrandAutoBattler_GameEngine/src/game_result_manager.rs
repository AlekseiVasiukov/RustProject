use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::battle_journal_manager::BattleJournal;
use crate::combatant_manager::Combatant;
use std::collections::BTreeMap;

use crate::error_types::Result;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GameResult {
    pub generated_combatants: BTreeMap<usize, Vec<Combatant>>,
    pub battle_journal: BattleJournal,
}

impl GameResult {
    pub fn new(
        generated_combatants: BTreeMap<usize, Vec<Combatant>>,
        battle_journal: BattleJournal,
    ) -> Result<Self> {
        Ok(Self {
            generated_combatants,
            battle_journal,
        })
    }
}
