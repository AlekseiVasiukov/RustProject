use std::collections::HashMap;

fn main() {
    let mut success_params = HashMap::new();
    success_params.insert("health".to_string(), Parameter::new(8).unwrap());
    success_params.insert("attack".to_string(), Parameter::new(6).unwrap());
    success_params.insert("range".to_string(), Parameter::new(6).unwrap());
    success_params.insert("defense".to_string(), Parameter::new(6).unwrap());

    match Character::new(&success_params) {
        Ok(character) => {
            println!("Successful{:?}", character);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let mut failure_params = HashMap::new();
    failure_params.insert("range".to_string(), Parameter::new(9).unwrap());
    failure_params.insert("defense".to_string(), Parameter::new(7).unwrap());
    match Character::new(&failure_params) {
        Ok(character) => {
            println!("Character initialized successfully: {:?}", character);
        }
        Err(e) => {
            println!("Error as expected: {}", e);
            println!("Original parameters map still exists: {:?}", failure_params);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    value: i32,
}

impl Parameter {
    fn new(value: i32) -> Option<Self> {
        if value >= 0 && value <= 10 {
            Some(Self { value })
        } else {
            Some(Self { value: 0 })
        }
    }
}

#[derive(Debug)]
pub struct Character {
    health_points: i32,
    parameters: HashMap<String, Parameter>,
}

impl Character {
    pub fn new(external_parameters: &HashMap<String, Parameter>) -> Result<Self, String> {
        match external_parameters.get("health") {
            Some(health_param) => {
                let calculated_hp = health_param.value * 10;
                let character = Self {
                    health_points: calculated_hp,
                    parameters: external_parameters.clone(),
                };

                Ok(character)
            }
            None => Err("Initialization failed: 'health' parameter was not found".to_string()),
        }
    }
}
