use clearscreen::clear;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};

#[derive(Deserialize)]
struct NameData {
    names: Vec<String>,
}

fn main() {
    let mut rng = rand::thread_rng();

    loop {
        clear();

        // Human Names
        let human_female_first_names = load_names("Assets/Factions/Humans/FemaleNames/FirstNames.toml");
        let human_male_first_names = load_names("Assets/Factions/Humans/MaleNames/FirstNames.toml");
        let human_female_last_names = load_names("Assets/Factions/Humans/FemaleNames/MiddleLastNames.toml");
        let human_male_last_names = load_names("Assets/Factions/Humans/MaleNames/MiddleLastNames.toml");

        // Demon Names
        let demon_female_first_names = load_names("Assets/Factions/Demons/FemaleNames/FirstNames.toml");
        let demon_male_first_names = load_names("Assets/Factions/Demons/MaleNames/FirstNames.toml");
        let demon_female_last_names = load_names("Assets/Factions/Demons/FemaleNames/MiddleLastNames.toml");
        let demon_male_last_names = load_names("Assets/Factions/Demons/MaleNames/MiddleLastNames.toml"); 

        println!("--- HUMAN PARTY ---");
        let mut humans = Vec::new();

        //Female
        for _ in 0..2 {
            let first = human_female_first_names.choose(&mut rng).unwrap();
            let last = human_female_last_names.choose(&mut rng).unwrap();

            humans.push(format!("{} {}", first, last));
        }

        //Male
        for _ in 0..2 {
            let first = human_male_first_names.choose(&mut rng).unwrap();
            let last = human_male_last_names.choose(&mut rng).unwrap();

            humans.push(format!("{} {}", first, last));
        }
        
        for name in &humans {
            println!("{}", name);
        }



        println!("--- DEMON PARTY ---");
        let mut demons = Vec::new();

        //Female
        for _ in 0..2 {
            let first = demon_female_first_names.choose(&mut rng).unwrap();
            let last = demon_female_last_names.choose(&mut rng).unwrap();

            demons.push(format!("{} {}", first, last));
        }

        //Male
        for _ in 0..2 {
            let first = demon_male_first_names.choose(&mut rng).unwrap();
            let last = demon_male_last_names.choose(&mut rng).unwrap();

            demons.push(format!("{} {}", first, last));
        }
        
        for name in &demons {
            println!("{}", name);
        }
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}

fn load_names(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    let data: NameData = toml::from_str(&content).expect("Failed to parse TOML");
    data.names
}
