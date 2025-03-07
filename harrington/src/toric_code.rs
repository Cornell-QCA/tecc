
pub enum Location {
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

impl Location {
    pub fn opposite_direction(location: Location) -> Location {
        match location {
            Location::NORTH => Location::SOUTH,
            Location::SOUTH => Location::NORTH,
            Location::WEST => Location::EAST,
            Location::EAST => Location::WEST,
            Location::NORTH_EAST => Location::SOUTH_WEST,
            Location::SOUTH_WEST => Location::NORTH_EAST,
            Location::NORTH_WEST => Location::SOUTH_EAST,
            Location::SOUTH_EAST => Location::NORTH_WEST,
            _ => Location::CENTER,
        }
    }
}

#[derive(Clone, Debug)]
struct ToricCode {
    // size/L 
    // vertices/processors
    // edges/qubits
    // syndroms
    // p - phase flip rate
    // q - measurement error rate
    // TODO: do we need bit flip rate?
}

impl ToricCode {
    pub fn new() -> Self {
        todo!()
    }

    pub fn apply_random_errors(&mut self) {
        todo!()
    }

    pub fn measure_syndromes(&mut self) {
        todo!() // star, plaquette, or both?
    }

    pub fn update_processors(&mut self) {
        todo!()
    }
    
    pub fn apply_correction_rules(&mut self) {
        todo!()
    }

    // should the time step function be here?
}
