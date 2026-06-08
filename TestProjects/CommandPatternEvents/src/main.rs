use rand::Rng;
use std::collections::HashSet;

struct IdRegistry {
    used_ids: HashSet<String>,
}

impl IdRegistry {
    fn new() -> Self {
        Self {
            used_ids: HashSet::new(),
        }
    }

    fn generate_id(&mut self) -> String {
        loop {
            let id = rand::random::<u32>();
            let id_str = format!("{:08x}", id);

            //If not exist - save for future and return
            if self.used_ids.insert(id_str.clone()) {
                return id_str;
            }
        }
    }
}

fn main() {
    let mut registry = IdRegistry::new();

    for _ in 0..5 {
        println!("Generated ID: {}", registry.generate_id());
    }
    println!("Hello, world!");
}
