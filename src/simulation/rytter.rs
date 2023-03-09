use std::collections::VecDeque;

use hashbrown::HashSet;

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::State;
use crate::simulation::config::Config;
use crate::parser::ast::{Readable, Input};

pub type StrIndex = i32;

pub fn rytter_procedure<'a>(autom : &'a Autom, input : &str) -> bool {
    false
}

struct RytterSimulator<'a> {
    autom : &'a Autom,

    input : Input,

    n : StrIndex,

    queue : VecDeque<(StrIndex, StrIndex)>,
}