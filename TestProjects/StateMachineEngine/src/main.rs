use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use std::{
    io,
    time::{Duration, Instant},
};

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

        std::io::stdout().execute(Clear(ClearType::All));

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
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    std::io::stdout().execute(Hide)?;

    let mut game = GameState::new();
    game.render();

    let mut last_input: Option<Instant> = None;

    loop {
        //Checkin user's input with a short timeout
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) => {
                if let Some(action) = get_action(&Event::Key(key)) {
                    let now = Instant::now();
                    //Debouncing to prevent several signals appear from pressed key
                    let is_debounced = match last_input {
                        None => true,
                        Some(last) => now.duration_since(last) >= Duration::from_millis(300),
                    };

                    if is_debounced {
                        last_input = Some(now);

                        match action {
                            Action::Continue => {
                                if game.current_stage == Stage::Results {
                                    game.reset_to_preparation();
                                } else {
                                    game.next_stage();
                                }

                                game.render();
                            }

                            Action::Exit => break,

                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }
    std::io::stdout().execute(Show)?;
    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn get_action(event: &Event) -> Option<Action> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => Some(Action::Continue),
            KeyCode::Esc => Some(Action::Exit),
            KeyCode::Up => Some(Action::MoveUp),
            KeyCode::Down => Some(Action::MoveDown),
            KeyCode::Left => Some(Action::MoveLeft),
            KeyCode::Right => Some(Action::MoveRight),
            _ => None,
        }
    } else {
        None
    }
}
