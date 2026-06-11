use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeSet;

#[derive(Clone)]
struct IdRegistry {
    used_ids: BTreeSet<String>,
}

impl IdRegistry {
    fn new() -> Self {
        Self {
            used_ids: BTreeSet::new(),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Attack { src: String, dst: String, dmg: u32 },
    //Others shit goes here Heal, Hex, Poison
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Effect {
    Damage {
        src: String,
        dst: String,
        dmg: u32,
        dst_hp_before: f32,
        dst_hp_after: f32,
    },
    // Other effects goes here like Heal_received, Got_Poisoned
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEvent {
    pub round: u32,
    pub turn: u32,
    pub cmd: Command,
    pub effects: Vec<Effect>,
}

struct Journal {
    events: Vec<GameEvent>,
}

impl Journal {
    fn new() -> Self {
        Self { events: Vec::new() }
    }

    fn record_event(&mut self, round: u32, turn: u32, cmd: Command, effects: Vec<Effect>) {
        self.events.push(GameEvent {
            round,
            turn,
            cmd,
            effects,
        });
    }

    fn print_json(&self) {
        let json = serde_json::to_string_pretty(&self.events).unwrap();
        println!("{}", json);
    }

    fn get_events(&self) -> Vec<GameEvent> {
        return self.events.clone();
    }
}

fn main() {
    let mut registry = IdRegistry::new();

    for _ in 0..5 {
        println!("Generated ID: {}", registry.generate_id());
    }

    let mut journal = Journal::new();

    let fake_src = registry.generate_id();
    let fake_dst = registry.generate_id();
    let fake_cmd = Command::Attack {
        src: fake_src.clone(),
        dst: fake_dst.clone(),
        dmg: 20,
    };

    let fake_effects = vec![Effect::Damage {
        src: fake_src.clone(),
        dst: fake_dst.clone(),
        dmg: 10,
        dst_hp_before: 80.0,
        dst_hp_after: 70.0,
    }];

    journal.record_event(1, 1, fake_cmd, fake_effects);

    journal.print_json();

    println!("{:?}", journal.get_events());

    for event in journal.get_events() {
        let single_json = serde_json::to_string_pretty(&event).unwrap();
        println!("{}", single_json);
    }

    println!("Hello, world!");
}
