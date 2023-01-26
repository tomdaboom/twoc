use crate::automaton::generic_autom::State;
use crate::automaton::determ_autom::Transition;
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

// Given a config a transition off of it and an input string, find the next config
pub fn next(config : Config, transition : Transition, input : Input) -> Config {
    // Find the new readhead position
    let mut new_read = config.read + transition.move_by;
    new_read = new_read.max(0).min(input.len().try_into().unwrap());

    // Find the new counter value
    let mut new_counter = config.counter + transition.incr_by;
    new_counter = new_counter.max(0);

    // Return new config
    Config {
        state   : transition.goto,
        read    : new_read,
        counter : new_counter,
    }
}