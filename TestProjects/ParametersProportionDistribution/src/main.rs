use rand::{Rng};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub values: Vec<u8>,
    pub min_value: u8,
    pub max_value: u8,
}

impl Parameter {
    pub fn new(count: usize) -> Self {
        Self {
            values: vec![0; count],
            min_value: 0,
            max_value: 9,
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
        let number_of_applicable_parameters = self.values.len().saturating_sub(1);

        //Destributing bonus points
        let mut bonus_affected_indices = vec![0; number_of_applicable_parameters];
        for i in 0..(number_of_applicable_parameters) {
            bonus_affected_indices[i] = rand::thread_rng().gen_range(0..self.values.len());
        }

        println!("Indices with bonus points{:?}", bonus_affected_indices);

        for i in &bonus_affected_indices {
            self.try_increase_parameter(*i);
        }

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

        if !unaffected_indices.is_empty() {
            let random_index_in_list = rand::thread_rng().gen_range(0..unaffected_indices.len());
            let actual_index_to_decrease = unaffected_indices[random_index_in_list];

            println!("Chosen index for decrease is {}", actual_index_to_decrease);

            self.try_decrease_parameter(actual_index_to_decrease);
        }
    }

    fn try_increase_parameter(&mut self, index: usize) {
        //searching for EXISTING object in index
        let parameter = self.values.get_mut(index);

        //If index exists - continuing
        match parameter {
            Some(value) => {
                //If lower than max, increase by 1
                if *value < self.max_value {
                    *value += 1;
                    println!("Parameter with index {} increased, now {}", index, *value);
                } else {
                    println!("Parameter with index {} is already at max", index);
                }
            }
            None => println!("Not found parameter to increase with index {:?}", index),
        };
    }

    fn try_decrease_parameter(&mut self, index: usize) {
        let parameter = self.values.get_mut(index);

        match parameter {
            Some(value) => {
                if *value > self.min_value {
                    *value -= 1;
                    println!("Parameter with index {} decreased, now {}", index, *value);
                } else {
                    println!("Parameter with index {} is already at min", index);
                }
            }
            None => println!("Not found parameter to increase with index {:?}", index),
        }
    }
}

fn main() {
    let mut my_params = Parameter::new(4);
    my_params.values = vec![8, 3, 2, 7];
    println!("SUm {}", my_params.sum());

    let mut custom_params = Parameter::new(4);
    custom_params.apply_proportion(vec![5, 5, 5, 5]);
    // custom_params.apply_proportion(vec![0, 0, 0, 0]);
    // custom_params.apply_proportion(vec![9, 9, 9, 9]);


    println!("Params before proportion applied {:?}", custom_params);

    custom_params.apply_random_variance();

    println!(
        "final parameter values after variation applied {:?}",
        custom_params
    );
}
