
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
}

fn main() {
   let mut my_params = Parameter::new(4);
   my_params.values = vec![8, 3, 2, 7];
   println!("SUm {}", my_params.sum());
}
