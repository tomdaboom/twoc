use crate::automaton::generic_autom::State;

// Configuration of an automaton (i.e. all the information required to keep track of a computation)
#[derive(Debug, Clone, Copy)]
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