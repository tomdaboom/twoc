use std::collections::{HashMap, HashSet};

use crate::parser::ast::Readable;

// A state is represented as an integer
pub type State = u16;

#[derive(Debug, Clone, Copy)]
pub struct Transition {
    // The state we transition to
    pub goto : State,
    
    // Increment by
    pub incr_by : i32,

    // Move by
    pub move_by : i32,
    
    // Some(true) if we check c==0, Some(false) if we check c!=0, None if we don't care about the counter
    pub test_counter_zero : Option<bool>,

    // Some(char) if we read some character from the tape, None if we don't care about the tape
    pub read_char : Option<Readable>,
}

// These implementations are constructors for specific kinds of transitions
impl Transition {
    // Construct a new transition that corresponds to a basic block
    pub fn new_basic_block_trans(next_state : State, mv : i32, ic : i32) -> Self {
        Self {
            goto : next_state,
            
            move_by : mv,
            incr_by : ic,
            
            test_counter_zero : None,
            read_char : None,
        }
    }

    // Construct an epsilon transition
    pub fn new_epsilon_trans(next_state : State) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            test_counter_zero : None,
            read_char : None,
        }
    }

    // Construct a transition that reads a character or an endmarker from the tape
    pub fn new_read_trans(next_state : State, read : Readable) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            test_counter_zero : None,
            read_char : Some(read),
        }
    }

    // Construct a transition that checks if the counter is zero or not
    pub fn new_checkzero_trans(next_state : State, counter_is_zero : bool) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            test_counter_zero : Some(counter_is_zero),
            read_char : None
        }
    }

    // Display the transition
    pub fn print(&self) {
        // Print the state to move to
        print!("goto {:?}", self.goto);

        // Print the move
        if self.move_by != 0 {
            print!(" move({:?})", self.move_by);
        }

        // Print the increment
        if self.incr_by != 0 {
            print!(" c+={:?}", self.incr_by);
        }

        // Print the counter check
        if let Some(is_zero) = self.test_counter_zero {
            if is_zero {
                print!(" c==0");
            } else {
                print!(" c!=0");
            }
        }

        // Print the readhead check
        if let Some(read) = self.read_char {
            print!(" read==");
            match read {
                Readable::Char(c) => print!("{:?}", c),

                Readable::LEnd() => print!("lend"),

                Readable::REnd() => print!("rend"),
            }
        }
    }
}

// Automatons are represented as adjacency lists
#[derive(Debug, Clone)]
pub struct Autom {
    // Adjacency list of states to transitions off of that state
    state_map : HashMap<State, Vec<Transition>>,

    // Counter to keep track of the number of states in the automaton
    state_total : State,

    // Vector to keep track of accepting states
    accepting : Vec<State>,

    // Vector to keep track of rejecting states
    rejecting : Vec<State>,

    // The tape alphabet (excluding the endmarkers)
    pub alpha : HashSet<char>,
}


impl Autom {
    // Create a new empty automaton
    pub fn new(char_set : HashSet<char>) -> Self {
        Self { 
            state_map : HashMap::new(), 
            state_total : 0,
            accepting : Vec::new(),
            rejecting : Vec::new(), 
            alpha : char_set,
        }
    }

    // Introduce a new state to the automaton
    pub fn introduce(&mut self) -> State {
        // Add a new state to the adjacency list
        self.state_map.insert(
            self.state_total,
            Vec::new(),
        );

        // Increment the total number of states
        self.state_total += 1;

        // Return the new state
        self.state_total-1
    }

    // Add a new transition to the automaton
    pub fn add_transition(&mut self, source : State, trans : Transition) {
        // Find the state in the adjacency list
        let search_map = self.state_map.get_mut(&source);
        
        // Push the transition to the adjacency list or panic
        match search_map {
            Some(trans_vec) => trans_vec.push(trans),

            None => panic!("State {} doesn't exist in the automaton!", source),
        }
    }

    // Turn a given state into an accept state
    pub fn make_accept_state(&mut self, state : State) {
        self.accepting.push(state);
    }

    // Turn a given state into a reject state
    pub fn make_reject_state(&mut self, state : State) {
        self.rejecting.push(state);
    }

    // Display the automaton
    pub fn print(&self) {
        // Display the states
        print!("States: 0-{:?}\n\n", self.state_total-1);

        // Display each of the transitions off of each state
        println!("Transitions:");
        for state in 0..self.state_total {
            print!("  From {:?}:\n", state);

            for trans in self.state_map.get(&state).unwrap() {
                print!("    ");
                trans.print();
                println!();
            }
        }
        println!();

        // Display the accepting states
        if self.accepting.len() > 0 {
            println!("Accepting:");
            for state in &self.accepting {
                println!("  {:?}", *state);
            }
        }

        // Display the rejecting states
        if self.rejecting.len() > 0 {
            println!("Rejecting:");
            for state in &self.rejecting {
                println!("  {:?}", *state);
            }
        }
    }

    // Get rid of all transitions coming from halting states
    pub fn clean_halting_states(&mut self) {
        for state in &self.accepting {
            let transitions = self.state_map.get_mut(state).unwrap();
            *transitions = Vec::new();
        }

        for state in &self.rejecting {
            let transitions = self.state_map.get_mut(state).unwrap();
            *transitions = Vec::new();
        }
    }

    // Add a transition to empty the counter for all accepting states
    pub fn empty_accept_states(&mut self) {
        for state in &self.accepting {
            // Create a transition that decrements the counter by 1
            let decr_trans = Transition::new_basic_block_trans(*state, 0, -1);

            // Add that transition to the state
            let transitions = self.state_map.get_mut(state).unwrap();
            transitions.push(decr_trans);
        }
    }

    // Get all the transitions off of a state
    pub fn get_transitions(&self, state : State) -> Vec<Transition> {
        match self.state_map.get(&state) {
            None => panic!("State doesn't exist in automaton!"),

            Some(transitions) => transitions.to_vec(),
        }
    }
    
    // Find out if a state is accepting or rejecting
    // Some(true) if accepting, Some(false) if rejecting
    pub fn check_if_halting(&self, state : State) -> Option<bool> {
        if self.accepting.contains(&state) {
            Some(true)
        } else if self.rejecting.contains(&state) {
            Some(false)
        } else {
            None
        }
    }

}
