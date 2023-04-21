use std::collections::VecDeque;
use hashbrown::HashMap;

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::{Config, StrippedConfig, get_transitions, strip_config, next_nondeterm};
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

type InverseTransition = Transition;

// Check if a string is accepted by a nondeterministic automaton using the Rytter procedure
// This should run in O(|input|^3)
// See https://www.sciencedirect.com/science/article/pii/S0019995885800243?via%3Dihub for more info
pub fn rytter_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input into a list of Readables
    let readable_input = Readable::from_input_str(input);

    // Declare the RytterSimulator object
    let mut simulator = RytterSimulator::new(autom, readable_input);

    // Return the result of simulating
    simulator.simulate()
}

struct RytterSimulator<'a> {
    // The automaton being simulated
    autom : &'a Autom,

    // The input being simulated on
    input : Input,

    // The size of the input
    n : StrIndex,

    // All the possible configurations of autom on input
    configs : Vec<StrippedConfig>,

    // How many configs there are in total
    num_configs : usize,

    // The queue of config pairs being considered
    queue : VecDeque<(usize, usize)>,

    // The boolean matrix R (represented as a pair of hashmaps)
    conf_matrix : (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>),

    // The inverse of the automaton's adjacency list
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
        //let mut conf_matrix = Array2D::filled_with(false, num_configs, num_configs);
        let mut conf_matrix = (HashMap::new(), HashMap::new());

        // Fill the queue and matrix appropriately
        for cfg in 0..num_configs {
            queue.push_back((cfg, cfg));

            //conf_matrix.set(cfg, cfg, true).unwrap();
            conf_matrix.0.insert(cfg, vec![cfg]);
            conf_matrix.1.insert(cfg, vec![cfg]);
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
        
        // Return
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
            // Take an element from the queue
            let (i, j) = self.queue.pop_front().unwrap();


            // 2: For each (k, l) in below(i, j), set to true in R and add to the queue
            for (k, l) in self.below(i, j) {                
                if !self.get_matrix(k, l) {
                    self.set_matrix_true(k, l);
                    self.queue.push_back((k, l));
                }
            }


            // 3: For (k, i) in R such that (k, j) not in R, set (k, j) to true in R and add to the queue
            // Get (k, i)s from the matrix
            let kis = self.conf_matrix.1.get(&i).unwrap();

            // Find the (k, j) notin R and set them to true

            let mut set_to_true_j = Vec::new();
            
            for k in kis {
                if !self.get_matrix(*k, j) {
                    set_to_true_j.push(*k);
                    self.queue.push_back((*k, j));
                }
            }

            for k in set_to_true_j {
                self.set_matrix_true(k, j);
            }
            
            // 3: For each (j, k) in R such that (i, k) not in R, set (i, k) to true in R and add to the queue
            let jks = self.conf_matrix.0.get(&j).unwrap();
            let mut set_to_true_i = Vec::new();
            for k in jks {
                if !self.get_matrix(i, *k) {
                    set_to_true_i.push(*k);
                    self.queue.push_back((i, *k));
                }
            }

            for k in set_to_true_i {
                self.set_matrix_true(i, k);
            }
        }
        
        // Find the start config and the configs coming from it
        let start_conf = self.get_index((0, 0, true));
        let end_confs = self.conf_matrix.0.get(&start_conf).unwrap();

        // Accept if any of the end_confs are accepting
        for conf in end_confs {
            let (state, _, _counter) = self.configs[*conf];
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
    }

    // Get conf_matrix[i, j]
    fn get_matrix(&self, i : usize, j : usize) -> bool {
        let vec = self.conf_matrix.0.get(&i).unwrap();
        vec.contains(&j)
    }

    // Insert (i, j) into conf_matrix
    fn set_matrix_true(&mut self, i : usize, j : usize) {
        let vec0 = self.conf_matrix.0.get_mut(&i).unwrap();
        let vec1 = self.conf_matrix.1.get_mut(&j).unwrap();
        vec0.push(j);
        vec1.push(i);
    }

    // Find all the configurations below a given configuration
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
            if trans.incr_by > 0 { i_push.push(trans); }
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

            // Check that the read statement is correct
            let read_correct = match trans.read_char {
                Some(c) => self.input[new_read as usize] == c,
                None => true,
            };

            for counter_zero in [false, true] {
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