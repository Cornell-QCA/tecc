#[derive(Clone, Debug, PartialEq)]
pub struct Qubit {
    bit: i32
}

impl Qubit {
    pub fn new() -> Self {
        Self {
            bit: 0
        }
    }

    pub fn flip(&mut self) {
        self.bit = (self.bit + 1) % 2;
    }
}
