#[derive(Debug)]
pub struct GameInstance {
    id: i32,
}

impl GameInstance {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}
