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
        Some(accept) => accept,
    }
}

struct GlueckSimulator {
    // Table that stores the previously computed config terminators
    config_table : HashMap<StrippedConfig, DeltaConfig>,

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
        if let Some(delta_config) = self.config_table.get(&strip_config(config)) {
            // Calculate the new config
            let new_config = Config {
                state : delta_config.state,
                read : delta_config.read,
                counter : config.counter + delta_config.counter,
            };

            return new_config;
        }

        // Check if the config is halting
        if let Some(_) = self.autom.check_if_halting(config.state) {
            // Construct DeltaConfig that gets mapped to
            let map_config = make_delta_config(config, config);

            // Memoize and return
            self.config_table.insert(strip_config(config), map_config);
            return config; 
        }

        // Find out which transition the automaton can take from this config
        let trans_box = get_transition(self.autom.clone(), config, self.input.clone());

        let trans = match trans_box {
            // If no such transition exists, then the automaton halts and rejects on this config
            None => {
                let map_config = make_delta_config(config, config);
                self.config_table.insert(strip_config(config), map_config);
                return config;
            },

            // If such a transition exists, save it in trans
            Some(t) => t,
        };

        // Find the next config
        let next_config = next(
            config, 
            trans, 
            self.input.clone()
        );

        // Simulate from the next config
        let new_config = self.simulate(next_config);

        // Make delta config for memoization
        let map_config = make_delta_config(config, new_config);

        // Memoise and return the new config
        self.config_table.insert(strip_config(config), map_config);
        new_config
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Config {
    // The state the automaton is in
    pub state : State,

    // The index of the read head
    pub read : i32,

    // The value of the counter
    pub counter : i32,
}

// Type alias for configs where counter stores a change in the counter value
pub type DeltaConfig = Config;

// Type alias for configs that have a bool tracking c==0 instead of c
pub type StrippedConfig = (State, i32, bool);

// Function to turn a config into a stripped config
pub fn strip_config(config : Config) -> StrippedConfig {
    (config.state, config.read, config.counter == 0)
}

// Construct a delta config from two other configs
pub fn make_delta_config(from : Config, to : Config) -> DeltaConfig {
    DeltaConfig {
        state : to.state,
        read : to.read,
        counter : to.counter - from.counter,
    }
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

    // TODO : empty counter checking here
    // Find the new counter value

    // Return new config
    Config {
        state   : transition.goto,
        read    : new_read,
        counter : config.counter + transition.incr_by,
    }
}