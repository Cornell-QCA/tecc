use crate::qubit::Qubit;

pub type Point = (i32, i32);

pub enum Syndrome {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
    NorthEast = 5,
    SouthWest = 6,
    NorthWest = 7,
    SouthEast = 8,
    Center = 9,
}

#[derive(Clone, Debug)]
pub struct Automaton<'clock> {
    age: &'clock u32, // local copy updated 
    address: Point,
    syndromes: [bool; 9],
}

impl<'clock> Automaton<'clock> {
    // TODO: Consider which of these need to be public
    pub fn new(age: &'clock u32, address: Point) -> Self {
        Self {
            age,
            address,
            syndromes: [false; 9]
        }
    }

    // access the hashmap in LatticeManager
    pub fn measure_syndrome() -> bool {
        todo!();
    }
    
    pub fn apply_star() {
        todo!();
    }

    pub fn apply_plaquette() {
        todo!();
    }

    pub fn apply_random_error() {
        todo!();
    }
}
