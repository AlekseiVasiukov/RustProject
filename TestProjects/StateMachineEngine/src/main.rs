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

    pub fn reset_to_preparation(&mut self) {
        self.current_stage = Stage::Preparation;
    }

    pub fn render(&self) {
        //Clean the screen
        println!("{}", "\n".to_string().repeat(20));

        match self.current_stage {
            Stage::Title => {
                println!("========================================");
                println!("       GRAND AUTO-BATTLER             ");
                println!(" ================================= ======");
                println!();
                println!(" [ START (spacebar) ] [ EXIT (esc) ]");
            }
            Stage::Preparation => {
                println!(" PREPARATION STAGE");
                println!(" Units: Knight, Archer, Mage, Paladin");
                println!(" Counter: {}", self.counter); // Showing counter for debugging
                println!();
                println!(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::UnitsGeneration => {
                println!(" UNITS GENERATION");
                println!(" Units: Knight, Archer, Mage, Paladin");
                println!();
                println!(" [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::BattleSimulation => {
                println!(" BATTLE SIMULATION");
                println!(" Simulation 34%");
                println!();
                println!(" [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::BattleJournal => {
                println!(" BATTLE JOURNAL");
                println!(" Mage Attacked Demon dealing 10 damage");
                println!();
                println!(" [ ◄ ] [ ► ] [ To Result (spacebar) ] [ Exit (esc) ]");
            }
            Stage::Results => {
                println!(" RESULTS");
                println!(" You Won!");
                println!();
                println!(" [ Play again (spacebar) ] [ Exit (esc) ]");
            }
            _ => {
                println!("Not implemented stage yet.");
                println!(" [ CONTINUE (spacebar) ] [ EXIT (esc)]");
            }
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
