use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub values: Vec<u8>,
}

impl Parameter {
    pub fn new(count: usize) -> Self {
        Self {
            values: vec![0; count],
        }
    }

    pub fn sum(&self) -> u32 {
        self.values.iter().map(|&v| v as u32).sum()
    }

    pub fn apply_proportion(&mut self, proportion: Vec<u8>) {
        for (i, &val) in proportion.iter().enumerate() {
            if i < self.values.len() {
                self.values[i] = val;
            }
        }
    }

    pub fn apply_random_variance(&mut self) {
        //can add bonus points to many parameter, but guarantying at least one will be unaffected.
        let number_of_applicable_parameters = self.values.len() - 1;

        //Destributing bonus points
        let mut bonus_affected_indices = vec![0; number_of_applicable_parameters];
        for i in 0..(number_of_applicable_parameters) {
            bonus_affected_indices[i] = rand::thread_rng().gen_range(0..self.values.len());
        }

        println!("Indices with bonus points{:?}", bonus_affected_indices);

        //the indices withou bonuses, even if it's only one will be randomly chosen for decreasing by 1 as penalty.
        let mut unaffected_indices = Vec::new();
        for i in 0..self.values.len() {
            if !bonus_affected_indices.contains(&i) {
                unaffected_indices.push(i);
            }
        }

        println!(
            "Indices left for random one point of penalty{:?}",
            unaffected_indices
        );
    }
}

fn main() {
    let mut my_params = Parameter::new(4);
    my_params.values = vec![8, 3, 2, 7];
    println!("SUm {}", my_params.sum());

    let mut custom_params = Parameter::new(4);
    custom_params.apply_proportion(vec![5, 5, 5, 5]);
    println!("Params after proportion applied {:?}", custom_params);

    custom_params.apply_random_variance();
}
