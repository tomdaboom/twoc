use hashbrown::{HashMap, HashSet};

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::Config;
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

pub type StateCounterState = (State, Vec<i32>, State);

pub fn ahu_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    let readable_input = Readable::from_input_str(input);
    let mut simulator = AhuSimulator::new(autom, readable_input);
    simulator.check_if_accepted()
}

struct AhuSimulator<'a> {
    autom : &'a Autom,

    input : Input,

    n : StrIndex,

    matrix : HashMap<(StrIndex, StrIndex), HashSet<StateCounterState>>,

    stack : Vec<(StrIndex, StrIndex, StateCounterState)>,
}

impl<'a> AhuSimulator<'a> {
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        // Initialise the dynamic programming matrix
        let mut matrix : HashMap<(StrIndex, StrIndex), HashSet<StateCounterState>> = HashMap::new();
        for i in 0..input.len() {
            for j in 0..input.len() {
                matrix.insert((i as StrIndex, j as StrIndex), HashSet::new());
            }
        }

        // Record the size of the input
        let n = input.len() as StrIndex;

        // Initialise the stack
        let stack : Vec<(StrIndex, StrIndex, StateCounterState)> = Vec::new();

        Self { autom, input, n, matrix, stack, }
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
        let cell = self.matrix.get_mut(&(i, j)).unwrap();
        cell.insert(elem);
    }

    pub fn get_from_matrix(&self, i : StrIndex, j : StrIndex) -> Vec<StateCounterState> {
        let set = &*self.matrix.get(&(i, j)).unwrap();

        let mut out = Vec::new();
        for elem in set {
            out.push(elem.clone());
        }

        out
    }

    pub fn check_if_accepted(&mut self) -> bool {
        let n = self.n;

        // Step 1
        for d in (-n+1)..(n-1) {
            for i in 0..n {
                if i + d < 0 || i + d >= n { continue; }

                for elem in self.delta_pop(i, i+d) {
                    self.add_to_matrix(i, i+d, elem.clone());
                    self.stack.push((i, i+d, elem.clone()));
                }
            }
        }

        // Step 2
        while !self.stack.is_empty() {
            let (i, j, b) = self.stack.pop().unwrap();

            // a
            for d in (-n)..(n+1) {
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
                for d in (-n)..(n+1) {
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
                if p == 0 && dc.len() == 1 && dc[0] == 0 { return true; }
            }
        }

        false
    }    
}


pub fn convolution(a : Vec<StateCounterState>, b : Vec<StateCounterState>, c : Vec<StateCounterState>) -> Vec<StateCounterState> {
    let mut out = Vec::new();

    for (p, incr, s1) in &a {
        for (s2, decr1, t1) in &b {
            for (t2, decr2, q) in &c {
                let (c1, c2) = (incr[0], incr[1]);

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


pub fn get_transitions(autom : &Autom, config : Config, input : Input) -> Vec<Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        // Compute whether or not the counter is zero and what character is at the read index
        let counter_zero = config.counter == 0;
        let read_char = input[config.read as usize];

        // Work out if the counter check passes
        let counter_check_passes = match trans.test_counter_zero {
            None => true,
            Some(check_counter_zero) => counter_zero == check_counter_zero,
        };

        // Work out if the read check passes
        let read_check_passes = match trans.read_char {
            None => true,
            Some(char) => char == read_char,
        };

        // Include the transition if both pass
        if counter_check_passes && read_check_passes {
            legal_transitions.push(trans);
        }
    }

    // Return
    legal_transitions
}