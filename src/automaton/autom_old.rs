use crate::parser::ast::Readable;
use crate::automaton::generic_autom::{State, TransitionTrait, GenericAutom};

// Potentially nondeterministic transitions
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


impl TransitionTrait for Transition {
    // Construct a new transition that corresponds to a basic block
    fn new_basic_block_trans(next_state : State, mv : i32, ic : i32) -> Self {
        Self {
            goto : next_state,
            
            move_by : mv,
            incr_by : ic,
            
            test_counter_zero : None,
            read_char : None,
        }
    }

    // Construct an epsilon transition
    fn new_epsilon_trans(next_state : State) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            test_counter_zero : None,
            read_char : None,
        }
    }

    // Display the transition
    fn print(&self) {
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
            read.print();
        }
    }
}

impl Transition {
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
}

// A nondeterministic automaton is an automaton with potentially nondeterministic transitions
pub type Autom = GenericAutom<Transition>;
