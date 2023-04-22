use crate::automaton::determ_autom::Autom;
use crate::simulation::config::{Config, get_transition, next};
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
        let mut cfg = Config { state : 0, read : 0, counter : 0 };

        loop {
            let trans = match get_transition(self.autom, cfg, self.input.clone()) {
                Some(t) => t,
                None => return false,
            };

            
            let next_cfg = match next(
                cfg, 
                trans, 
                &self.input,
                self.autom.decr_zero
            ) {
                None => return false,
                Some(c) => c,
            };


           
            // Check if this state halts; if so return
            if let Some(accepting) = self.autom.check_if_halting(cfg.state) {
                return accepting;
            }

            // Continue from the next config
            cfg = next_cfg;
        }
    }
}