#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{VecDeque, HashMap};
use array2d::Array2D;

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::{Config, StrippedConfig, get_transitions, strip_config, next_nondeterm};
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

type InverseTransition = Transition;

pub fn rytter_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input into a list of Readables
    let readable_input = Readable::from_input_str(input);

    // Declare the RytterSimulator object
    let mut simulator = RytterSimulator::new(autom, readable_input);

    // Return the result of simulating
    simulator.simulate()
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
                    // swap to_state and from_state
                    goto : from_state, 

                    // leave everything else alone
                    incr_by : trans.incr_by,
                    move_by : trans.move_by,
                    condition : trans.condition,
                    //read_char : trans.read_char,
                    //test_counter_zero : trans.test_counter_zero,
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
            for k in 0..self.num_configs {
                let ki = *self.conf_matrix.get(k, i).unwrap();
                let kj = *self.conf_matrix.get(k, j).unwrap();

                if ki && !kj {
                    self.conf_matrix.set(k, j, true).unwrap();
                    self.queue.push_back((k, j));
                }
            }

            // for each (j, k) in R such that (i, k) notin R do
            for k in 0..self.num_configs {
                let jk = *self.conf_matrix.get(j, k).unwrap();
                let ik = *self.conf_matrix.get(i, k).unwrap();

                if jk && !ik {
                    self.conf_matrix.set(i, k, true).unwrap();
                    self.queue.push_back((i, k));
                }
            }
            
        }
        
        // Find the start config
        let start_conf = self.get_index((0, 0, true));

        // Get the configs j such that (start_conf, j) in R
        let mut end_confs = Vec::new();
        for i in 0..self.num_configs {
            if *self.conf_matrix.get(start_conf, i).unwrap() { 
                end_confs.push(i); 
            }    
        }

        // Accept if any of the end_confs are accepting
        for conf in end_confs {
            let (state, _, _counter) = self.configs[conf];
            if let Some(true) = self.autom.check_if_halting(state) {
                return true;
            }
        }

        false
    }

    // Get the index of a given configuration
    fn get_index(&self, conf : StrippedConfig) -> usize {
        // Get the state, index and counter
        let (state, index, counter_zero) = conf;

        // Compute the relevant offsets based on these values
        let counter_offset = if counter_zero {0} else {1} as usize;
        let index_offset = (index * 2) as usize;
        let state_offset = ((state as i32) * self.n * 2) as usize;

        // The index is the sum of the offsets
        let index = counter_offset + index_offset + state_offset;

        // Panic if this config is too big
        if index > self.num_configs {
            panic!("Config {:?} doesn't exist!", conf);
        }

        // Return
        index

        /* O(n) search implementation
        for i in 0..self.num_configs {
            if self.configs[i] == conf { return i; }
        }
        */ 
    }

    // Find all the configurations below a given configuration
    // If we can decrement on zero, also include the transitions that decrement
    fn below(&self, i : usize, j : usize) -> Vec<(usize, usize)> {
        // Get ith and jth configs
        let (i_state, i_index, i_counter_zero) = self.configs[i];
        let (j_state, j_index, j_counter_zero) = self.configs[j];

        // If i has an empty counter then we can't have pushed to it, so return empty
        if i_counter_zero { return vec![]; }


        // FIND k CONFIGURATIONS

        // Get the transitions onto i that push
        let i_transes = self.inverse_state_map.get(&i_state).unwrap();
        let mut i_push = Vec::new();
        for trans in i_transes {            
            if trans.incr_by > 0 { 
                i_push.push(trans); 
            }
        }

        // Turn these into configurations
        let mut k_configs = Vec::new();
        for trans in i_push {
            let new_state = trans.goto;
            let new_read = i_index - trans.move_by;//.max(0).min(self.n-1);

            // Invalidate this transition if the readhead has to be in an illegal position for it to work
            if new_read < 0 || new_read >= self.n {
                continue;
            }

            for counter_zero in [false, true] {
                match &trans.condition {
                    // Push if there's no condition to test
                    None => k_configs.push((new_state, new_read, counter_zero)),
    
                    // Push iff the condition passes on the current read and counter values
                    Some(cond) => { 
                        if cond.check(self.input[new_read as usize], if counter_zero {0} else {1}) {
                            k_configs.push((new_state, new_read, counter_zero));
                        }
                    }, 
                };
            }
        }


        // FIND l CONFIGURATIONS

        let j_conf = Config { 
            state : j_state, 
            read : j_index, 
            counter : if j_counter_zero {0} else {1}
        };

        // Get the legal transitions off of j that pop
        let j_transes = get_transitions(self.autom, j_conf, self.input.clone());
        let mut j_pop = Vec::new();
        for trans in j_transes {
            if trans.incr_by < 0 { j_pop.push(trans); }
        }

        // Turn these into configurations (minus the counter condition, as this will be dictated by k_configs)
        let mut l_configs = Vec::new();
        for trans in j_pop {
            match next_nondeterm(j_conf, trans, &self.input, self.autom.decr_zero) {
                Some(conf) => {
                    let (state, read, _) = strip_config(conf);
                    l_configs.push((state, read));
                },
                None => continue,
            }
        }

        /*
        if i_state == 6 && j_state == 6 {
            println!("kconfs: {:?}", k_configs);
            println!("lconfs: {:?}\n", l_configs);
        }
        */

        // MATCH l AND k ACCORDING TO c==0?
        let mut out = Vec::new();
        for k_conf in &k_configs {
            for (l_state, l_index) in &l_configs {
                let k_index = self.get_index(*k_conf);
                let l_index = self.get_index((*l_state, *l_index, k_conf.2));
                out.push((k_index, l_index));
            }
        }

        // Return
        out
    }
}