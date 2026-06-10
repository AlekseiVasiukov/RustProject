use crossterm::event::{self, Event, KeyCode};
use std::io;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Stage {
    Title,
    Preparation,
    UnitsGeneration,
    BattleSimulation,
    BattleJournal,
    Results,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UnitType {
    Knight,
    Archer,
    Mage,
    Paladin,
}

pub struct GameState {
    pub current_stage: Stage,
    pub counter: i32,
    pub selected_units: Vec<(UnitType, u32)>,
    pub journal_index: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_stage: Stage::Title,
            counter: 0,
            selected_units: Vec::new(),
            journal_index: 0,
        }
    }

    pub fn next_stage(&mut self) {
        self.counter += 1;
        self.current_stage = match self.current_stage {
            Stage::Title => Stage::Preparation,
            Stage::Preparation => Stage::UnitsGeneration,
            Stage::UnitsGeneration => Stage::BattleSimulation,
            Stage::BattleSimulation => Stage::BattleJournal,
            Stage::BattleJournal => Stage::Results,
            Stage::Results => Stage::Preparation,
        }
    }
}

enum Action {
    Continue,
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

fn main() -> io::Result<()> {
    Ok(())
}
