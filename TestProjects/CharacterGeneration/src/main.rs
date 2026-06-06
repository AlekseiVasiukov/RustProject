fn main() {

    let char1: Character = Character {
        health: 100,
        attack: 100,
        range: 100, 
        defense: 100,

    };

    println!("{:?}", char1);
}

#[derive(Debug)]
pub struct Character {
    pub health: i32,
    pub attack: i32,
    pub range: i32,
    pub defense: i32,


}

