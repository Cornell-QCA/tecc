use crate::qubit::Qubit;

type Point = (i32, i32);


pub enum Syndrome {
    NORTH = 1,
    SOUTH = 2,
    EAST = 3,
    WEST = 4,
    NORTH_EAST = 5,
    SOUTH_WEST = 6,
    NORTH_WEST = 7,
    SOUTH_EAST = 8,
    CENTER = 9,
}

#[derive(Clone, Debug)]
pub struct Automaton<'clock> {
    age: &'clock u32, // local copy updated 
    address: Point,
    syndromes: [bool; 9],
}

impl<'clock> Automaton<'clock> {
    // TODO: Consider which of these need to be public
    pub fn new(age: &'clock, address: Point) -> Self {
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
