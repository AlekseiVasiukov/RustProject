use GrandAutoBattler_GameEngine;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers, read},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::collections::BTreeMap;
use std::io::{Write, stdout};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Stage {
    Title,
    PreparationFactions,
    PreparationUnitTypes,
    UnitsGeneration,
    BattleSimulation,
    BattleJournal,
    Results,
}

pub struct GameState {
    pub current_stage: Stage,
    pub counter: i32,
    // pub selected_units: Vec<()
    pub journal_index: usize,
    pub conflicting_sides: Vec<ConflictngSide>,
}

pub struct ConflictngSide {
    pub selected_faction: usize,
    selected_units: BTreeMap<usize, Option<usize>>,

    

}

impl ConflictngSide {
    pub fn new () -> Self {
        Self {
            selected_faction: 0,
            selected_units:  BTreeMap::new()
        }
    }
}


impl GameState {
    pub fn new() -> Self {
        Self {
            current_stage: Stage::Title,
            counter: 0,
            // selected_units:
            journal_index: 0,
            conflicting_sides: Vec::new()
        }
    }

    pub fn next_stage(&mut self) {
        self.counter += 1;
        self.current_stage = match self.current_stage {
            Stage::Title => Stage::PreparationFactions,
            Stage::PreparationFactions => Stage::PreparationUnitTypes,
            Stage::PreparationUnitTypes => Stage::UnitsGeneration,
            Stage::UnitsGeneration => Stage::BattleSimulation,
            Stage::BattleSimulation => Stage::BattleJournal,
            Stage::BattleJournal => Stage::Results,
            Stage::Results => Stage::PreparationFactions,
        }
    }

    pub fn reset_to_preparation(&mut self) {
        self.current_stage = Stage::PreparationFactions;
    }

    pub fn render(&self) {
        // Single clear call to prevent cumulative scrolling/clipping

        let mut stdout = std::io::stdout();
        execute!(
            stdout, 
            Clear(ClearType::All), 
            MoveTo(0, 40),
        ).unwrap();

        
        match self.current_stage {
            Stage::Title => {
                println!("========================================");
                println!("       GRAND AUTO-BATTLER             ");
                println!(" ================================= ======");
                println!();
                println!(" [ START (spacebar) ] [ EXIT (esc) ]");
            }
            Stage::PreparationFactions => {
                println!(" PREPARATION STAGE: CHOOSE YOUR FACTION");
                println!(" YOUR FACTION:");
                
                println!(" OPPONENT'S FACTION:");
                
                println!(" Counter: {}", self.counter); // Showing counter for debugging
                println!();
                println!(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::PreparationUnitTypes => {
                println!(" PREPARATION STAGE: CHOOSE YOUR WARRIORS");
                println!(" Units: Knight, Archer, Mage, Paladin");
                println!(" Counter: {}", self.counter); // Showing counter for debugging
                println!();
                println!(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
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

pub fn RunConsoleApp() -> io::Result<()> {
    println!("Console App launched");
    println!("Trying to request new game instance from game engine");
    let mut games_manager = GrandAutoBattler_GameEngine::GameInstancesManager::new();
    let game_instance_id = games_manager.GetNewGameInstanceId();
    println!(
        "Game engine generated new game with Id: {}",
        game_instance_id
    );

    let game_instance = games_manager.GetGameDataById(&game_instance_id);
    println!("{:#?}", &game_instance.factions);

    let faction_index = 0;
    let cursor_on_faction: usize = 0;
    let cursor_on_unit_types: usize = 0;
    // let unitTypes: BTreeMap<usize, i32> = BTreeMap::new();

    //StateMachine loop

    // enable_raw_mode().unwrap();
    std::io::stdout().execute(EnterAlternateScreen).unwrap();
    std::io::stdout().execute(Hide).unwrap();

    let mut game = GameState::new();

    game.render();

    //User Input
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
                // if game.current_stage == Stage::Pr
            }
            _ => {}
        }
    }

    // loop {

    // println!("{:#?}", current_unit_types);
    // }
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
