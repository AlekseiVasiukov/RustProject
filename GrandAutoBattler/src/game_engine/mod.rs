mod combatants;
use combatants::PrintComb;
use std::io::{Write, stdout};

use std::error::Error;
// use serde::Deserialize;

pub fn PrintGame() {
    println!("YoGame");
    PrintComb();
}
