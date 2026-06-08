use rand::Rng;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use serde_json;

struct IdRegistry {
    used_ids: HashSet<String>,
}

impl IdRegistry {
    fn new() -> Self {
        Self {
            used_ids: HashSet::new(),
        }
    }

    fn generate_id(&mut self) -> String {
        loop {
            let id = rand::random::<u32>();
            let id_str = format!("{:08x}", id);

            //If not exist - save for future and return
            if self.used_ids.insert(id_str.clone()) {
                return id_str;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Attack {
        src: String,
        dst: String,
        dmg: u32,
    },
    //Others shit goes here Heal, Hex, Poison
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Effect {
    Damage {
        src: String,
        dst: String,
        dmg: u32,
        dst_hp: u32,
    },
    // Other effects goes here like Heal_received, Got_Poisoned
}

#[derive(Debug,Serialize,Deserialize)]
pub struct GameEvent {
    pub round: u32,
    pub turn: u32,
    pub cmd: Command,
    pub effect: Effect,
}

fn main() {
    let mut registry = IdRegistry::new();

    for _ in 0..5 {
        println!("Generated ID: {}", registry.generate_id());
    }
    println!("Hello, world!");
}
