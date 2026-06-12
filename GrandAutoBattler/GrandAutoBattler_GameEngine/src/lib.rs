mod combatants;
mod game_instance;
use game_instance::*;

use std::io::{stdout, Write};

use std::error::Error;
pub fn PrintEngine() {
    println!("YoEngine");

    let mut i = GameInstance::new();
    println!("{:#?}", i);

    i.initialize();
    println!("{:#?}", i);
}
