use hashbrown::HashSet;
use array2d::Array2D;

use crate::automaton::autom::Autom;
use crate::automaton::generic_autom::State;
use crate::simulation::config::{Config, get_transitions};
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

pub type StateCounterState = (State, Vec<i32>, State);

pub fn ahu_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input string into a list of readables 
    let readable_input = Readable::from_input_str(input);
    
    // Run the simulator 
    let mut simulator = AhuSimulator::new(autom, readable_input);
    simulator.check_if_accepted()
}

struct AhuSimulator<'a> {
    // The automaton being simulated
    autom : &'a Autom,

    // The input string
    input : Input,

    // The size of the input
    n : StrIndex,

    // The dynamic programming matrix
    matrix : Array2D<HashSet<StateCounterState>>,

    // The outputs of delta_pop
    deltapop : Array2D<Vec<StateCounterState>>,

    // The outputs of delta_push
    deltapush : Array2D<Vec<StateCounterState>>,

    // The stack used by the algorithm
    stack : Vec<(StrIndex, StrIndex, StateCounterState)>,
}

impl<'a> AhuSimulator<'a> {
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        // Record the size of the input
        let n = input.len() as StrIndex;

        // Initialise the dynamic programming matrix
        let matrix = Array2D::filled_with(HashSet::new(), n as usize, n as usize);

        // Initialise the stack
        let stack : Vec<(StrIndex, StrIndex, StateCounterState)> = Vec::new();

        // Declare the deltapop array
        let deltapop = Array2D::filled_with(Vec::new(), n as usize, n as usize);

        // Declare the deltapush array
        let deltapush = Array2D::filled_with(Vec::new(), n as usize, n as usize);

        Self { autom, input, n, matrix, deltapop, deltapush, stack }
    }

    pub fn delta_pop(&self, i : StrIndex, j : StrIndex) -> Vec<StateCounterState> {
        // Output vector containing triples of states, counter symbols, and other states
        let mut out  = Vec::new();

        for state in 0..self.autom.state_total {
            for counter in [0, 1] {
                // Construct the appropriate config
                let config = Config { state, read : i as i32, counter };

                // Find the transitions that can be taken from this config
                let transitions = get_transitions(self.autom, config, self.input.clone());

                for trans in transitions {
                    // Check if the transition decrements the counter
                    let decrementing = trans.incr_by < 0;
                    
                    // Check if the counter moves from index i to index j
                    let new_index = (i + trans.move_by).max(0).min(self.n);
                    let moves_to_j = new_index == j;

                    if decrementing && moves_to_j  {
                        out.push((state, vec![counter], trans.goto));
                    }
                }
            }
        }

        // Return
        out
    }

    pub fn delta_push(&self, i : StrIndex, j : StrIndex) -> Vec<StateCounterState> {
        // Output vector containing triples of states, counter symbols, and other states
        let mut out  = Vec::new();

        for state in 0..self.autom.state_total {
            for counter in [0, 1] {
                // Construct the appropriate config
                let config = Config { state, read : i, counter };

                // Find the transitions that can be taken from this config
                let transitions = get_transitions(self.autom, config, self.input.clone());

                for trans in transitions {
                    // Check if the transition increments the counter
                    let incrementing = trans.incr_by >= 0;
                    
                    // Check if the counter moves from index i to index j
                    let new_index = (i + trans.move_by).max(0).min(self.n);
                    let moves_to_j = new_index == j;

                    if incrementing && moves_to_j  {
                        //println!("delta_push is nonempty!");
                        out.push((state, vec![counter, trans.incr_by], trans.goto));
                    }
                }
            }
        }

        // Return
        out
    }

    // TODO: Change these if I come up with a more efficient table implementation

    pub fn add_to_matrix(&mut self, i : StrIndex, j : StrIndex, elem : StateCounterState) {
        let cell = self.matrix.get_mut(i as usize, j as usize).unwrap();
        cell.insert(elem);
    }

    pub fn get_from_matrix(&self, i : StrIndex, j : StrIndex) -> Vec<StateCounterState> {
        let set = self.matrix.get(i as usize, j as usize).unwrap();

        let mut out = Vec::new();
        for elem in set {
            out.push(elem.clone());
        }

        out
    }

    pub fn check_if_accepted(&mut self) -> bool {
        let n = self.n;

        // Step 1
        for d in [-1, 0, 1] {
            for i in 0..n {
                if i + d < 0 || i + d >= n { continue; }

                for elem in self.delta_pop(i, i+d) {
                    self.add_to_matrix(i, i+d, elem.clone());
                    self.stack.push((i, i+d, elem.clone()));
                }
            }
        }

        println!("Step 1 completed");

        // Step 2
        while !self.stack.is_empty() {
            let (i, j, b) = self.stack.pop().unwrap();

            // a
            for d in [-1, 0, 1] {
                if i - d < 0 || i - d >= n { continue; }

                for k in 0..n {
                    let dpush = self.delta_push(i-d, i);
                    let cell = self.get_from_matrix(j, k);

                    for elem in convolution(dpush, vec![b.clone()], cell) {
                        self.add_to_matrix(i-d, k, elem.clone());
                        self.stack.push((i-d, k, elem.clone()));
                    }
                }
            }

            // b
            for h in 0..n {
                for d in [-1, 0, 1] {
                    if h + d < 0 || h + d >= n { continue; }

                    let dpush = self.delta_push(h, h+d);
                    let cell = self.get_from_matrix(h+d, i);

                    for elem in convolution(dpush, cell, vec![b.clone()]) {
                        self.add_to_matrix(h, j, elem.clone());
                        self.stack.push((h, j, elem.clone()));
                    }
                }
            }
            
        }

        println!("Step 2 completed");

        // Step 3   
        /*
        for i in 0..n {
            for j in 0..n {
                println!("{:?}", self.get_from_matrix(i, j));
            }
        }
        */

        //println!("\n\nr(0, n-1) = {:?}", self.get_from_matrix(0, n-1));

        for (p, dc, q) in self.get_from_matrix(0, n-1) {
            if let Some(true) = self.autom.check_if_halting(q) {
                println!("{:?}", (p, dc.clone(), q));

                if p == 0 && dc.len() == 1 && dc[0] == 0 { return true; }
            }
        }

        false
    }    
}


pub fn convolution(a : Vec<StateCounterState>, b : Vec<StateCounterState>, c : Vec<StateCounterState>) -> Vec<StateCounterState> {
    let mut out = Vec::new();

    for (p, incr, s1) in &a {
        let (c1, c2) = (incr[0], incr[1]);

        for (s2, decr1, t1) in &b {
            if *s1 != *s2 { continue; }
            if decr1[0] != c2 { continue; }

            for (t2, decr2, q) in &c {
                let states_correct = (*s1 == *s2) && (*t1 == *t2);
                let decrs_correct = (decr1[0] == c2) && (decr2[0] == c1);

                if states_correct && decrs_correct {
                    //println!("Convolution has a member!");
                    out.push((*p, decr2.clone(), *q));
                }
            }
        }
    }

    out
}