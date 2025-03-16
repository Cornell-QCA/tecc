#[derive(Clone, Debug, PartialEq)]
pub struct Qubit {
    bit: bool,
    spin: bool,
}

impl Qubit {
    pub fn new() -> Self {
        Self {
            bit: false,
            spin: false,
        }
    }

    pub fn flip_bit(&mut self) {
        self.bit = !self.bit;
    }

    pub fn flip_spin(&mut self) {
        self.spin = !self.spin;
    }
}
