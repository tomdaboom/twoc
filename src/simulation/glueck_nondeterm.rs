use hashbrown::{HashMap, HashSet};

use crate::automaton::determ_autom::Autom;
use crate::simulation::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config, next_nondeterm, get_transitions};
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
    let final_configs = simulator.simulate(start_config);

    for final_config in final_configs {
        // Return based on the final config
        if let Some(a) = autom.check_if_halting(final_config.state) {
            if a { return true; }
        }
    }
    
    false
}

// Struct to hold variables for the Glueck procedure
struct GlueckSimulator<'a> {
    // Table that stores the previously computed terminators
    config_table : HashMap<StrippedConfig, Vec<DeltaConfig>>,

    // Automaton being simulated
    autom : &'a Autom,

    // Input being simulated on
    input : Input,

    // Stack of past configurations
    past_configs : HashSet<StrippedConfig>,
}

impl<'a> GlueckSimulator<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        Self { 
            config_table : HashMap::new(), 
            autom,
            input,
            past_configs : HashSet::new(),
        }
    }

    // Find the terminator of a given configuration
    pub fn simulate(&mut self, config : Config) -> Vec<Config> {    
        let stripped_config = strip_config(config);

        // Check for infinite loops
        // If we loop infinitely, return nothing
        if self.past_configs.contains(&stripped_config) {
            return vec![];
        }

        // Record config in past configs stack
        self.past_configs.insert(stripped_config);

        // Check if we've seen this configuration before
        if let Some(delta_configs) = self.config_table.get(&stripped_config) {
            // Calculate the new configs and return
            let mut new_configs = Vec::new();

            for delta_config in delta_configs {
                new_configs.push(Config {
                    state : delta_config.state,
                    read : delta_config.read,
                    counter : config.counter + delta_config.counter,
                });
            }

            return new_configs;
        }

        // Check if this configuration is halting
        if let Some(accepting) = self.autom.check_if_halting(config.state) {
            // Return if we are in a reject state or in an accept state with an empty counter
            if !accepting || (accepting && config.counter == 0) { 
                return vec![config]; 
            } 
        }

        // Find the legal transition from this config if one exists
        let transes = get_transitions(self.autom, config, self.input.clone());
        if transes.len() == 0 { 
            return vec![config]; 
        }  

        // Variable to hold the value the procedure should output
        let mut outs = Vec::new();

        for trans in transes {
            let mut out = Vec::new() ;

            // Check if this transition is decrementing
            // i.e. pop(config)
            if trans.incr_by < 0 { 
                // We've found the terminator (yaaaayyyyyyyyyyyy!!!!!!!!!)
                out.push(config);
            }
            
            // Check if this transition is incrementing
            // i.e. push(config)
            else if trans.incr_by > 0 { 
                // Find the next configuration
                let next_config = match next_nondeterm(
                    config, 
                    trans.clone(), 
                    &self.input,
                    self.autom.decr_zero
                ) {
                    None => continue,
                    Some(c) => c,
                };

                // Find the terminator of the next configuration
                let next_terminators = self.simulate(next_config);

                let mut follow = Vec::new();

                for next_terminator in next_terminators {
                    // Find the legal transitions off of the next terminator if one exists
                    let next_terminator_transes = get_transitions(self.autom, next_terminator, self.input.clone());  
                    
                    for next_terminator_trans in next_terminator_transes {
                        // Find the configuration following the last terminator
                        match next_nondeterm(
                            next_terminator, 
                            next_terminator_trans,
                            &self.input,
                            self.autom.decr_zero
                        ) {
                            None => continue,
                            Some(c) => follow.push(c),
                        };
                    }
                }

                // Recurse
                for cfg in follow {
                    let mut new_cfgs = self.simulate(cfg);
                    out.append(&mut new_cfgs);
                }
            } 
            
            // op(config)
            else {
                // Find the next configuration
                let next_config = match next_nondeterm(
                    config, 
                    trans.clone(), 
                    &self.input,
                    self.autom.decr_zero
                ) {
                    None => continue,
                    Some(c) => c,
                };

                // Recurse
                out.append(&mut self.simulate(next_config));
            }

            outs.append(&mut out);
        }

        // Memoize
        let mut new_delta_configs = Vec::new();
        for out in &outs {
            new_delta_configs.push(make_delta_config(config, *out));
        }
        self.config_table.insert(stripped_config, new_delta_configs);

        // Return
        self.past_configs.remove(&stripped_config);
        outs
    }
}
