enum Stage {
    Title,
    Preparation,
    UnitsGeneration,
    BattleSimulation,
    BattleJournal,
    Results,
}

struct GameState {
    counter: i32,
}

enum Action {
    Continue,
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

fn main() {
    println!("Hello, world!");
}
