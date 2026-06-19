use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Attribute {
    Health, 
    Attack, 
    Range,
    Defense,
}

impl Attribute {
    pub fn all() -> Vec<Self>{
        vec![
            Self::Health,
            Self::Attack,
            Self::Range,
            Self::Defense,
        ]
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Health => "health",
            Self::Attack => "attack",
            Self::Range => "range",
            Self::Defense => "defense",
        }
    }
}

fn main() {
    let mut success_params = HashMap::new();

    for attr in Attribute::all() {
        success_params.insert(attr.as_str().to_string(), Parameter::new(6).unwrap());
    }


    // success_params.insert("health".to_string(), Parameter::new(8).unwrap());
    // success_params.insert("attack".to_string(), Parameter::new(6).unwrap());
    // success_params.insert("range".to_string(), Parameter::new(6).unwrap());
    // success_params.insert("defense".to_string(), Parameter::new(6).unwrap());

    match Character::new(&success_params){
        Ok(character) => println!(" Successful{:?}", character),
        Err(e) => println!("Error  {}", e),
    }

    // match Character::new(&success_params) {
    //     Ok(character) => {
    //         println!("Successful{:?}", character);
    //     }
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
    // }

    let mut failure_params = HashMap::new();
    failure_params.insert("range".to_string(), Parameter::new(9).unwrap());
    failure_params.insert("defense".to_string(), Parameter::new(7).unwrap());
    match Character::new(&failure_params) {
        Ok(character) => {
            println!("Character initialized successfully: {:#?}", character);
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
        if (0..=10).contains(&value) {
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

    pub fn new (external_parameters: &HashMap<String, Parameter>) -> Result<Self, String> {
        for attr in Attribute::all() {
            let key = attr.as_str().to_string();
            if !external_parameters.contains_key(&key) {
                return Err(format!("Initialization failed: '{}' parameter was not found", key));
            }
        }

        let health_param = external_parameters.get("health").unwrap();
        let calculated_hp = health_param.value * 10;

        Ok(Self {
            health_points: calculated_hp,
            parameters: external_parameters.clone(),
        })
    } 
    // pub fn new(external_parameters: &HashMap<String, Parameter>) -> Result<Self, String> {
    //     match external_parameters.get("health") {
    //         Some(health_param) => {
    //             let calculated_hp = health_param.value * 10;
    //             let character = Self {
    //                 health_points: calculated_hp,
    //                 parameters: external_parameters.clone(),
    //             };

    //             Ok(character)
    //         }
    //         None => Err("Initialization failed: 'health' parameter was not found".to_string()),
    //     }
    // }
}
