use GrandAutoBattler_GameEngine;
use std::io::{Write, stdout};

use std::error::Error;

pub fn RunConsoleApp() {
    println!("Console App launched");
    println!("Trying to request new game instance from game engine");
    let mut games_manager = GrandAutoBattler_GameEngine::GameInstancesManager::new();
    let game_instance_id = games_manager.GetNewGameInstanceId();
    println!(
        "Game engine generated new game with Id: {}",
        game_instance_id
    );
}
