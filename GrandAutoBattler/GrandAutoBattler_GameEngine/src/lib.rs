mod combatants;
use crate::combatants::PrintComb;

use std::io::{Write, stdout};

use std::error::Error;
pub fn PrintEngine() {
    println!("YoEngine");

    PrintComb();
}
