mod game_data_retriever;
mod game_instance;
mod toml_parser;

use game_instance::*;

use std::io::{stdout, Write};

use std::error::Error;
pub fn PrintEngine() {
    println!("YoEngine");

    // let mut i = GameInstance::new();

    let mut result = match GameInstance::new() {
        Err(e) => {
            println!(
                "Initialization failed. Game continuation is not possible. Error: {}",
                e
            );

            return;
        }

        // Err(e) => println!("Initialization failed. Game continuation is not possible. Error: {}", e),
        Ok(result) => println!("Initialization succeed. {:#?}", result),
    };
}
// {
// Ok(i) => i,
// Err(e) => {
//     println!("Failed to create game instance: {}", e);
//     return;
// }

// println!("{:#?}", i);

// match ;

//  match i.initialize() {
//     Err(e) => println!("Initialization failed. Game continuation is not possible. Error: {}", e),
//     Ok(result) => println!("Initialization succeed. {:#?}", i),
// }
