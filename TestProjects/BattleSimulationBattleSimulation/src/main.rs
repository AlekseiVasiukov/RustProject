use std::{collections::HashMap, hash::Hash};

#[derive (Debug)]
pub struct Parameters{
    values: HashMap<String, i32>,
    health_points: u32,
}

pub impl Parameters{
    pub fn new() -> Self{
        Self {
            values: HashMap::new(),
            health_points: 0,
        }
    }
}

fn main() {

    let mut combatant1 = Parameters::new();
    combatant1.values.insert("health".to_string(), 5);
    combatant1.values.insert("attack".to_string(), 5);
    combatant1.values.insert("range".to_string(), 5);
    combatant1.values.insert("defense".to_string(), 5);
    combatant1.health_points = 100;

        let mut combatant2 = Parameters::new();
    combatant2.values.insert("health".to_string(), 1);
    combatant2.values.insert("attack".to_string(), 1);
    combatant2.values.insert("range".to_string(), 1);
    combatant2.values.insert("defense".to_string(), 9);
    combatant2.health_points = 5;



    println!("Hello, world!");
}
