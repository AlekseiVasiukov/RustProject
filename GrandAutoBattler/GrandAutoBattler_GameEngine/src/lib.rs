mod game_data_retriever;
mod game_instance;
use game_instance::*;

use std::io::{stdout, Write};

use std::error::Error;
pub fn PrintEngine() {
    println!("YoEngine");

    let mut i = GameInstance::new();
    println!("{:#?}", i);

    // match ;

     match i.initialize() {
        Err(e) => println!("Initialization failed. Game continuation is not possible. Error: {}", e),
        Ok(result) => println!("Initialization succeed. {:#?}", i),
    }

    
}
