use GrandAutoBattler_GameEngine;
use std::collections::BTreeMap;
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

    let game_instance = games_manager.GetGameDataById(&game_instance_id);
    println!("{:#?}", &game_instance.factions);

    let faction_index = 0;
    let unitTypes: BTreeMap<usize, i32> = BTreeMap::new();

    // loop {
        let current_faction = &game_instance.factions[faction_index];
        let current_unit_types = &current_faction.unit_types;
        println!("{:#?}", current_unit_types);
    // }
}
