use std::collections::HashMap;

use crate::automaton::autom::{Autom, State};
use crate::parser::ast::{Readable, Input};
use super::autom::Transition;

pub fn glueck_procedure(autom : Autom, input : &str) -> bool {
    // Convert the input into a list of Readables
    let readable_input = Readable::from_input_str(input);

    // Get the starting configuration of the automaton
    // Automaton always starts in state zero from lend with c=0
    let start_config = Config { state : 0, read : 0, counter : 0 };
    
    // Declare the GlueckSimulator object
    let mut simulator = GlueckSimulator::new(autom.clone(), readable_input);

    // Run the simulator
    let final_config = simulator.simulate(start_config);

    // Return based on the final config
    match autom.check_if_halting(final_config.state) {
        None => false,
        Some(acc) => acc,
    }
}

struct GlueckSimulator {
    // Table that stores the previously computed config terminators
    config_table : HashMap<Config, Config>,

    // Automaton being simulated
    autom : Autom,

    // Input being simulated on
    input : Input,
}

impl GlueckSimulator {
    pub fn new(autom : Autom, input : Input) -> Self {
        Self { 
            config_table : HashMap::new(), 
            autom : autom,
            input : input,
        }
    }

    pub fn simulate(&mut self, config : Config) -> Config {
        // Check if the config has been memoized
        if let Some(next_config) = self.config_table.get(&config) {
            return *next_config;
        }

        // Check if the config is halting
        if let Some(_) = self.autom.check_if_halting(config.state) {
            self.config_table.insert(config, config);
            return config; 
        }

        // Find out which transition the automaton can take from this config
        let trans_box = get_transition(self.autom.clone(), config, self.input.clone());
        let trans;

        // If no such transition exists, then the automaton halts and rejects on this config
        match trans_box {
            None => {
                self.config_table.insert(config, config);
                return config;
            },

            Some(t) => trans = t,
        }

        // Find the next config
        let next_config = next(
            config, 
            trans, 
            self.input.clone()
        );

        // Simulate from the next config
        let new_config = self.simulate(next_config);

        // Memoise and return the new config
        self.config_table.insert(config, new_config);
        new_config
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Config {
    // The state the automaton is in
    pub state : State,

    // The index of the read head
    pub read : i32,

    // The current value of the counter
    pub counter : i32,
}

// Get the transition that the automaton can take from the given configuration, if one exists
pub fn get_transition(autom : Autom, config : Config, input : Input) -> Option<Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        // If the current transition is an epsilon transition, include it
        if trans.test_counter_zero == None && trans.read_char == None {
            legal_transitions.push(trans);
            continue;
        }

        // Check that a zero test passes if it exists
        let mut zero_check_passes = true;
        if let Some(check_zero) = trans.test_counter_zero {
            zero_check_passes = (config.counter == 0) == check_zero;
        } 

        // Check that a read check passes if it exists
        let mut read_check_passes = true;
        if let Some(symbol) = trans.read_char {
            read_check_passes = input[config.read as usize] == symbol;
        }

        // Include the transition if the zero check and the read check both passed
        if zero_check_passes && read_check_passes {
            legal_transitions.push(trans);
        }
    }

    // Check for non-determinism
    if legal_transitions.len() > 1 {
        panic!("This automaton is non-deterministic!");
    }

    // Return based on number of legal transitions 
    match legal_transitions.len() {
        0 => None,
        _ => Some(legal_transitions[0]),
    }
}

pub fn next(config : Config, transition : Transition, input : Input) -> Config {
    // Find the new readhead position
    let mut new_read = config.read + transition.move_by;
    new_read = new_read.max(0).min(input.len().try_into().unwrap());

    // Return new config
    Config {
        state   : transition.goto,
        read    : new_read,
        counter : config.counter + transition.incr_by,
    }
}