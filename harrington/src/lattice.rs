use crate::automaton::Automaton;
use slotmap::SlotMap;
use std::collections::HashMap;

enum LatticeType {
    Primary,
    Dual,
}

slotmap::new_key_type! {
    pub struct LatticeId;
}

slotmap::new_key_type! {
    struct QubitKey;
}

/// LatticeManager is the "higher level" lattice structure.
pub struct LatticeManager<'clock, 'automata_ref> {
    work_period: u32,
    clock: u32, // will send a reference w/ 'clock to automata
    colony_size: u32,
    colony_degree: u32, // `k` in the paper
    cell_threshold: f64,
    neighbor_threshold: f64,

    store: SlotMap<LatticeId, Lattice<'clock, 'automata_ref>>,
}

impl<'clock, 'automata_ref> LatticeManager<'clock, 'automata_ref> {
    pub fn new(
        colony_size: u32,
        colony_degree: u32,
        work_period: u32,
        cell_threshold: f64,
        neighbor_threshold: f64,
    ) -> Self {
        let store: SlotMap<LatticeId, Lattice<'clock, 'automata_ref>> = SlotMap::with_key();

        // TODO: Build the lattice and add it to the store.

        Self {
            work_period,
            clock: 0,
            colony_size,
            colony_degree,
            cell_threshold,
            neighbor_threshold,
            store,
        }
    }

    // Example method for building new lattice
    pub fn _create_lattice(&mut self, _supercolony: Option<LatticeId>) -> LatticeId {
        todo!();
        // let id = self.store.insert(Lattice {
        //     supercolony,
        //     colonies: vec![],
        // });
        // if let Some(pid) = supercolony {
        //     self.store[pid].colonies.push(id);
        // }
        // id
    }

    // Example method for colony communication
    pub fn _broadcast_to_supercolony(&mut self, _me: LatticeId, _msg: &str) {
        todo!()
    }
}

pub struct Lattice<'clock, 'automata_ref> {
    supercolony: Option<LatticeId>,
    colonies: Vec<LatticeId>,

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
