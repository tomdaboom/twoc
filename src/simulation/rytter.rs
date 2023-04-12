#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::{HashMap, VecDeque};

use array2d::Array2D;

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::{Config, StrippedConfig, get_transitions, strip_config, next_nondeterm};
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

type InverseTransition = Transition;

pub fn rytter_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    false
}

struct RytterSimulator<'a> {
    autom : &'a Autom,

    input : Input,

    n : StrIndex,

    configs : Vec<StrippedConfig>,

    num_configs : usize,

    queue : VecDeque<(usize, usize)>,

    conf_matrix : Array2D<bool>,

    inverse_state_map : HashMap<State, Vec<Transition>>
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
        let mut queue = VecDeque::new();

        // Initialise config matrix
        let mut conf_matrix = Array2D::filled_with(false, num_configs, num_configs);

        // Fill the queue and matrix appropriately
        for cfg in 0..num_configs {
            queue.push_back((cfg, cfg));

            conf_matrix.set(cfg, cfg, true).unwrap();
        }

        // Construct inverse state map

        let mut inverse_state_map = HashMap::new();
        for state in 0..autom.state_total {
            inverse_state_map.insert(state, Vec::new());
        }

        for from_state in 0..autom.state_total {
            for trans in autom.get_transitions(from_state) {
                let to_state = trans.goto;

                let inverse_trans = InverseTransition {
                    goto : from_state,
                    incr_by : trans.incr_by,
                    move_by : trans.move_by,
                    read_char : trans.read_char,
                    test_counter_zero : trans.test_counter_zero,
                };

                let search_map = inverse_state_map.get_mut(&to_state).unwrap();
                search_map.push(inverse_trans);
            }
        }
        
        Self { 
            autom, 
            input, 
            n, 
            configs, 
            num_configs, 
            queue, 
            conf_matrix, 
            inverse_state_map, 
        }
    }

    // Run the simulator
    pub fn simulate(&mut self) -> bool {
        while !self.queue.is_empty() {
            let (i, j) = self.queue.pop_front().unwrap();

            for (k, l) in self.below(i, j) {
                if !self.conf_matrix.get(k, l).unwrap() {
                    self.conf_matrix.set(k, l, true).unwrap();
                    self.queue.push_back((k, l));
                }
            }

            // for each (k, i) in R such that (k, j) notin R do

            // for each (j, k) in R such that (i, k) notin R do
        }

        // FInd the start config
        let start_conf = self.get_index((0, 0, false));

        // Get the configs j such that (start_conf, j) in R
        let mut end_confs = Vec::new();
        for i in 0..self.num_configs {
            if *self.conf_matrix.get(start_conf, i).unwrap() { 
                end_confs.push(i); 
            }    
        }

        // Accept if any of the end_confs are accepting
        for conf in end_confs {
            let (state, _, counter) = self.configs[conf];
            if let Some(true) = self.autom.check_if_halting(state) {
                return true;
            }
        }

        false
    }

    // Get the index of a given configuration
    fn get_index(&self, conf : StrippedConfig) -> usize {
        for i in 0..self.num_configs {
            if self.configs[i] == conf { return i; }
        }

        panic!("Config doesn't exist!")
    }

    // Find all the configurations below a given configuration
    fn below(&self, i : usize, j : usize) -> Vec<(usize, usize)> {
        // Get ith and jth configs
        let (i_state, i_index, i_counter_zero) = self.configs[i];
        let (j_state, j_index, j_counter_zero) = self.configs[j];

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

        // FIND k CONFIGURATIONS

        // Get the transitions onto i that push
        let i_transes = self.inverse_state_map.get(&i_state).unwrap();
        let mut i_push = Vec::new();
        for trans in i_transes {            
            if trans.incr_by > 0 { i_push.push(trans); }
        }

        // Turn these into configurations
        let mut k_configs = Vec::new();
        for trans in i_push {
            let new_state = trans.goto;
            let new_read = (i_index - trans.move_by).max(0).min(self.n-1);

            for counter_zero in [false, true] {
                // Check that the read statement is correct
                let read_correct = match trans.read_char {
                    Some(c) => self.input[new_read as usize] == c,
                    None => true,
                };

                // Check that the counter condition is correct
                let counter_correct = match trans.test_counter_zero {
                    Some(counter_shouldbe_zero) => counter_zero == counter_shouldbe_zero,
                    None => true,
                };

                // Include the config iff trans is legal
                if read_correct && counter_correct {
                    k_configs.push((new_state, new_read, counter_zero));
                }
            }
        }


        // FIND l CONFIGURATIONS

        // Get the legal transitions off of j that pop
        let j_transes = get_transitions(self.autom, j_conf, self.input.clone());
        let mut j_pop = Vec::new();
        for trans in j_transes {
            if trans.incr_by < 0 { j_pop.push(trans); }
        }

        // Turn these into configurations
        let mut l_configs = Vec::new();
        for trans in j_pop {
            match next_nondeterm(j_conf, trans, self.input.clone(), self.autom.decr_zero) {
                Some(conf) => l_configs.push(strip_config(conf)),
                None => continue,
            }
        }

        // MATCH l AND k ACCORDING TO c==0?
        let mut out = Vec::new();
        for k_conf in &k_configs {
            for l_conf in &l_configs {
                if k_conf.2 == l_conf.2 { 
                    let k_index = self.get_index(*k_conf);
                    let l_index = self.get_index(*l_conf);
                    out.push((k_index, l_index));
                }
            }
        }

        // Return
        out
    }
}