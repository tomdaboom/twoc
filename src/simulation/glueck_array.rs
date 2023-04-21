//use hashbrown::HashMap;
//use hashbrown::HashSet;

use crate::automaton::determ_autom::Autom;
use crate::simulation::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config, next, get_transition};
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

    // Return based on the final config
    match autom.check_if_halting(final_config.state) {
        None => false,
        Some(accept) => accept,
    }
}

// Struct to hold variables for the Glueck procedure
struct GlueckSimulator<'a> {
    // Automaton being simulated
    autom : &'a Autom,

    // Input being simulated on
    input : Input,

    // Table that stores the previously computed terminators
    config_table : Vec<Option<DeltaConfig>>,

    // Number of configurations in total
    num_configs : usize,

    // Past configurations
    past_configs : Vec<StrippedConfig>,
}

impl<'a> GlueckSimulator<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        let mut config_table = Vec::new();

        let num_configs = (autom.state_total as usize) * input.len() * 2;
        for _ in 0..num_configs { 
            config_table.push(None); 
        }
        
        Self { 
            config_table, 
            autom,
            input,
            num_configs,
            past_configs : Vec::new(),
        }
    }

    // Find the terminator of a given configuration
    pub fn simulate(&mut self, config : Config) -> Config {
        //println!("{:?}", config);

        let stripped_config = strip_config(config);
        let cfg_index = self.get_index(stripped_config);

        // Check for infinite loops
        // If we loop infinitely, return the starting config
        if self.past_configs.contains(&stripped_config) {
            return Config { state : 0, read : 0, counter : 0 };
        }

        // Record config in past configs stack
        self.past_configs.push(stripped_config);

        // Check if we've seen this configuration before
        if let Some(delta_config) = self.config_table[cfg_index] {
            // Calculate the new config and return
            self.past_configs.pop();
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
                self.past_configs.pop();
                return config; 
            } 
        }

        // Find the legal transition from this config if one exists
        let trans = match get_transition(self.autom, config, self.input.clone()) {
            // If no such transition exists, then the automaton halts and rejects on this config
            None => {
                self.past_configs.pop(); 
                return config;
            },

            // If such a transition exists, save it in trans
            Some(t) => t,
        };  

        // Variable to hold the value the procedure should output
        let out;

        // Check if this transition is decrementing
        // i.e. pop(config)
        if trans.incr_by < 0 { 
            // We've found the terminator (yaaaayyyyyyyyyyyy!!!!!!!!!)
            out = config;
        }
        
        // Check if this transition is incrementing
        // i.e. push(config)
        else if trans.incr_by > 0 {  
            // Record the current value of the counter
            //let start_counter_val = config.counter;

            // Find the next configuration
            let next_config = match next(
                config, 
                trans.clone(), 
                &self.input,
                self.autom.decr_zero
            ) {
                None => return config,
                Some(c) => c,
            };

            // Find the terminator of the next configuration
            let next_terminator = self.simulate(next_config);

            // Find the legal transition off of the next terminator if one exists
            let next_terminator_trans = match get_transition(
                self.autom, 
                next_terminator, 
                self.input.clone()
            ) {
                None => return next_terminator,
                Some(t) => t,
            };  

            
            // Find the configuration following the last terminator
            let follow = match next(
                next_terminator, 
                next_terminator_trans,
                &self.input,
                self.autom.decr_zero
            ) {
                None => return next_terminator,
                Some(c) => c,
            };

            // Recurse
            out = self.simulate(follow);

            /*
            if out.counter != start_counter_val {
                println!("Counter not the same");
            }
            */

            //print!("push: {:?}, push_to : {:?}\nnext_term: {:?}\nfollow: {:?}", config, next_config, next_terminator, follow);
            //println!("\nout: {:?}\n", out);
        } 
        
        // op(config)
        else {
            // Find the next configuration
            let next_config = match next(
                config, 
                trans.clone(), 
                &self.input,
                self.autom.decr_zero
            ) {
                None => return config,
                Some(c) => c,
            };

            // Recurse
            out = self.simulate(next_config);
        }

        // Memoize
        let map_config = make_delta_config(config, out);
        self.config_table[cfg_index] = Some(map_config);

        // Return
        self.past_configs.pop();
        out
    }

    fn get_index(&self, config : StrippedConfig) -> usize {
        // Get the state, index and counter
        let (state, index, counter_zero) = config;

        // Compute the relevant offsets based on these values
        let counter_offset = if counter_zero {0} else {1} as usize;
        let index_offset = (index * 2) as usize;
        let state_offset = (state * (self.input.len() as u16) * 2) as usize;

        // The index is the sum of the offsets
        let index = counter_offset + index_offset + state_offset;

        // Panic if this config is too big
        if index > self.num_configs {
            panic!("Config {:?} doesn't exist!", config);
        }

        // Return
        index
    }
}
