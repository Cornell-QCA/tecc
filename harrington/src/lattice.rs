use crate::automaton::Automaton;
use slotmap::{SlotMap, basic};
use std::collections::HashMap;

slotmap::new_key_type! { struct QubitKey; }

enum LatticeType {
    Primary,
    Dual,
}

// The graph owns Automatons and (vecs of) EdgeKeys
pub struct Lattice<'clock, 'automata_ref> {
    colony_size: u32,
    work_period: u32,
    cell_threshold: f64,
    neighbor_threshold: f64,
    clock: u32, // will send a reference w/ 'clock to automata
    primary: HashMap<Automaton<'clock>, Vec<Edge<'automata_ref, 'clock>>>,
    dual: HashMap<Automaton<'clock>, Vec<Edge<'automata_ref, 'clock>>>,
}

pub enum Edge<'automata, 'clock> {
    Primary(LatticeEdge<'automata, 'clock>),
    Dual(LatticeEdge<'automata, 'clock>),
}

pub struct LatticeEdge<'automata, 'clock> {
    automata: (&'automata Automaton<'clock>, &'automata Automaton<'clock>),
    qubit: Option<QubitKey>,
}

impl<'automata, 'clock> Edge<'automata, 'clock> {}

// // If we want this to be thread-safe, we should use Mutex instead of RefCell and check SlotMap
// // the qubits are owned by a slotmap
// let mut qubits: SlotMap<QubitKey, RefCell<Qubit>> = SlotMap::new();
// let mut edges: SlotMap<EdgeKey, RefCell<Qubit>> = SlotMap::new();
