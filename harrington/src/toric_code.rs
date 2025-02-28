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
