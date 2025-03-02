use crate::{qubit::Qubit, toric_code::Location};

type Point = (i32, i32);

/// Memory for each automaton. Naming convention is scuffed (copying the other repo's naming
/// convention for now), will fix it later
///
/// TODO: Fix the naming abomination
///
/// TODO: Maybe these are supposed to be arrays instead of vectors? The authors used vectors but
///       more or less treated them like arrays lol
/// NOTE: From what I can see, they should just be arrays
#[derive(Clone, Debug)]
struct Memory {
    age: i32,
    address: Point,
    // lord forgive me for copying the horrific naming + types
    q: i32,
    u: i32,
    b: i32,
    count_signal: Vec<i32>,
    new_count_signal: Vec<i32>,
    flip_signal: Vec<Qubit>,
    new_flip_signal: Vec<Qubit>,
    count: Vec<Vec<i32>>,
    is_flipped_signal: Vec<bool>,
    is_new_flipped_signal: Vec<bool>,
}

impl Memory {
    fn new(address: Point, q: i32, u: i32, b: i32) -> Self {
        Self {
            age: 0,
            address,
            q,
            u,
            b,
            count_signal: Vec::new(),
            new_count_signal: Vec::new(),
            flip_signal: Vec::new(),
            new_flip_signal: Vec::new(),
            count: Vec::new(),
            is_flipped_signal: Vec::new(),
            is_new_flipped_signal: Vec::new(),
        }
    }
}

/// This is the same a a processor, just different naming convention
#[derive(Clone, Debug)]
struct Automaton {
    // TODO: I think there should also be a Point field
    hierarchy_depth: i32,
    f_c: f64,
    f_n: f64,
    memory: Vec<Memory>,
    neighbors: Vec<Automaton>,
    qubits: Vec<Qubit>, // Is this needed? I think this is jut the adjacent qubits. It may be
    // helpful if it is an array of references to the adjacent qubits
    syndrome: [i32; 9],
}

impl Automaton {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set_neighbors(&mut self, neighbors: Vec<Automaton>) {
        self.neighbors = neighbors;
    }

    pub fn set_qubits(&mut self, qubits: Vec<Qubit>) {
        self.qubits = qubits;
    }

    pub fn set_syndrome(&mut self, syndrome: i32) {
        self.syndrome[Location::CENTER as usize] = syndrome;
    }

    pub fn local_dependencies() {
        todo!()

    }

    pub fn execute_flips() {
        todo!()

    }

    pub fn ca_updates() {
        todo!()
    }

    fn print_rule(k: i32, depth: i32) {
        todo!()
    }

    /// TODO: Implement this in a smarter way than the authors did lol. Their implementation of the
    ///       Harrington rules look like a crime against humanity and it's super verbose
    fn local_update_rules(location: Point, syndromes: [i32; 9], bits: [Qubit; 4], q: i32) -> i32 {
        todo!()
    }
}
