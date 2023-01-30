use hashbrown::HashMap;

use crate::automaton::determ_autom::{Autom, Transition};
use crate::simulation::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config, next};
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

    // Run the simulator to find the terminator of this config
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
    // Table that stores the previously computed terminators
    config_table : HashMap<StrippedConfig, DeltaConfig>,

    // Automaton being simulated
    autom : &'a Autom,

    // Input being simulated on
    input : Input,

    // Stack of past configurations
    past_configs : Vec<StrippedConfig>,
}

impl<'a> GlueckSimulator<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        Self { 
            config_table : HashMap::new(), 
            autom,
            input,
            past_configs : Vec::new(),
        }
    }

    // Find the terminator of a given configuration
    pub fn simulate(&mut self, config : Config) -> Config {    
        let stripped_config = strip_config(config);

        // Check for infinite loops
        // If we loop infinitely, return the starting config
        if self.past_configs.contains(&stripped_config) {
            return Config { state : 0, read : 0, counter : 0 };
        }

        // Record config in past configs stack
        self.past_configs.push(stripped_config);

        // Check if we've seen this configuration before
        if let Some(delta_config) = self.config_table.get(&stripped_config) {
            // Calculate the new config and return
            return Config {
                state : delta_config.state,
                read : delta_config.read,
                counter : config.counter + delta_config.counter,
            };
        }

        // Check if this configuration is halting
        if let Some(accepting) = self.autom.check_if_halting(config.state) {
            // Return if we are in a reject state or in an accept state with an empty counter
            if !accepting || (accepting && config.counter == 0) { 
                return config; 
            } 
        }

        // Find the legal transition from this config if one exists
        let trans = match get_transition(self.autom, config, self.input.clone()) {
            // If no such transition exists, then the automaton halts and rejects on this config
            None => return config,

            // If such a transition exists, save it in trans
            Some(t) => t,
        };  

        // Variable to hold the value the procedure should output
        let out : Config;

        // Check if this transition is decrementing
        // i.e. pop(config)
        if trans.incr_by < 0 { 
            // We've found the terminator (yaaaayyyyyyyyyyyy!!!!!!!!!)
            out = config;
        }
        
        // Check if this transition is incrementing
        // i.e. push(config)
        else if trans.incr_by > 0 {  
            // Find the next configuration
            let next_config = next(
                config, 
                trans.clone(), 
                self.input.clone()
            );

            // Find the terminator of the next configuration
            let next_terminator = self.simulate(next_config);

            // Find the legal transition off of the next terminator if one exists
            let next_terminator_trans = match get_transition(self.autom, next_terminator, self.input.clone()) {
                None => return next_terminator,
                Some(t) => t,
            };  

            // Find the configuration following the last terminator
            let follow = next(
                next_terminator, 
                next_terminator_trans,
                self.input.clone()
            );

            // Recurse
            out = self.simulate(follow);
        } 
        
        // op(config)
        else {
            // Find the next configuration
            let next_config = next(
                config, 
                trans.clone(), 
                self.input.clone()
            );

            // Recurse
            out = self.simulate(next_config);
        }

        // Memoize
        let map_config = make_delta_config(config, out);
        self.config_table.insert(stripped_config, map_config);

        // Return
        self.past_configs.pop();
        out
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
            // If this transition has a condition, 
            // check that it's true before adding it to legal transitions
            Some(ref cond) => {
                // Find the character under the readhead
                let read_char = input[config.read as usize];
                
                // Check the condition and push
                if cond.check(read_char, config.counter) {
                    legal_transitions.push(trans);
                }
            },

            // Otherwise, add to legal transitions
            None => legal_transitions.push(trans), 
        }
    }

    // Check that all the potentially conflicting transitions do the same thing
    // If this isn't the case, then the automaton is non-deterministic
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