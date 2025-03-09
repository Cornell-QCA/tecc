use crate::automaton::{Automaton, Point};
use slotmap::SlotMap;
use std::collections::HashMap;

pub trait Lattice<'lat_id, 'lat> {
    // TODO: add to this trait, or perhaps just keep it as a state marker
    // I did not put add_colony and remove_colony here because they are not applicable to BaseLattice
    fn insert_lattice_to_store(self, lattice_manager: &mut LatticeManager) -> &'lat_id LatticeId;
    fn get_lattice_from_store(self, lattice_manager: &mut LatticeManager) -> &'lat Self;
}

// TODO: Consider which functions need to be public
pub trait HarringtonRules<'lat_id, 'lat, T: Lattice<'lat_id, 'lat>> {
    // TODO: what should these return?
    // should it put the values in the lattices or return a matrix of syndromes?
    fn compute_syndromes(lattice: T) -> (); 
    fn apply_corrections(lattice: T) -> ();
    fn apply_random_error(lattice: T) -> ();  
    fn update_automata(lattice: T) -> ();
}


slotmap::new_key_type! {
    pub struct LatticeId;
}

slotmap::new_key_type! {
    pub struct QubitKey;
}

slotmap::new_key_type! {
    pub struct AutomatonId;
}

slotmap::new_key_type! {
    pub struct EdgeId;
}


/// LatticeManager is the "higher level" lattice structure.
pub struct LatticeManager<'clock, 'lat_id, 'aut_id> {
    work_period: u32,
    clock: u32, // will send a reference w/ 'clock to automata
    colony_size: u32,
    colony_degree: u32, // `k` in the paper
    cell_threshold: f64,
    neighbor_threshold: f64,

    // Owns the lattices
    top_store: SlotMap<LatticeId, TopLattice<'clock, 'lat_id>>, 
    mid_store: SlotMap<LatticeId, MiddleLattice<'clock, 'lat_id>>, 
    base_store: SlotMap<LatticeId, BaseLattice<'clock, 'aut_id>>, 
    // Owns the Edges
    edges: SlotMap<EdgeId, Edge<'aut_id>>,
    // Owns the Automata
    primary: HashMap<Automaton<'clock>, Vec<Edge<'aut_id>>>, 
    dual: HashMap<Automaton<'clock>, Vec<Edge<'aut_id>>>,
}

// TODO: I now have the LatticeManager owning everything, to get around boundaries between
// BaseLattice's. 
impl<'clock, 'lat_id, 'aut_id> LatticeManager<'clock, 'lat_id, 'aut_id> {
    pub fn new(
        colony_size: u32,
        colony_degree: u32,
        work_period: u32,
        cell_threshold: f64,
        neighbor_threshold: f64,
    ) -> Self {
        let top_store: SlotMap<LatticeId, TopLattice> = SlotMap::with_key();
        let mid_store: SlotMap<LatticeId, MiddleLattice> = SlotMap::with_key();
        let base_store: SlotMap<LatticeId, BaseLattice> = SlotMap::with_key();
        let edges: SlotMap<EdgeId, Edge<'aut_id>> = SlotMap::with_key();
        let primary = HashMap::new();
        let dual = HashMap::new();
        // TODO: create automata_store, primary, and dual
        // TODO: Build the lattice and add it to the store.

        Self {
            work_period,
            clock: 0,
            colony_size,
            colony_degree,
            cell_threshold,
            neighbor_threshold,
            top_store,
            mid_store,
            base_store,
            edges,
            primary,
            dual,
        }
    }


    // There should only be one of these
    pub fn _create_top_lattice(&mut self) {
        todo!()
        // let id = self.insert_lattice_to_store(
        //     TopLattice::new() 
        // );
    }
    
    // Example method for building new lattice
    pub fn _create_middle_lattice(&mut self, _supercolony: LatticeId) -> LatticeId {
        todo!()
        // Adds the new lattice to the LatticeManager
        // Note that I think id is assigned the valued given by vacant_key()
        // let id = MiddleLattice::insert_lattice_to_store(
        //     MiddleLattice::new(store.vacant_key()),
        //     &mut self, 
        // );
        // // Adds the new lattice as a child colony of the given supercolony
        // self.store[_supercolony].colonies.push(id);
        // id
    }

    pub fn _create_base_lattice(&mut self, _supercolony: LatticeId) -> LatticeId {
        todo!()
        // let id = self.insert_lattice_to_store(
        //     BaseLattice::new(store.vacant_key()) 
        // );
        // BaseLattice::get_lattice_from_store(_supercolony).colonies.push(id);
        // id
    }


    // Example method for colony communication
    pub fn _broadcast_to_supercolony(&mut self, _me: LatticeId, _msg: &str) {
        todo!()
    }

    pub fn _add_automaton(
        &mut self,
        aut: Automaton<'clock>,
        edges: Vec<Edge<'aut_id>>
    ) {
       todo!() 
    }
}


// TODO: each of these needs fields that would have corresponeded to the center automata in
// Harrington originally. These are the fields that count the errors and determine if the value
// is greater than the cell cell_threshold, which is a value stored in LatticeManager.
// Perhaps they should have age, or we should remove the lifetimes for Top and Middle
pub struct TopLattice<'clock, 'lat_id> {
    age: &'clock u32,
    colonies: Vec<&'lat_id LatticeId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],
}

impl<'clock, 'lat_id> TopLattice<'clock, 'lat_id> {
    pub fn new(age: &'clock u32) -> Self {
        Self {
            age,
            colonies: Vec::new(),
            count_signal: [false; 8],
            new_count_signal: [false; 8],
            flip_signal: [false; 8],
            new_flip_signal: [false; 8]
        }
    }

    pub fn add_colony(&mut self, id: &'lat_id LatticeId) {
        self.colonies.push(id);
    }

    pub fn remove_colony(&mut self, id: &'lat_id LatticeId) {
        if let Some(index) = self.colonies.iter().position(|colony| *colony == id) {
            self.colonies.remove(index);
        }
    }
}

impl<'clock, 'lat_id, 'lat> Lattice<'lat_id, 'lat> for TopLattice<'clock, 'lat_id> {
    fn insert_lattice_to_store(self, _lattice_manager: &mut LatticeManager) -> &'lat_id LatticeId {
        todo!()
    }
    fn get_lattice_from_store(self, _lattice_manager: &mut LatticeManager) -> &'lat Self {
        todo!()
    }
}

impl<'clock, 'lat_id, 'lat> HarringtonRules<'lat_id, 'lat, TopLattice<'clock, 'lat_id>> for TopLattice<'clock, 'lat_id> {
    fn compute_syndromes(_lattice: TopLattice) -> () {
        todo!()
    } 
    fn apply_corrections(_lattice: TopLattice) -> () {
        todo!()
    }
    fn apply_random_error(_lattice: TopLattice) -> () {
        todo!()
    }  
    fn update_automata(_lattice: TopLattice) -> () {
        todo!()
    }
    // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub struct MiddleLattice<'clock, 'lat_id> {
    age: &'clock u32,
    supercolony: &'lat_id LatticeId,
    colonies: Vec<&'lat_id LatticeId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],

    //Think we need list of neighbors for implementing higher level harrington rules easily
    //Could make specific higher level 'edge' objects to link these but we don't need qubit info stored btw 
    //higher level colonies
    neighbor_colonies: [&'lat_id LatticeId; 8],
}

impl<'clock, 'lat_id> MiddleLattice<'clock, 'lat_id> {
    pub fn new(age: &'clock u32, supercolony: &'lat_id LatticeId, neighbors: [&'lat_id LatticeId; 8]) -> Self {
        Self {
            age,
            supercolony,
            colonies: Vec::new(),
            count_signal: [false; 8],
            new_count_signal: [false; 8],
            flip_signal: [false; 8],
            new_flip_signal: [false; 8],
            neighbor_colonies: neighbors
        }
    }

    pub fn add_colony(&mut self, id: &'lat_id LatticeId) {
        self.colonies.push(id);
    }

    pub fn remove_colony(&mut self, id: &'lat_id LatticeId) {
        if let Some(index) = self.colonies.iter().position(|colony| *colony == id) {
            self.colonies.remove(index);
        }
    }
}

impl<'clock, 'lat_id, 'lat> Lattice<'lat_id, 'lat> for MiddleLattice<'clock, 'lat_id> {
    fn insert_lattice_to_store(self, _lattice_manager: &mut LatticeManager) -> &'lat_id LatticeId {
        todo!()
    }
    fn get_lattice_from_store(self, _lattice_manager: &mut LatticeManager) -> &'lat Self {
        todo!()
    }
}

impl<'clock, 'lat_id, 'lat> HarringtonRules<'lat_id, 'lat, MiddleLattice<'clock, 'lat_id>> for MiddleLattice<'clock, 'lat_id> {
    fn compute_syndromes(_lattice: MiddleLattice<'clock, 'lat_id>) -> () {
        todo!()
    } 
    fn apply_corrections(_lattice: MiddleLattice<'clock, 'lat_id>) -> () {
        todo!()
    }
    fn apply_random_error(_lattice: MiddleLattice<'clock, 'lat_id>) -> () {
        todo!()
    }  
    fn update_automata(_lattice: MiddleLattice<'clock, 'lat_id>) -> () {
        todo!()
    }
    // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub struct BaseLattice<'clock, 'aut_id> {
    age: &'clock u32,
    supercolony: LatticeId,
    // Primary and Dual are now maps from points (indexed relative to that specific lattice) to
    // (AutomataKey) automata, which are owned by LatticeManager. This is to solve the issue of who owns the
    // edges that connect automata owned by separate lattices.
    //
    // The Points are indexed with respect to the individual BaseLattice
    primary: HashMap<Point, &'aut_id AutomatonId>,
    dual: HashMap<Point, &'aut_id AutomatonId>,

    count_signal: [bool; 8], 
    new_count_signal: [bool; 8],
    flip_signal: [bool; 8],
    new_flip_signal: [bool; 8],
    neighbor_colonies: [LatticeId; 8],
}

impl <'clock, 'aut_id>BaseLattice<'clock, 'aut_id> {
    pub fn new(age: &'clock u32, supercolony: LatticeId, neighbors: [LatticeId; 8]) -> Self {
        Self {
            age,
            supercolony,
            primary: HashMap::new(),
            dual: HashMap::new(),

            count_signal: [false; 8],
            new_count_signal: [false; 8],
            flip_signal: [false; 8],
            new_flip_signal: [false; 8],
            neighbor_colonies: neighbors
        }
    }

    pub fn add_primary_point(&mut self, point: Point, aut: &'aut_id AutomatonId) {
        self.primary.insert(point, aut);
    }

    pub fn add_dual_point(&mut self, point: Point, aut: &'aut_id AutomatonId) {
        self.dual.insert(point, aut);
    }
}

impl<'clock, 'aut_id, 'lat_id, 'lat> Lattice<'lat_id, 'lat> for BaseLattice<'clock, 'aut_id> {
    fn insert_lattice_to_store(self, _lattice_manager: &mut LatticeManager) -> &'lat_id LatticeId {
        todo!()
    }
    fn get_lattice_from_store(self, _lattice_manager: &mut LatticeManager) -> &'lat Self {
        todo!()
    }
}

impl <'clock, 'aut_id, 'lat_id, 'lat>HarringtonRules<'lat_id, 'lat, BaseLattice<'clock, 'aut_id>> for BaseLattice<'clock, 'aut_id> {
    fn compute_syndromes(_lattice: BaseLattice<'clock, 'aut_id>) -> () {
        todo!()
    } 
    fn apply_corrections(_lattice: BaseLattice<'clock, 'aut_id>) -> () {
        todo!()
    }
    fn apply_random_error(_lattice: BaseLattice<'clock, 'aut_id>) -> () {
        todo!()
    }  
    fn update_automata(_lattice: BaseLattice<'clock, 'aut_id>) -> () {
        todo!()
    }
    // This will include the methods for modifying count_signal, flip_signal, and their
    // new variants
}


pub enum Edge<'aut_id> {
    Primary(LatticeEdge<'aut_id>),
    Dual(LatticeEdge<'aut_id>),
}

impl<'aut_id> Edge<'aut_id> {
    pub fn new_primary(lattice_edge: LatticeEdge<'aut_id>) -> Self {
        Edge::Primary(lattice_edge)
    }

    pub fn new_dual(lattice_edge: LatticeEdge<'aut_id>) -> Self {
        Edge::Dual(lattice_edge)
    }
}


pub struct LatticeEdge<'aut_id> {
    automata: (&'aut_id AutomatonId, &'aut_id AutomatonId),
    qubit: Option<QubitKey>,
}

impl<'aut_id> LatticeEdge<'aut_id> {
    // Is there much use to this since it is just a wrapper over making a new LatticeEdge without
    // any defaults?
    pub fn new(aut_1: &'aut_id AutomatonId, aut_2: &'aut_id AutomatonId, qubit: Option<QubitKey>) -> Self {
        Self {
            automata: (aut_1, aut_2),
            qubit
        }
    }
}
