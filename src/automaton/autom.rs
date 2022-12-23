use std::collections::HashMap;

// A state is represented as an integer
pub type State = u8;

pub struct Transition {
    // The state we transition to
    goto : State,
    
    // Increment by
    incr_by : i32,

    // Move by
    move_by : i32,
    
    // Some(true) if we check c=0, Some(false) if we check c!=0, None if we don't care about the counter
    test_counter_zero : Option<bool>,

    // Some(char) if we read some character from the tape, None if we don't care about the tape
    read_char : Option<char>,
}

impl Transition {

}

// Automatons are represented as adjacency lists
pub struct Autom {
    // Adjacency list of states to transitions off of that state
    state_map   : HashMap<State, Vec<Transition>>,

    // Counter to keep track of the number of states in the automaton
    state_total : State,
}


impl Autom {
    pub fn new() -> Self {
        Self { 
            state_map : HashMap::new(), 
            state_total : 0 
        }
    }

    pub fn introduce(&mut self) -> State {
        self.state_total += 1;

        self.state_map.insert(
            self.state_total,
            Vec::new(),
        );

        self.state_total
    }

    pub fn add_transition(&mut self, source : State, trans : Transition) {
        let search_map = self.state_map.get_mut(&source);
        
        match search_map {
            Some(trans_vec) => trans_vec.push(trans),

            None => panic!("State {} doesn't exist in the automaton!", source),
        }
    }


}
