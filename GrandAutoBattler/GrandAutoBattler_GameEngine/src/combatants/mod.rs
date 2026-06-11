use std::collections::BTreeMap;

use std::io::{stdout, Write};

use std::error::Error;

pub fn PrintComb() {
    println!("YoCombat");
}

pub struct Combatant {
    id: String,
    faction: Faction,
    unit_type_name: String,
    unit_type_pretty_name: Option<String>,
    parameters: BTreeMap<String, u8>,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub struct Faction {
    name: String,
    unit_types: Vec<UnitType>,
}

pub struct UnitType {
    name: String,
    pretty_name: Option<String>,
    females_only: bool,
    parameters_set: BTreeMap<String, u8>,
    first_names: Option<Vec<String>>,
    last_names: Option<Vec<String>>,
}

pub(crate) fn load_available_factions() -> Result<Vec<Faction>, String> {
    Err("Error".to_string())
}
