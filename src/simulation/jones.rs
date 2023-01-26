use std::collections::HashMap;

use crate::automaton::determ_autom::{Autom, Transition};
use crate::simulation::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config};
use crate::parser::ast::{Readable, Input};

pub fn jones_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    false
}

struct JonesProcedure<'a> {
    config_table : HashMap<StrippedConfig, DeltaConfig>,

    autom : &'a Autom,

    input : Input,

    stack : Vec<StrippedConfig>,
}

impl<'a> JonesProcedure<'a> {
    // Constructor
    pub fn new(autom : &'a Autom, input : Input) -> Self {
        Self { 
            config_table : HashMap::new(), 
            autom,
            input,
            stack : Vec::new(),
        }
    }

    pub fn simulate(&mut self) {
        
    }
}