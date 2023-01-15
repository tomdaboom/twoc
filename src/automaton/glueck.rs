use std::collections::HashMap;

use crate::automaton::determ_autom::{Autom, Transition};
use crate::automaton::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config};
use crate::parser::ast::{Readable, Input};

// Check if a string is accepted by a deterministic automaton using the glueck procedure
// This should run in time O(|input|)
// See https://arxiv.org/pdf/1309.5142.pdf for more info
pub fn glueck_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input into a list of Readables
    let readable_input = Readable::from_input_str(input);

    // Get the starting configuration of the automaton
    // Automaton always starts in state zero from lend with c=0
    let start_config = Config { state : 0, read : 0, counter : 0 };
    
    // Declare the GlueckSimulator object
    let mut simulator = GlueckSimulator::new(autom, readable_input);

    // Run the simulator
    let final_config = simulator.simulate(start_config);

    //println!("\n{:?}", final_config);

    // Return based on the final config
    match autom.check_if_halting(final_config.state) {
        None => false,
        Some(accept) => accept,
    }
}

// Struct to hold variables for the Glueck procedure
struct GlueckSimulator<'a> {
    // Table that stores the previously computed config terminators
    config_table : HashMap<StrippedConfig, DeltaConfig>,

    // Automaton being simulated
    autom : &'a Autom,

    // Input being simulated on
    input : Input,
}

impl<'a> GlueckSimulator<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        Self { 
            config_table : HashMap::new(), 
            autom,
            input,
        }
    }

    // Find the terminator of a given configuration
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

        let trans = match get_transition(self.autom, config, self.input.clone()) {
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

// Get the transition that the automaton can take from the given configuration, if one exists
pub fn get_transition(autom : &Autom, config : Config, input : Input) -> Option<Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        match trans.condition {
            Some(ref cond) => {
                let read_char = input[config.read as usize];
                
                if cond.check(read_char, config.counter) {
                    legal_transitions.push(trans);
                }
            },

            None => legal_transitions.push(trans), 
        }
    }

    // Check that all the potentially conflicting transitions do the same thing
    // If this isn't the case, then the automaton is non-deterministic
    // TODO: Fix this check to deal with or conditions that are subtly deterministic
    if legal_transitions.len() > 1 {
        // Get a tuple of actions executed by the first transition
        let first_actions = (
            legal_transitions[0].goto, 
            legal_transitions[0].move_by, 
            legal_transitions[0].incr_by
        );

        // Panic if any other legal transition doesn't do exactly the same thing
        for trans in legal_transitions.iter().skip(1) {
            if (trans.goto, trans.move_by, trans.incr_by) != first_actions {
                panic!("From state {:?}, this automaton is nondeterministic!", config.state);
            }
        }
    }

    // Return based on number of legal transitions 
    match legal_transitions.len() {
        0 => None,
        _ => Some(legal_transitions[0].clone()),
    }
}

// Given a config a transition off of it and an input string, find the next config
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