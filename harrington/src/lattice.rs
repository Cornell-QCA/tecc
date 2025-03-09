use crate::automaton::{Automaton, Point};
use slotmap::SlotMap;
use std::collections::HashMap;

trait Lattice {
    // TODO: add to this trait, or perhaps just keep it as a state marker
    // I did not put add_colony and remove_colony here because they are not applicable to BaseLattice
}

// TODO: Consider which functions need to be public
trait HarringtonRules<T: Lattice> {
    // TODO: what should these return?
    // should it put the values in the lattices or return a matrix of syndromes?
    pub fn compute_syndromes(lattice: T) -> (); 
    pub fn apply_corrections(lattice: T) -> ();
    pub fn apply_random_error(lattice: T) -> ();  
    pub fn update_automata(lattice: T) -> ();
}


slotmap::new_key_type! {
    pub struct LatticeId;
}

slotmap::new_key_type! {
    struct QubitKey;
}

slotmap::new_key_type! {
    struct AutomatonId;
}

/// LatticeManager is the "higher level" lattice structure.
pub struct LatticeManager<'clock, 'automata_ref> {
    work_period: u32,
    clock: u32, // will send a reference w/ 'clock to automata
    colony_size: u32,
    colony_degree: u32, // `k` in the paper
    cell_threshold: f64,
    neighbor_threshold: f64,

    // Owns the lattices
    store: SlotMap<LatticeId, Lattice<'clock, 'automata_ref>>,
    // Owns the Automata
    automata_store: SlotMap<AutomatonId, Automaton<'clock>>,
    // Owns the edges
    primary: HashMap<&Automaton<'clock>, Vec<Edge<'automata_ref, 'clock>>>, // I think that is the
    // right syntax for references to automata, though there might need to be a lifetime. I'm not
    // sure if this removes the need from some other lifetimes.
    dual: HashMap<&Automaton<'clock>, Vec<Edge<'automata_ref, 'clock>>>,
}

// TODO: I now have the LatticeManager owning everything, to get around boundaries between
// BaseLattice's. This may require refactoring some of the other code below with the lattices,
// edges, references, and lifetimes.
impl<'clock, 'automata_ref> LatticeManager<'clock, 'automata_ref> {
    pub fn new(
        colony_size: u32,
        colony_degree: u32,
        work_period: u32,
        cell_threshold: f64,
        neighbor_threshold: f64,
    ) -> Self {
        let store: SlotMap<LatticeId, Lattice<'clock, 'automata_ref>> = SlotMap::with_key();
        // TODO: create automata_store, primary, and dual
        // TODO: Build the lattice and add it to the store.

        Self {
            work_period,
            clock: 0,
            colony_size,
            colony_degree,
            cell_threshold,
            neighbor_threshold,
            store,
            primary,
            dual,
        }
    }

    // There should only be one of these
    pub fn _create_top_lattice(&mut self) {
        let id = self.store.insert(
            TopLattice::new() 
        );
    }
    
    // Example method for building new lattice
    pub fn _create_middle_lattice(&mut self, _supercolony: LatticeId) -> LatticeId {
        // Adds the new lattice to the LatticeManager
        // Note that I think id is assigned the valued given by vacant_key()
        let id = self.store.insert(
            MiddleLattice::new(store.vacant_key()) 
        );
        // Adds the new lattice as a child colony of the given supercolony
        self.store[_supercolony].colonies.push(id);
        id
    }

    pub fn _create_base_lattice(&mut self, _supercolony: LatticeId) -> LatticeId {
        let id = self.store.insert(
            BaseLattice::new(store.vacant_key()) 
        );
        self.store[_supercolony].colonies.push(id);
        id
    }


    // Example method for colony communication
    pub fn _broadcast_to_supercolony(&mut self, _me: LatticeId, _msg: &str) {
        todo!()
    }

    pub fn add_automaton(
        &mut self,
        aut: Automaton<'clock>,
        edges: Vec<Edge<'automata_ref, 'clock>>
    ) {
       todo!() 
    }
}


// TODO: each of these needs fields that would have corresponeded to the center automata in
// Harrington originally. These are the fields that count the errors and determine if the value
// is greater than the cell cell_threshold, which is a value stored in LatticeManager.
// Perhaps they should have age, or we should remove the lifetimes for Top and Middle
pub struct TopLattice<'colony_ref> {
    colonies: Vec<LatticeId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],
}

impl TopLattice<'colony_ref> {
    pub fn new() -> Self {
        Self {
            colonies: Vec::new(),
            count_signal: [false; 9],
            new_count_signal: [false; 9],
            flip_signal: [false; 9],
            new_flip_signal: [false; 9]
        }
    }

    pub fn add_colony(&mut self, id: LatticeId) {
        self.colonies.push(id);
    }

    pub fn remove_colony(&mut self, id: LatticeId) {
        self.colonies.pop(id);
    }
}

impl Lattice for TopLattice {
    todo!();
}

impl HarringtonRules for TopLattice {
    todo!(); // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub struct MiddleLattice<'colony_ref> {
    supercolony: LatticeId,
    colonies: Vec<LatticeId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],

    //Think we need list of neighbors for implementing higher level harrington rules easily
    //Could make specific higher level 'edge' objects to link these but we don't need qubit info stored btw 
    //higher level colonies
    neighbor_colonies: [LatticeId; 8],
}

impl <'colony_ref> MiddleLattice<'colony_ref> {
    pub fn new(supercolony: LatticeId, neighbors: [LatticeId; 8]) -> Self {
        Self {
            supercolony,
            colonies: Vec::new(),
            count_signal: [false; 9],
            new_count_signal: [false; 9],
            flip_signal: [false; 9],
            new_flip_signal: [false; 9],
            neighbor_colonies: neighbors
        }
    }

    pub fn add_colony(&mut self, id: LatticeId) {
        self.colonies.push(id);
    }

    pub fn remove_colony(&mut self, id: LatticeId) {
        self.colonies.pop(id);
    }
}

impl Lattice for MiddleLattice {
    todo!();
}

impl HarringtonRules for MiddleLattice {
    todo!(); // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub struct BaseLattice<'automata_ref> {
    supercolony: LatticeId,
    // Primary and Dual are now maps from points (indexed relative to that specific lattice) to
    // (AutomataKey) automata, which are owned by LatticeManager. This is to solve the issue of who owns the
    // edges that connect automata owned by separate lattices.
    //
    // The Points are indexed with respect to the individual BaseLattice
    primary: HashMap<Point, &'automata_ref AutomatonId>,
    dual: HashMap<Point, &'automata_ref AutomatonId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],
    neighbor_colonies: [LatticeId; 8],
}

impl<'automata_ref> BaseLattice<'automata_ref> {
    pub fn new(supercolony: LatticeId, neighbors: [LatticeId; 8]) -> Self {
        Self {
            supercolony,
            primary: HashMap::new(),
            dual: HashMap::new(),

            count_signal: [false; 9],
            new_count_signal: [false; 9],
            flip_signal: [false; 9],
            new_flip_signal: [false; 9]
            neighbor_colonies: neighbors
        }
    }

    pub fn add_primary_point<'automata_ref>(&mut self, point: Point, aut: &'automata_ref AutomatonId) {
        self.primary.push(point, aut);
    }

    pub fn add_dual_point<'automata_ref>(&mut self, point: Point, aut: &'automata_ref AutomatonId) {
        self.dual.push(point, aut);
    }
}

impl Lattice for BaseLattice {
    todo!();
}

impl HarringtonRules for BaseLattice {
    todo!(); // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub enum Edge<'automata_ref> {
    Primary(LatticeEdge<'automata_ref>),
    Dual(LatticeEdge<'automata_ref>),
}

impl<'automata_ref> Edge<'automata_ref> {
    pub fn new_primary(lattice_edge: LatticeEdge<'automata_ref>) -> Self {
        Edge::Primary(lattice_edge)
    }

    pub fn new_dual(lattice_edge: LatticeEdge<'automata_ref>) -> Self {
        Edge::Dual(lattice_edge)
    }
}


pub struct LatticeEdge<'automata_ref> {
    automata: (&'automata_ref AutomatonId, &'automata_ref AutomatonId),
    qubit: Option<QubitKey>,
}

impl<'automata_ref> LatticeEdge<'automata_ref> {
    // Is there much use to this since it is just a wrapper over making a new LatticeEdge without
    // any defaults?
    pub fn new(aut_1: &'automata_ref AutomatonId, aut_2: &'automata_ref AutomatonId, qubit: Option<QubitKey>) -> Self {
        Self {
            aut_1,
            aut_2,
            qubit
        }
    }
}
