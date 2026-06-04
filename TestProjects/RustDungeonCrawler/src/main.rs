// src/main.rs
use rand::Rng;
use std::error::Error;
use std::io;
use std::io::{stdout, Write};

use crossterm::{execute, terminal::{Clear, ClearType}};


mod adventurer;
mod consumable;
mod enemy;
mod weapon;
use crate::consumable::{ConsumableRepository, TomlConsumableRepository};
use crate::enemy::{EnemyRepository, TomlEnemyRepository};
use crate::weapon::{TomlWeaponRepository, WeaponRepository};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    // std::io::stdout()\.
    // let repo = TomlWeaponRepository::new(asset_path_str);
    // clear_screen();
    let weapons_repo = TomlWeaponRepository::new()?;

    let weapons: Vec<weapon::Weapon> = weapons_repo.load_all_possible_weapons()?;

    for weapon in weapons {
        println!("Loaded Weapon: {:?}", weapon);
    }

    let enemies_repo = TomlEnemyRepository::new()?;

    let enemies = enemies_repo.load_all_possible_enemies()?;

    for enemy in enemies {
        println!("Loaded Enemy: {:?}", enemy);
    }

    let consumables_repo = TomlConsumableRepository::new()?;

    let consumables = consumables_repo.load_all_possible_consumables()?;

    for consumable in consumables {
        println!("Loaded Consumable: {:?}", consumable);
    }

    // let mut secret_number = rand::thread_rng().gen_range(1..=100);
    //     println!("{secret_number}");
    //  io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");

    let mut adventurer1: adventurer::Adventurer = adventurer::Adventurer {
        name: "Alice".to_string(),
        health: 100,
        weapon: None,
        mood: "😊".to_string(),
    };

    println!("{:?}", &adventurer1);
    adventurer1.receive_damage(55);
    println!("{:?}", &adventurer1);

    let mut adventurer2: adventurer::Adventurer = adventurer::Adventurer {
        health: 200,
        name: "Zog'ar".to_string(),
        weapon: None,
        mood: "😊".to_string(),
    };

    let adventurers: Vec<adventurer::Adventurer> = vec![adventurer1, adventurer2];
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    

    loop {
        // clear_screen();


        // calculate the event
        let random = get_rand_100();
        match random {
            r if r > 95 => println!("found weapon"),
            r if r > 80 => println!("found consumable"),
            r if r > 60 => println!("enemy attack"),
            _ => println!("nothing happened"),
        }

       

        //main check
        if adventurers.is_empty() {
            println!("GAME OVER");
            break;
        }

        println!("");
        // displaying adventurers health
        for adventurer in &adventurers {
        print!("「Adventurer ({:?}) {:?} with {:?} hp is holding {:?}」 ", adventurer.mood, adventurer.name, adventurer.health, adventurer.weapon);
    }
        


        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // println!("{secret_number}");
        println!("{:?}", get_rand_100());
    }

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(())
}

fn get_rand_100() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn clear_screen() {
    // \x1b[2J clears the screen
    // \x1b[1;1H moves the cursor to the top-left
    // print!("\x1b[2J\x1b[1;1H");
    // print!("\x1b[2J");
        // clearscreen::clear().expect("Failed to clear screen");

        let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();

}
