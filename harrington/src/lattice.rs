use std::collections::HashMap;
use slotmap::basic; 
use crate::automaton::Automaton;

// Creates distinct key types for the multiple SlotMaps
new_key_type! { struct QubitKey; }
new_key_type! { struct EdgeKey; }

// Used to ensure that no edge with one automata in the primary lattice and the other automata in the dual
// lattice can exist
enum LatticeType {
    primary,
    dual,
}

// The Graph owns Automatons and (vecs of) EdgeKeys
struct Graph {
    primary_lattice: HashMap<Automaton<LatticeType::primary>, Vec<EdgeKey>>,
    dual_lattice: HashMap<Automaton<LatticeType::dual>, Vec<EdgeKey>>,
}

// Edge just contains refereces
struct Edge<LatticeType::T> {
    automata: (&Automaton<T>, &Automaton<T>),
    qubit: Option<QubitKey>,
}

struct Qubit {
    bit: bool,
    spin: bool,
}

// If we want this to be thread-safe, we should use Mutex instead of RefCell and check SlotMap
// the qubits are owned by a slotmap
let mut qubits: SlotMap<QubitKey, RefCell<Qubit>> = SlotMap::new();
let mut edges: SlotMap<EdgeKey, RefCell<Qubit>> = SlotMap::new();
