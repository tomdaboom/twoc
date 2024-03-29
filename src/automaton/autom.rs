// Struct to store constructed 2nc

use crate::parser::ast::Cond;
use crate::automaton::generic_autom::{State, TransitionTrait, GenericAutom};

// Deterministic transitions (conditions are contained as part of these transitions)
#[derive(Debug, Clone)]
pub struct Transition {
    // The state we transition to
    pub goto : State,
    
    // Increment by
    pub incr_by : i32,

    // Move by
    pub move_by : i32,
    
    // Any conditionals required to take this transition if they exist
    pub condition : Option<Cond>,
}


impl TransitionTrait for Transition {
    // Construct a new transition that corresponds to a basic block
    fn new_basic_block_trans(next_state : State, mv : i32, ic : i32) -> Self {
        Self {
            goto : next_state,
            
            move_by : mv,
            incr_by : ic,
            
            condition : None,
        }
    }

    // Construct an epsilon transition
    fn new_epsilon_trans(next_state : State) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            condition : None,
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

        // Print the condition
        if let Some(cond) = &self.condition {
            print!(" if ");
            cond.print();
        };
    }
}

impl Transition {
    // Construct a transition that checks a condition
    pub fn new_cond_trans(next_state : State, cond : Cond) -> Self {
        Self {
            goto : next_state,

            move_by : 0,
            incr_by : 0,

            condition : Some(cond),
        }
    }
}

// A deterministic automaton is a generic automaton with deterministic transitions
pub type Autom = GenericAutom<Transition>;