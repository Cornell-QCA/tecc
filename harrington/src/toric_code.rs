pub enum Location {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTH_EAST,
    SOUTH_WEST,
    NORTH_WEST,
    SOUTH_EAST,
    CENTER,
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
