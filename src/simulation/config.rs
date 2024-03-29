// Types corresponding to automaton configurations

use crate::automaton::generic_autom::State;
use crate::automaton::autom;
//use crate::automaton::autom;
use crate::parser::ast::Input;

// Configuration of an automaton (i.e. all the information required to keep track of a computation)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Config {
    // The state the automaton is in
    pub state : State,

    // The index of the read head
    pub read : i32,

    // The value of the counter
    pub counter : i32,
}

// Type alias for configs where counter stores a change in the counter value
pub type DeltaConfig = Config;

// Type alias for configs that have a bool tracking c==0 instead of c
pub type StrippedConfig = (State, i32, bool);

// Function to turn a config into a stripped config
pub fn strip_config(config : Config) -> StrippedConfig {
    (config.state, config.read, config.counter == 0)
}

// Construct a delta config from two other configs
pub fn make_delta_config(from : Config, to : Config) -> DeltaConfig {
    DeltaConfig {
        state : to.state,
        read : to.read,
        counter : to.counter - from.counter,
    }
}

// Given a config, a determ transition off of it and an input string, find the next config
pub fn next(config : Config, transition : autom::Transition, input : &Input, decr_zero : bool) -> Option<Config> {
    // Find the new readhead position
    let mut new_read = config.read + transition.move_by;
    new_read = new_read.max(0).min(input.len() as i32 - 1);

    // Find the new counter value
    let mut new_counter = config.counter + transition.incr_by;
    if !decr_zero && new_counter < 0 { return None; }
    new_counter = new_counter.max(0);

    // Return new config
    Some(Config {
        state   : transition.goto,
        read    : new_read,
        counter : new_counter,
    })
}

// Given a config, a nondeterm transition off of it and an input string, find the next config
pub fn next_nondeterm(config : Config, transition : autom::Transition, input : &Input, decr_zero : bool) -> Option<Config> {
    // Find the new readhead position
    let mut new_read = config.read + transition.move_by;
    new_read = new_read.max(0).min(input.len() as i32 - 1);

    // Find the new counter value
    let mut new_counter = config.counter + transition.incr_by;
    if !decr_zero && new_counter < 0 { return None; }
    new_counter = new_counter.max(0);

    // Return new config
    Some(Config {
        state   : transition.goto,
        read    : new_read,
        counter : new_counter,
    })
}

// Get the legal nondeterministic transition off of an automaton if one exists
// Get the transition that the automaton can take from the given configuration, if one exists
pub fn get_transition(autom : &autom::Autom, config : Config, input : Input) -> Option<autom::Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        match trans.condition {
            // If this transition has a condition, 
            // check that it's true before adding it to legal transitions
            Some(ref cond) => {
                // Find the character under the readhead
                let read_char = input[config.read as usize];
                
                // Check the condition and push
                if cond.check(read_char, config.counter) {
                    legal_transitions.push(trans);
                }
            },

            // Otherwise, add to legal transitions
            None => legal_transitions.push(trans), 
        }
    }

    // Check that all the potentially conflicting transitions do the same thing
    // If this isn't the case, then the automaton is non-deterministic
    if legal_transitions.len() > 1 {
        // Get a tuple of actions executed by the first transition
        let first_actions = (
            legal_transitions[0].goto, 
            legal_transitions[0].move_by, 
            legal_transitions[0].incr_by
        );

        // Panic if any other legal transition doesn't do exactly the same thing
        for trans in legal_transitions.iter().skip(1) {
            if (trans.goto, trans.move_by, trans.incr_by) != first_actions {
                panic!("From state {:?}, this automaton is nondeterministic!", config.state);
            }
        }
    }

    // Return based on number of legal transitions 
    match legal_transitions.len() {
        0 => None,
        _ => Some(legal_transitions[0].clone()),
    }
}

// Get all the legal nondeterministic transitions off of a given config
pub fn get_transitions(autom : &autom::Autom, config : Config, input : Input) -> Vec<autom::Transition> {
    // Get transitions from the automaton
    let transitions = autom.get_transitions(config.state);

    // Declare vector of legal transitions
    let mut legal_transitions = Vec::new();

    for trans in transitions {
        match trans.condition {
            // If this transition has a condition, 
            // check that it's true before adding it to legal transitions
            Some(ref cond) => {
                // Find the character under the readhead
                let read_char = input[config.read as usize];
                
                // Check the condition and push
                if cond.check(read_char, config.counter) {
                    legal_transitions.push(trans);
                }
            },

            // Otherwise, add to legal transitions
            None => legal_transitions.push(trans), 
        }
    }

    // Return
    legal_transitions
}