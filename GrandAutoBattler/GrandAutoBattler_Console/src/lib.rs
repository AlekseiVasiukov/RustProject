use GrandAutoBattler_GameEngine::combatant_manager::Combatant;
use GrandAutoBattler_GameEngine::{
    battle_journal_manager::{Command, GameEvent},
    game_instance_manager::GameInstancesManager,
    game_result_manager::GameResult,
};

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
    pub(crate) game_result: Option<GameResult>,
}

#[derive(Debug)]
pub struct ConflictngSide {
    pub selected_faction: usize,
    pub selected_units: BTreeMap<usize, Option<usize>>,
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
            game_result: None,
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
        if self.current_stage == Stage::BattleJournal {
            self.journal_index = 0;
            if let Some(result) = self.game_result.as_mut() {
                for combatants in result.generated_combatants.values_mut() {
                    for combatant in combatants {
                        combatant.current_health_points = combatant.health_points;
                    }
                }
            }
            let has_events = self
                .game_result
                .as_ref()
                .map(|result| !result.battle_journal.events.is_empty())
                .unwrap_or(false);
            if has_events {
                self.apply_journal_event(0);
            }
        }
        if self.current_stage == PreparationFactions {
            self.cursor_on_faction = 0;
            self.cursor_on_unit_types = 0;
            let mut default_conflicting_sides: BTreeMap<usize, ConflictngSide> = BTreeMap::new();
            default_conflicting_sides.insert(0, ConflictngSide::default());
            default_conflicting_sides.insert(1, ConflictngSide::default());
            self.conflicting_sides = default_conflicting_sides;
        }
    }

    pub fn reset_stage_to_preparation(&mut self) {
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
                raw_println("");
                raw_println(" OPPONENT'S FACTION:");
                raw_println(format!(" {}", self.get_faction_display(&1)));
                raw_println("");
                raw_println(format!(" Counter: {}", self.counter)); // Showing counter for debugging
                raw_println("");
                raw_println(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::PreparationUnitTypes => {
                raw_println_yellow(" PREPARATION STAGE: CHOOSE YOUR WARRIORS");
                raw_println(format!(" {}", self.get_unittype_display(&0)));
                raw_println(format!(" {}", self.get_unittype_display(&1)));
                raw_println(format!(" {}", self.get_unittype_display(&2)));
                raw_println(format!(" {}", self.get_unittype_display(&3)));
                raw_println(format!(" Counter: {}", self.counter)); // Showing counter for debugging
                raw_println("");
                raw_println(" [ ↕ ] [ ◄ ] [ ► ] [ Continue (spacebar) ] [ Exit (esc) ]");
            }
            Stage::UnitsGeneration => {
                raw_println_yellow(" UNIT GENERATION STAGE: SEE YOUR WARRIORS BEFORE BATTLE");
                raw_println("");
                raw_println("info: (H[] A[] R[] D[] is Health, Attack, Range and Defense)");
                raw_println("");
                raw_println_green(" |——————————————————————————————————");
                raw_println_green(format!(" |Your Faction:\"{}\"", self.get_faction_name(&0)));
                raw_println_green(" |——————————————————————————————————");
                raw_println_green(format!("{}", self.get_generated_warriors_display(&0)));
                raw_println_magenta(" |——————————————————————————————————");
                raw_println_magenta(format!(
                    " |Opponent's faction: \"{}\"",
                    self.get_faction_name(&1)
                ));
                raw_println_magenta(" |——————————————————————————————————");
                raw_println_magenta(format!("{}", self.get_generated_warriors_display(&1)));

                raw_println(format!(" Counter: {}", self.counter)); // Showing counter for debugging
                raw_println(" [ Continue (spacebar) ] [ EXIT (esc) ]");
            }
            Stage::BattleJournal => {
                let result = self.game_result.as_ref().unwrap();
                let journal = &result.battle_journal;
                let events = &journal.events;

                if events.is_empty() {
                    raw_println_yellow(" No events recorded in the battle journal.");
                    return;
                }

                let current_event = &events[self.journal_index];
                let (attacker_id, victim_id) = match &current_event.cmd {
                    Command::Attack { src, dst, .. } => (src, dst),
                };

                // Header
                raw_println_yellow(format!(
                    " BATTLE JOURNAL: ROUND {} | TURN {}",
                    current_event.round, current_event.turn
                ));
                raw_println(" ------------------------------------------------------------");

                // Combatants Display (Two columns)
                let side0_combatants = &result.generated_combatants[&0];
                let side1_combatants = &result.generated_combatants[&1];
                let max_combatants = side0_combatants.len().max(side1_combatants.len());

                for i in 0..max_combatants {
                    let mut line = String::new();

                    // Side 0 (Left)
                    if let Some(c) = side0_combatants.get(i) {
                        line.push_str(&self.format_combatant_line(c, attacker_id, victim_id));
                    } else {
                        line.push_str(&" ".repeat(30));
                    }

                    line.push_str(" \t \t \t"); // Gap

                    // Side 1 (Right)
                    if let Some(c) = side1_combatants.get(i) {
                        line.push_str(&self.format_combatant_line(c, attacker_id, victim_id));
                    } else {
                        line.push_str(&" ".repeat(30));
                    }

                    raw_println(&line);
                }

                raw_println(" ------------------------------------------------------------");

                // Battle Log
                match &current_event.cmd {
                    Command::Attack { src, dst, dmg } => {
                        let attacker_name = self.find_combatant_name(src, result);
                        let victim_name = self.find_combatant_name(dst, result);
                        raw_println(format!(
                            " [LOG] {} attacks {} dealing {} damage",
                            attacker_name, victim_name, dmg
                        ));
                    }
                    _ => {
                        raw_println(" [LOG] An unknown event occurred.");
                    }
                }

                raw_println("");
                raw_println(" [ ◄ ] [ ► ] [ Continue (spacebar) ] [ EXIT (esc) ]");
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

    fn get_generated_warriors_display(&self, side_index: &usize) -> String {
        let game_result = self.game_result.as_ref().unwrap();
        let combatants = &game_result.generated_combatants[&side_index];

        let mut line_to_print = "".to_string();

        for (i, combatant) in combatants.iter().enumerate() {
            line_to_print.push_str(&format!("  "));

            //NUMBER
            line_to_print.push_str(&format!("№{} ", i));

            //PARAMETERS
            let parameters_line = format!(
                "H[{}] A[{}] R[{}] D[{}]",
                combatant.parameters["health"],
                combatant.parameters["attack"],
                combatant.parameters["range"],
                combatant.parameters["defense"]
            );
            line_to_print.push_str(&format!("parameters: {} \t", parameters_line));

            //HP
            let hp_line = format!("{}", combatant.health_points);
            line_to_print.push_str(&format!("{}HP \t", hp_line));

            //Unittype
            let mut unittype_name = &combatant.unit_type_name;
            if let Some(name) = &combatant.unit_type_pretty_name {
                unittype_name = name;
            }
            line_to_print.push_str(&format!("\"{}\" \t \t", unittype_name));

            //NAME
            if let Some(first_name) = &combatant.first_name {
                line_to_print.push_str(&format!("{} ", first_name));
            }

            if let Some(last_name) = &combatant.last_name {
                line_to_print.push_str(&format!("{}", last_name));
            }

            line_to_print.push_str(&format!("\r\n"));
        }

        let result = line_to_print;
        return result;
    }

    fn get_faction_name(&self, side_index: &usize) -> String {
        let faction_name = self.available_factions_names
            [self.conflicting_sides[side_index].selected_faction]
            .clone();
        let result = format!("{}", faction_name);
        result
    }

    fn find_combatant_name(&self, id: &str, result: &GameResult) -> String {
        for combatants in result.generated_combatants.values() {
            for c in combatants {
                if c.id == id {
                    return format!(
                        "{} {}",
                        c.first_name.as_deref().unwrap_or(""),
                        c.last_name.as_deref().unwrap_or("")
                    )
                    .trim()
                    .to_string();
                }
            }
        }
        id.to_string()
    }

    fn format_combatant_line(
        &self,
        c: &Combatant,
        attacker_id: &String,
        victim_id: &String,
    ) -> String {
        let color = if c.current_health_points <= (0 as usize) {
            "\x1b[90m"
        } else if c.id == *attacker_id {
            "\x1b[93m"
        } else if c.id == *victim_id {
            "\x1b[38;5;208m"
        } else {
            "\x1b[92m"
        };
        let reset = "\x1b[0m";

        let name = format!(
            "{} {}",
            c.first_name.as_deref().unwrap_or(""),
            c.last_name.as_deref().unwrap_or("")
        )
        .trim()
        .to_string();
        let unittype = c
            .unit_type_pretty_name
            .as_deref()
            .unwrap_or(&c.unit_type_name);

        format!(
            "{}{: <15} H[{}] A[{}] R[{}] D[{}] {}HP \"{}\"{}",
            color,
            name,
            c.parameters["health"],
            c.parameters["attack"],
            c.parameters["range"],
            c.parameters["defense"],
            c.current_health_points,
            unittype,
            reset
        )
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
            Stage::BattleJournal => match action {
                Action::MoveLeft => {
                    if self.journal_index > 0 {
                        self.undo_journal_event(self.journal_index);
                        self.journal_index -= 1;
                    }
                }
                Action::MoveRight => {
                    let event_count = self
                        .game_result
                        .as_ref()
                        .unwrap()
                        .battle_journal
                        .events
                        .len();
                    if self.journal_index + 1 < event_count {
                        self.journal_index += 1;
                        self.apply_journal_event(self.journal_index);
                    }
                }
                Action::Continue => {
                    self.next_stage();
                }
                _ => {}
            },
            _ => return,
        }
        self.render();
    }
    fn apply_journal_event(&mut self, event_index: usize) {
        let (victim_id, damage) = {
            let result = self.game_result.as_ref().unwrap();
            let event = &result.battle_journal.events[event_index];
            match &event.cmd {
                Command::Attack { dst, dmg, .. } => (dst.clone(), *dmg as usize),
                _ => return,
            }
        };
        let result = self.game_result.as_mut().unwrap();
        for combatants in result.generated_combatants.values_mut() {
            if let Some(victim) = combatants.iter_mut().find(|c| c.id == victim_id) {
                victim.current_health_points = victim.current_health_points.saturating_sub(damage);
                break;
            }
        }
    }
    fn undo_journal_event(&mut self, event_index: usize) {
        let (victim_id, damage) = {
            let result = self.game_result.as_ref().unwrap();
            let event = &result.battle_journal.events[event_index];
            match &event.cmd {
                Command::Attack { dst, dmg, .. } => (dst.clone(), *dmg as usize),
                _ => return,
            }
        };
        let result = self.game_result.as_mut().unwrap();
        for combatants in result.generated_combatants.values_mut() {
            if let Some(victim) = combatants.iter_mut().find(|c| c.id == victim_id) {
                victim.current_health_points =
                    (victim.current_health_points + damage).min(victim.health_points);
                break;
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
    raw_println("Console App launched");
    raw_println("Trying to request new game instance from game engine");
    let mut games_manager = GameInstancesManager::new();
    // let mut games_manager = GrandAutoBattler_GameEngine::GameInstancesManager::new();
    let game_instance_id = &games_manager.GetNewGameInstanceId();
    raw_println(format!(
        "Game engine generated new game with Id: {}",
        &game_instance_id
    ));

    let game_instance = games_manager.GetGameDataById(&game_instance_id);

    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen).unwrap();
    std::io::stdout().execute(Hide).unwrap();

    //collectin available factions and unittypes from engine
    let mut faction_names: Vec<String> = games_manager.GetAvailableFactions(&game_instance_id);
    let mut unittypes_names: BTreeMap<usize, Vec<String>> =
        games_manager.GetAvailableUnitTypes(&game_instance_id);

    // clear any buffered input events
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }

    let mut game = GameState::new(faction_names, unittypes_names);

    game.render();

    //User Input
    let mut last_input: Option<Instant> = None;

    loop {
        //Checkin user's input with a short timeout
        if !event::poll(Duration::from_millis(20))? {
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
                                    game.reset_stage_to_preparation();
                                } else if game.current_stage == Stage::PreparationUnitTypes {
                                    let mut selected_factions: BTreeMap<usize, usize> =
                                        BTreeMap::new();

                                    let selected_factions = game
                                        .conflicting_sides
                                        .iter()
                                        .map(|(&index, side)| (index, side.selected_faction))
                                        .collect();

                                    let game_result = games_manager.GetGameResultByIdAndSelection(
                                        &game_instance_id,
                                        &selected_factions,
                                        &game.conflicting_sides[&0].selected_units,
                                    );
                                    game.game_result = Some(game_result);
                                    game.next_stage();
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

pub fn raw_print(s: impl AsRef<str>) {
    let mut out = io::stdout().lock();
    write!(out, "{}", s.as_ref()).unwrap();
    out.flush().unwrap()
}

pub fn raw_println(s: impl AsRef<str>) {
    let mut out = io::stdout().lock();
    write!(out, "\r\n{}", s.as_ref()).unwrap();
    out.flush().unwrap()
}

//Including colors     "\x1b[97mHello \x1b[37mworld\x1b[0m"
//\x1b[
//97m color choose (light gray)
//0m - resetting formatting.
pub fn raw_println_yellow(s: impl AsRef<str>) {
    let mut out = io::stdout().lock();
    write!(out, "\r\n\x1b[33m{}\x1b[0m", s.as_ref()).unwrap();
    out.flush().unwrap()
}
pub fn raw_println_green(s: impl AsRef<str>) {
    let mut out = io::stdout().lock();
    write!(out, "\r\n\x1b[92m{}\x1b[0m", s.as_ref()).unwrap();
    out.flush().unwrap()
}

pub fn raw_println_magenta(s: impl AsRef<str>) {
    let mut out = io::stdout().lock();
    write!(out, "\r\n\x1b[95m{}\x1b[0m", s.as_ref()).unwrap();
    out.flush().unwrap()
}

// Colors
// 30 black
// 31 red
// 32 green
// 33 yellow
// 34 blue
// 35 magenta
// 36 cyan
// 37 white/light gray

// 90 bright black/dark gray
// 91 bright red
// 92 bright green
// 93 bright yellow
// 94 bright blue
// 95 bright magenta
// 96 bright cyan
// 97 bright white
