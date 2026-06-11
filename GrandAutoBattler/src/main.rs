mod game_engine;

use crate::game_engine::PrintGame;
use std::error::Error;
use std::io::{Write, stdout};

fn main() {
    PrintGame();
    
    println!("Hello, world!");
}
