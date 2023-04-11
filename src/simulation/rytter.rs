use array2d::Array2D;

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::{Config, StrippedConfig, get_transitions, strip_config, next_nondeterm};
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

pub fn rytter_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    false
}

struct RytterSimulator<'a> {
    autom : &'a Autom,

    input : Input,

    n : StrIndex,

    configs : Vec<StrippedConfig>,

    num_configs : usize,

    queue : Vec<(usize, usize)>,

    conf_matrix : Array2D<bool>,
}

impl<'a> RytterSimulator<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        let n = input.len() as StrIndex;

        // Initialise configs list
        let mut configs = Vec::new();
        for state in 0..autom.state_total {
            for index in 0..n {
                for counter in [true, false] {
                    configs.push((state, index as i32, counter));
                }
            }
        }

        let num_configs = configs.len();
        
        // Initialise queue
        let mut queue = Vec::new();

        // Initialise config matrix
        let mut conf_matrix = Array2D::filled_with(false, num_configs, num_configs);

        // Fill the queue and matrix appropriately
        for cfg in 0..num_configs {
            queue.push((cfg, cfg));

            conf_matrix.set(cfg, cfg, true).unwrap();
        }
        
        Self { autom, input, n, configs, num_configs, queue, conf_matrix, }
    }


    fn get_index(&self, conf : StrippedConfig) -> usize {
        for i in 0..self.num_configs {
            if self.configs[i] == conf { return i; }
        }

        panic!("Config doesn't exist!")
    }

    fn below(&self, i : usize, j : usize) -> Vec<usize> {
        // Get ith and jth configs
        let (i_state, i_index, i_counter_zero) = self.configs[i];
        let (j_state, j_index, j_counter_zero) = self.configs[j];

        // Turn these into proper configs

        let i_conf = Config { 
            state : i_state, 
            read : i_index, 
            counter : if i_counter_zero {0} else {1}
        };

        let j_conf = Config { 
            state : j_state, 
            read : j_index, 
            counter : if j_counter_zero {0} else {1}
        };

        // Get the transitions off of j that pop
        let j_transes = get_transitions(self.autom, j_conf, self.input.clone());
        let mut j_pop = Vec::new();
        for trans in j_transes {
            if trans.incr_by < 0 { j_pop.push(trans); }
        }

        // Turn these into configurations
        let mut k_configs = Vec::new();
        for trans in j_pop {
            match next_nondeterm(j_conf, trans, self.input.clone(), self.autom.decr_zero) {
                Some(k_conf) => k_configs.push(k_conf),
                None => continue,
            }
        }
        

        vec![]
    }

    
}