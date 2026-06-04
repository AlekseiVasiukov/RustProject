// use weapon;

use crate::weapon::Weapon;

#[derive(Debug)]
pub struct Adventurer {
    pub health: i32,
    pub name: String,
    pub weapon: Option<Weapon>, 
    pub mood: String,
}


impl Adventurer {
    pub fn receive_damage(&mut self, amount: i32) {
        if self.health < amount {
            self.health = 0;
        } else {
            self.health -= amount
        }

        if self.health > 50 {
            self.mood = "😊".to_string();
        } else {
            self.mood = "😔".to_string();
        }
    }

    // fn aaa () {
    //     let weapons_repo = TomlWeaponRepository::new()?;

    //     let weapons: Vec<weapon::Weapon> = weapons_repo.load_all_possible_weapons()?;

    //     for weapon in weapons {
    //         println!("Loaded Weapon: {:?}", weapon);
    //     }
    // }
}
