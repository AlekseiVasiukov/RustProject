use std::collections::HashMap;

#[derive(Debug)]
pub struct Parameters {
    values: HashMap<String, i32>,
    health_points: u32,
}

impl Parameters {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            health_points: 0,
        }
    }
}

fn calculate_damage(combatant1: &Parameters, combatant2: &Parameters) -> Result<u32, String> {
    //checking parameters are within the limits
    for p in combatant1.values.values() {
        if !(0..=9).contains(p) {
            return Err("Parameter is out of range".to_string());
        }
    }

    Ok(0)
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

    match calculate_damage(&combatant1, &combatant2) {
        Err(e) => println!("There was error {}", e),
        Ok(result) => println!("result {}", result),
    }

    println!("Hello, world!");
}
