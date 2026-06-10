use std::collections::HashMap;

const BASIC_HEALTH: u32 = 100;
const BASIC_ATTACK: f32 = 10.0;
const ATTACK_PERCENT_PER_POINT: f32 = 5.0;
const DEFENSE_PERCENT_PER_POINT: f32 = 5.0;
const ATTACK_MIDDLE_VALUE_PARAMETER: f32 = 4.0;
const DEFENSE_MIDDLE_VALUE_PARAMETER: f32 = 4.0;

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

    if !combatant1.values.contains_key("attack") {
        return Err("Combatant 1 is missing \"attack\" parameter".to_string());
    }

    if !combatant2.values.contains_key("defense") {
        return Err("Combatant 1 is missing \"defense\" parameter".to_string());
    }
    let attack_bonus_points = combatant1.values["attack"] as f32 - ATTACK_MIDDLE_VALUE_PARAMETER;
    println!("attack bonus points are {}", attack_bonus_points);

    let attack_bonus = ATTACK_PERCENT_PER_POINT * attack_bonus_points;
    println!("attack bonus is {}", attack_bonus);

    let incomming_damage = BASIC_ATTACK + (BASIC_ATTACK * attack_bonus) / 100.0;

    println!("incomming damage is {}", incomming_damage);

    //calculating defense
    let defense_bonus_points =
        if combatant2.values["defense"] as f32 > DEFENSE_MIDDLE_VALUE_PARAMETER {
            combatant2.values["defense"] as f32 - DEFENSE_MIDDLE_VALUE_PARAMETER
        } else {
            0.0
        };
    println!("defense bonus points are  {}", defense_bonus_points);

    let defense_bonus = DEFENSE_PERCENT_PER_POINT * defense_bonus_points;
    println!("defense bonus is {}", defense_bonus);

    let damage_taken = incomming_damage - (incomming_damage * defense_bonus) / 100.0;
    println!("damage taken is {}", damage_taken);

    if damage_taken < 0.0 {
        Ok(0)
    } else {
        Ok(damage_taken.trunc() as u32)
    }
}

fn main() {
    let mut combatant1 = Parameters::new();
    combatant1.values.insert("health".to_string(), 5);
    combatant1.values.insert("attack".to_string(), 8);
    combatant1.values.insert("range".to_string(), 5);
    combatant1.values.insert("defense".to_string(), 5);
    combatant1.health_points = BASIC_HEALTH;

    let mut combatant2 = Parameters::new();
    combatant2.values.insert("health".to_string(), 0);
    combatant2.values.insert("attack".to_string(), 0);
    combatant2.values.insert("range".to_string(), 0);
    combatant2.values.insert("defense".to_string(), 8);
    combatant2.health_points = 5;

    match calculate_damage(&combatant1, &combatant2) {
        Err(e) => println!("There was error {}", e),
        Ok(result) => println!("result {}", result),
    }

    println!("Hello, world!");
}
