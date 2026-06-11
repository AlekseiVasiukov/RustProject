#[derive(Debug)]
pub struct GameInstance {
    id: i32,
}

impl GameInstance {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        println!("Initializing unit types");
        self.initialize_unit_types();
        Ok(())
    }

    fn initialize_unit_types(&mut self) -> Result<(), String> {
        self.id = 666;
        Ok(())
    }
}
