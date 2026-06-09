
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

}

fn main() {
   
}
