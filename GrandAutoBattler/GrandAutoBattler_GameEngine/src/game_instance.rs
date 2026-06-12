use crate::combatants::*;

#[derive(Debug)]
pub struct GameInstance {
    id: i32,
    factions: Vec<Faction>,
}

impl GameInstance {
    pub fn new() -> Self {
        Self {
            id: 0,
            factions: Vec::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        println!("Initializing unit types");
        self.initialize_game_config();
        self.initialize_unit_types();

        Ok(())
    }

    fn initialize_game_config(&mut self) -> Result<(), String> {
        self.id = 666;
        let mut config = load_config();
        Ok(())
    }

    fn initialize_unit_types(&mut self) -> Result<(), String> {
        let factions = load_available_factions();

        match factions {
            Err(e) => {
                println!("Couldn't initialize unit types: {}", e);
                Err(e)
            }
            Ok(result) => {
                self.factions = result;
                Ok(())
            }
        }
    }
}
