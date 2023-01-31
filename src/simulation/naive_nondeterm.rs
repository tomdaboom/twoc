use crate::automaton::autom::Autom;
use crate::simulation::config::{Config, get_transitions, next_nondeterm};
use crate::parser::ast::{Readable, Input};

pub fn naive<'a>(autom : &'a Autom, input : &str) -> bool {
    // Convert the input string into a list of readables 
    let readable_input = Readable::from_input_str(input);

    // Run the simulator 
    let mut simulator = NaiveSimulator::new(autom, readable_input);
    simulator.run()
}

struct NaiveSimulator<'a> {
    // The automaton being simulated
    autom : &'a Autom,

    // The input string
    input : Input,

}

impl<'a> NaiveSimulator<'a> {
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        Self { autom, input }
    }

    pub fn run(&mut self) -> bool {
        // Automaton always starts from 0, read==lend and c==0 
        let start_cfg = Config { state : 0, read : 0, counter : 0 };

        // Vector to store current computation paths
        let mut paths = vec![start_cfg];

        loop {
            if paths.len() == 0 { return false; }

            let mut possible_next_paths = Vec::new();

            for cfg in &paths {
                let transes = get_transitions(
                    self.autom, 
                    *cfg, 
                    self.input.clone()
                );

                for trans in &transes {
                    let next_cfg = next_nondeterm(
                        *cfg, 
                        *trans, 
                        self.input.clone()
                    );

                    possible_next_paths.push(next_cfg);
                }
            }

            paths = Vec::new();

            for cfg in possible_next_paths {
                if let Some(accepting) = self.autom.check_if_halting(cfg.state) {
                    match accepting {
                        true => return true,
                        false => continue,
                    }
                }

                paths.push(cfg);
            }
        }
    }
}