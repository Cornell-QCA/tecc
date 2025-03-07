type Point = (i32, i32);

#[derive(Clone, Debug)]
pub struct Automaton<'clock> {
    age: &'clock u32, // local copy updated 
    address: Point,
    syndromes: [bool; 9],
    count_signal: [bool; 8],
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],
}

impl<'clock> Automaton<'clock> {
}
