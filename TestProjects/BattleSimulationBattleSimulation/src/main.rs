use rand::Rng;
//I Need the Binary tree, to be sure that order is preserved for items, damn!
use std::collections::BTreeMap;

const BASIC_HEALTH: f32 = 100.0;
const BASIC_ATTACK: f32 = 10.0;
const ATTACK_PERCENT_PER_POINT: f32 = 5.0;
const DEFENSE_PERCENT_PER_POINT: f32 = 5.0;
const HEALTH_PERCENT_PER_POINT: f32 = 5.0;

const ATTACK_MIDDLE_VALUE_PARAMETER: f32 = 4.0;
const DEFENSE_MIDDLE_VALUE_PARAMETER: f32 = 4.0;
const HEALTH_MIDDLE_VALUE_PARAMETER: f32 = 4.0;

#[derive(Debug)]
pub struct Parameters {
    values: BTreeMap<String, i32>,
    health_points: u32,
}

impl Parameters {
    fn new() -> Self {
        Self {
            values: BTreeMap::new(),
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

fn generate_health_points(combatant: &Parameters) -> Result<u32, String> {
    for p in combatant.values.values() {
        if !(0..=9).contains(p) {
            return Err("Parameter is out of range".to_string());
        }
    }

    if !combatant.values.contains_key("health") {
        return Err("Combatant 1 is missing \"health\" parameter".to_string());
    }

    let health_bonus_points = combatant.values["health"] as f32 - HEALTH_MIDDLE_VALUE_PARAMETER;
    println!("health bonus points are {}", health_bonus_points);

    let health_bonus = HEALTH_PERCENT_PER_POINT * health_bonus_points;
    println!("health bonus is {}", health_bonus);

    let health_points = BASIC_HEALTH + (BASIC_HEALTH * health_bonus) / 100.0;

    if health_points < 0.0 {
        Ok(1)
    } else {
        Ok(health_points.trunc() as u32)
    }
}

fn calculate_target_for_hit_based_on_range(combatants: &Vec<Parameters>) -> Result<usize, String> {
    for combatant in combatants {
        for p in combatant.values.values() {
            if !(0..=9).contains(p) {
                return Err("Parameter is out of range".to_string());
            }
        }
    }

    for combatant in combatants {
        if !combatant.values.contains_key("range") {
            return Err(format!("Combatant is missing \"range\" parameter"));
        }
    }

    let mut targets_map = BTreeMap::new();

    for (index, combatant) in combatants.iter().enumerate() {
        let probability = 10 - *combatant.values.get("range").unwrap();
        targets_map.insert(index, probability);
    }

    // pretty-print Debug, cool thing
    println!("probability list {:#?}", &targets_map);

    let probabilities_sum = &targets_map.values().sum();
    println!("probabilities_sum {:#?}", probabilities_sum);

    let random_result = rand::thread_rng().gen_range(1..*probabilities_sum);
    println!("random_result {:#?}", random_result);

    let mut step = 0;
    for target in &targets_map {
        if random_result > step + target.1 {
            step += target.1;
        } else {
            // first display the probabilities line
            println!();
            for &interval in targets_map.values() {
                print!("|");

                if interval > 1 {
                    for _ in 0..interval.saturating_sub(1) {
                        print!("_");
                    }
                }
            }
            //then display pointer of random number
            println!();
            for _ in 0..random_result.saturating_sub(1) {
                print!(" ");
            }
            print!("^");
            println!();

            //And now return the value
            return Ok(*target.0);
        }
    }

    Err("Probabilities list ended up, but chosen number wasn't found our".to_string())
}

fn main() {
    //Testing Attack
    let mut combatant1 = Parameters::new();
    combatant1.values.insert("attack".to_string(), 9);
    combatant1.health_points = 100;

    let mut combatant2 = Parameters::new();
    combatant2.values.insert("defense".to_string(), 7);
    combatant2.health_points = 5;

    match calculate_damage(&combatant1, &combatant2) {
        Err(e) => println!("There was error {}", e),
        Ok(result) => println!("result {}", result),
    }

    //Testing Health initialize
    let mut combatant3 = Parameters::new();
    combatant3.values.insert("health".to_string(), 2);
    let health_points = generate_health_points(&combatant3);

    match health_points {
        Err(e) => println!("There was error {}", e),
        Ok(result) => println!("result {}", result),
    }

    //Testing Target for attack choice
    let mut combatant4 = Parameters::new();
    combatant4.values.insert("range".to_string(), 8);

    let mut combatant5 = Parameters::new();
    combatant5.values.insert("range".to_string(), 2);

    let mut combatant6 = Parameters::new();
    combatant6.values.insert("range".to_string(), 5);

    let mut combatant7 = Parameters::new();
    combatant7.values.insert("range".to_string(), 0);

    let combatants = vec![combatant4, combatant5, combatant6, combatant7];
    let target_for_hit = calculate_target_for_hit_based_on_range(&combatants);
    match target_for_hit {
        Err(e) => println!("There was error calculating target: {}", e),
        Ok(result) => println!("target's index result {}", result),
    };
    println!("Hello, world!");
}
