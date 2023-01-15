use std::collections::{HashSet, HashMap};

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::automaton::config::Config;
use crate::parser::ast::{Readable, Input};

pub type StrIndex = usize;

pub type StateCounterState = (State, i32, State);

pub fn ahu_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input into a list of Readables
    let readable_input = Readable::from_input_str(input);
    let n = readable_input.len();

    // Initialise the dynamic programming matrix
    let mut matrix : HashMap<(StrIndex, StrIndex), HashSet<StateCounterState>> = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            matrix.insert((i, j), HashSet::new());
        }
    }

    // Initialise the stack
    let mut stack : Vec<(StrIndex, StrIndex, StateCounterState)> = Vec::new();

    false
}

struct AhuSimulator<'a> {
    autom : &'a Autom,

    input : Input,

    matrix : HashMap<(StrIndex, StrIndex), HashSet<StateCounterState>>,

    stack : Vec<(StrIndex, StrIndex, StateCounterState)>,
}

impl<'a> AhuSimulator<'a> {
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        // Initialise the dynamic programming matrix
        let mut matrix : HashMap<(StrIndex, StrIndex), HashSet<StateCounterState>> = HashMap::new();
        for i in 0..input.len() {
            for j in 0..input.len() {
                matrix.insert((i, j), HashSet::new());
            }
        }

        // Initialise the stack
        let stack : Vec<(StrIndex, StrIndex, StateCounterState)> = Vec::new();

        Self {
            autom,
            input,
            matrix,
            stack,
        }
    }

    pub fn delta_pop(&self, i : StrIndex, j : StrIndex) -> Vec<StateCounterState> {
        for state in 0..self.autom.state_total {
            for counter in [0, 1] {
                let config = Config { state, read : i as i32, counter };

                let transitions = get_transitions(self.autom, config, self.input);
            }
        }

        vec![(0, 0, 0)]
    }
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