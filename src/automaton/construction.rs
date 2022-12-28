use crate::parser::{program::Program, ast};

use super::autom::{Autom, State, Transition};

fn construct_from_prog(prog : Program) -> Autom {
    // Initialise the automaton
    let mut autom = Autom::new();

    // Introduce a start state and record it
    let mut state = autom.introduce();

    

    autom
}

fn construct_stmt(autom : &mut Autom, state : &mut State, stmt : ast::Stmt) {
    match stmt {
        // Turn the current state into an accept/reject state
        ast::Stmt::Accept() => autom.make_accept_state(*state),
        ast::Stmt::Reject() => autom.make_reject_state(*state),

        // Add a new state/transition for a basic block
        ast::Stmt::BasicBlock(move_by, incr_by) => {
            // Make a new state
            let new_state = autom.introduce();

            // Create a new transition
            let transition = Transition::new_basic_block_trans(
                new_state, 
                move_by, 
                incr_by
            );

            // Add the transition to the automaton
            autom.add_transition(*state, transition);

            // Update the current state to the new state
            *state = new_state;
        },

        ast::Stmt::If(cond, if_body, else_body) => {
            
        },

        ast::Stmt::While(cond, while_body) => {

        },

        ast::Stmt::Branch(branches) => {

        },

        _ => panic!("Can't construct this type of statement yet!"),
    }
}