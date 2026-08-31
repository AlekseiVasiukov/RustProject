use GrandAutoBattler_GameEngine;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers, read},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use std::collections::BTreeMap;
use std::io::{Write, stdout};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crate::Stage::PreparationFactions;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Stage {
    Title,
    PreparationFactions,
    PreparationUnitTypes,
    UnitsGeneration,
    BattleSimulation,
    BattleJournal,
    Results,
}

#[derive(Debug)]
pub struct GameState {
    pub current_stage: Stage,
    pub counter: i32,
    pub journal_index: usize,
    pub conflicting_sides: BTreeMap<usize, ConflictngSide>,
    pub(crate) cursor_on_faction: usize,
    pub(crate) cursor_on_unit_types: usize,
    pub(crate) available_factions_names: Vec<String>,
    pub(crate) available_unittypes_names: BTreeMap<usize, Vec<String>>,
}

#[derive(Debug)]
pub struct ConflictngSide {
    pub selected_faction: usize,
    selected_units: BTreeMap<usize, Option<usize>>,
}

impl ConflictngSide {
    pub fn new() -> Self {
        let mut units: BTreeMap<usize, Option<usize>> = BTreeMap::new();
        units.insert(0, Some(0));
        units.insert(1, None);
        units.insert(2, None);
        units.insert(3, None);
        Self {
            selected_faction: 0,
            selected_units: units,
        }
    }
}

impl Default for ConflictngSide {
    fn default() -> Self {
        ConflictngSide::new()
    }
}

impl GameState {
    pub fn new(
        default_factions_names: Vec<String>,
        default_unittypes_names: BTreeMap<usize, Vec<String>>,
    ) -> Self {
        let mut default_conflicting_sides: BTreeMap<usize, ConflictngSide> = BTreeMap::new();
        default_conflicting_sides.insert(0, ConflictngSide::default());
        default_conflicting_sides.insert(1, ConflictngSide::default());

        Self {
            current_stage: Stage::Title,
            counter: 0,
            journal_index: 0,
            conflicting_sides: default_conflicting_sides,
            cursor_on_faction: 0,
            cursor_on_unit_types: 0,
            available_factions_names: default_factions_names,
            available_unittypes_names: default_unittypes_names,
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
        };

        //resetting values
        if self.current_stage == PreparationFactions {
            self.cursor_on_faction = 0;
            self.cursor_on_unit_types = 0;

            let mut default_conflicting_sides: BTreeMap<usize, ConflictngSide> = BTreeMap::new();
            default_conflicting_sides.insert(0, ConflictngSide::default());
            default_conflicting_sides.insert(1, ConflictngSide::default());

            self.conflicting_sides = default_conflicting_sides;
        }
    }

    pub fn reset_to_preparation(&mut self) {
        self.current_stage = Stage::PreparationFactions;
    }

    pub fn render(&self) {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 40),).unwrap();

        match self.current_stage {
            Stage::Title => {
                raw_println_yellow("========================================");
                raw_println_yellow("       GRAND AUTO-BATTLER             ");
                raw_println_yellow("========================================");
                raw_println("");
                raw_println(" [ START (spacebar) ] [ EXIT (esc) ]");
            }
            Stage::PreparationFactions => {
                raw_println_yellow(" PREPARATION STAGE: CHOOSE YOUR FACTION");
                raw_println(" YOUR FACTION:");
                raw_println(format!(" {}", self.get_faction_display(&0)));
                raw_println(format!(""));
                raw_println(" OPPONENT'S FACTION:");
                raw_println(format!(" {}", self.get_faction_display(&1)));
                raw_println(format!(""));
                raw_println(format!(" Counter: {}", self.counter)); // Showing counter for debugging
                raw_println(format!(""));
                raw_println(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::PreparationUnitTypes => {
                raw_println_yellow(" PREPARATION STAGE: CHOOSE YOUR WARRIORS");
                raw_println(format!(" {}", self.get_unittype_display(&0)));
                raw_println(format!(" {}", self.get_unittype_display(&1)));
                raw_println(format!(" {}", self.get_unittype_display(&2)));
                raw_println(format!(" {}", self.get_unittype_display(&3)));
                raw_println(format!(" Counter: {}", self.counter)); // Showing counter for debugging
                raw_println(format!(""));
                raw_println(format!(
                    " [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]"
                ));
            }
            _ => {
                raw_println("Not implemented stage yet.");
                raw_println(" [ CONTINUE (spacebar) ] [ EXIT (esc)]");
            }
        }
    }

    fn get_faction_display(&self, side_index: &usize) -> String {
        let faction_name = self.available_factions_names
            [self.conflicting_sides[side_index].selected_faction]
            .clone();
        let marked = if self.cursor_on_faction == *side_index {
            "►"
        } else {
            ""
        };
        let result = format!(" {}\t{}", marked, faction_name);
        result
    }

    fn get_unittype_display(&self, unit_index: &usize) -> String {
        let side = &self.conflicting_sides[&0];

        let unit_name = match side.selected_units.get(&unit_index).and_then(|&v| v) {
            Some(index) => self.available_unittypes_names[&side.selected_faction][index].clone(),
            None => "None".to_string(),
        };

        let marked = if self.cursor_on_unit_types == *unit_index {
            "►"
        } else {
            ""
        };
        let result = format!(" {}\t[{}]\t{}", marked, unit_index + 1, unit_name);
        result
    }

    pub(crate) fn apply_action_to_game_stage(&mut self, action: Action) {
        match self.current_stage {
            Stage::PreparationFactions => match action {
                //Switching between ours and opponent's faction
                //Using modulo, to cicle the loop
                Action::MoveUp => {
                    let len = self.conflicting_sides.len();
                    if len <= 1 {
                        return;
                    }
                    // Adding len ensures the result is positive before modulo
                    self.cursor_on_faction = (self.cursor_on_faction + len - 1) % len;
                }
                Action::MoveDown => {
                    let len = self.conflicting_sides.len();
                    if len <= 1 {
                        return;
                    }
                    self.cursor_on_faction = (self.cursor_on_faction + 1) % len;
                }

                Action::MoveLeft => {
                    let len = self.available_factions_names.len();
                    if len <= 1 {
                        return;
                    }
                    // self.available_factions_names
                    if let Some(side) = self.conflicting_sides.get_mut(&self.cursor_on_faction) {
                        side.selected_faction = (side.selected_faction + len - 1) % len;
                    }
                }
                Action::MoveRight => {
                    let len = self.available_factions_names.len();
                    if len <= 1 {
                        return;
                    }
                    if let Some(side) = self.conflicting_sides.get_mut(&self.cursor_on_faction) {
                        side.selected_faction = (side.selected_faction + 1) % len;
                    }
                }

                _ => {}
            },
            Stage::PreparationUnitTypes => match action {
                Action::MoveUp => {
                    let side = &self.conflicting_sides[&0];

                    if side.selected_units.len() <= 1 {
                        return;
                    }
                    let len = side.selected_units.len();
                    self.cursor_on_unit_types = (self.cursor_on_unit_types + len - 1) % len;
                }
                Action::MoveDown => {
                    let side = &self.conflicting_sides[&0];

                    if side.selected_units.len() <= 1 {
                        return;
                    }
                    let len = side.selected_units.len();
                    self.cursor_on_unit_types = (self.cursor_on_unit_types + 1) % len;

                    raw_println(format!("{:#?}", side));
                }

                Action::MoveLeft => {
                    if let Some(side) = self.conflicting_sides.get_mut(&0) {
                        let faction_index = side.selected_faction;
                        let cursor_index = self.cursor_on_unit_types;

                        // Get current value to check for None constraint
                        let current_val = *side.selected_units.get(&cursor_index).unwrap();

                        if let Some(unit_types_list) =
                            self.available_unittypes_names.get(&faction_index)
                        {
                            if !unit_types_list.is_empty() {
                                let len = unit_types_list.len();

                                let total_slots = len + 1;

                                if current_val.is_none() {
                                    *side.selected_units.get_mut(&cursor_index).unwrap() =
                                        Some(len - 1);
                                } else if let Some(index) = current_val {
                                    if index == 0 {
                                        let others_all_none = side
                                            .selected_units
                                            .iter()
                                            .filter(|(k, _)| *k != &cursor_index) // exclude the slot we're about to change
                                            .all(|(_, v)| v.is_none());

                                        if others_all_none {
                                            return; // no mutation happened at all
                                        }

                                        *side.selected_units.get_mut(&cursor_index).unwrap() = None;
                                    } else {
                                        *side.selected_units.get_mut(&cursor_index).unwrap() =
                                            Some(index - 1);
                                    }
                                }
                            }
                        }
                    }
                }
                Action::MoveRight => {
                    if let Some(side) = self.conflicting_sides.get_mut(&0) {
                        let faction_index = side.selected_faction;
                        let cursor_index = self.cursor_on_unit_types;

                        let current_val = *side.selected_units.get(&cursor_index).unwrap();

                        if let Some(unit_types_list) =
                            self.available_unittypes_names.get(&faction_index)
                        {
                            if !unit_types_list.is_empty() {
                                let len = unit_types_list.len();

                                if current_val.is_none() {
                                    *side.selected_units.get_mut(&cursor_index).unwrap() = Some(0);
                                } else if let Some(index) = current_val {
                                    if index == len - 1 {
                                        let others_all_none = side
                                            .selected_units
                                            .iter()
                                            .filter(|(k, _)| *k != &cursor_index) // exclude the slot we're about to change
                                            .all(|(_, v)| v.is_none());

                                        if others_all_none {
                                            return; // no mutation happened at all
                                        }

                                        *side.selected_units.get_mut(&cursor_index).unwrap() = None;
                                    } else {
                                        *side.selected_units.get_mut(&cursor_index).unwrap() =
                                            Some(index + 1);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => return,
        }
        self.render();
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
    raw_println("Console App launched");
    raw_println("Trying to request new game instance from game engine");
    let mut games_manager = GrandAutoBattler_GameEngine::GameInstancesManager::new();
    let game_instance_id = games_manager.GetNewGameInstanceId();
    raw_println(format!(
        "Game engine generated new game with Id: {}",
        game_instance_id
    ));

    let game_instance = games_manager.GetGameDataById(&game_instance_id);

    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen).unwrap();
    std::io::stdout().execute(Hide).unwrap();

    let mut faction_names: Vec<String> = Vec::new();
    let mut unittypes_names: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for (i, faction) in game_instance.factions.iter().enumerate() {
        faction_names.push(faction.name.clone());

        let mut unittypes_names_of_faction: Vec<String> = Vec::new();
        for unit_type in faction.unit_types.clone() {
            if let Some(pretty_name) = unit_type.unit_type_pretty_name {
                unittypes_names_of_faction.push(pretty_name);
            } else {
                unittypes_names_of_faction.push(unit_type.unit_type_name);
            }
        }
        unittypes_names.insert(i, unittypes_names_of_faction);
    }

    let mut game = GameState::new(faction_names, unittypes_names);

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
                        Some(last) => now.duration_since(last) >= Duration::from_millis(150),
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
                            _ => game.apply_action_to_game_stage(action),
                            // _ => {}
                        }
                    }
                }
                // if game.current_stage == Stage::Pr
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

pub fn raw_print(s: impl AsRef<str>) -> io::Result<()> {
    let mut out = io::stdout().lock();
    write!(out, "{}", s.as_ref())?;
    out.flush()
}

pub fn raw_println(s: impl AsRef<str>) -> io::Result<()> {
    let mut out = io::stdout().lock();
    write!(out, "\r\n{}", s.as_ref())?;
    out.flush()
}

//Including colors     "\x1b[97mHello \x1b[37mworld\x1b[0m"
//\x1b[
//97m color choose (light gray)
//0m - resetting formatting.
pub fn raw_println_yellow(s: impl AsRef<str>) -> io::Result<()> {
    let mut out = io::stdout().lock();
    write!(out, "\r\n\x1b[33m{}\x1b[0m", s.as_ref())?;
    out.flush()
}
