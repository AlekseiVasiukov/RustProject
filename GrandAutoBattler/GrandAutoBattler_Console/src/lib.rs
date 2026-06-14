use GrandAutoBattler_GameEngine;
use std::io::{Write, stdout};

use std::error::Error;

pub fn RunConsoleApp() {
    println!("Console App launched");
    println!("Trying to request new game instance from game engine");

    GrandAutoBattler_GameEngine::GetNewGameInstance();
}
