use crate::automaton::autom::{Autom, State};
use crate::parser::ast::Readable;

use super::autom::Transition;

pub fn glueck_procedure(autom : Autom, input : &str) -> bool {
    false
}

pub struct Config {
    // The state the automaton is in
    state : State,

    // The index of the read head
    read : usize,

    // The current value of the counter
    counter : i32,

    // Some(true) if accepting, Some(false) if rejecting, None otherwise
    halting : Option<bool>
}

// Get the transition that the automaton can take from the given configuration, if one exists
pub fn get_transition(autom : Autom, config : Config, input : Vec<Readable>) -> Option<Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        // If the current transition is an epsilon transition, include it
        if trans.test_counter_zero == None && trans.read_char == None {
            legal_transitions.push(trans);
            continue;
        }

        // Check that a zero test passes if it exists
        let zero_check_passes;
        if let Some(check_zero) = trans.test_counter_zero {
            zero_check_passes = (config.counter == 0) == check_zero;
        } else {
            zero_check_passes = true;
        }

        // Check that a read check passes if it exists
        let read_check_passes;
        if let Some(symbol) = trans.read_char {
            read_check_passes = input[config.read] == symbol;
        } else {
            read_check_passes = true;
        }

        // Include the transition if the zero check and the read check both passed
        if zero_check_passes && read_check_passes {
            legal_transitions.push(trans);
        }
    }

    // Check for non-determinism
    if legal_transitions.len() > 1 {
        panic!("This automaton is non-deterministic!");
    }

    // Return
    if legal_transitions.len() == 0 {
        None
    } else {
        Some(legal_transitions[0])
    }
}

pub fn next(config : Config, transition : Transition, input : Vec<Readable>) -> Config {
    let mut new_config = Config {
        state : transition.goto,
        read : config.read + transition.move_by,
        counter : config.counter + transition.incr_by,
        halting : None,
    };

    new_config
}