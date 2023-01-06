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
            // Introduce a common final state for each of the branches
            let final_state = autom.introduce();

            for branch in branches {
                // Introduce a new start state for each of the branches
                let new_state = autom.introduce();

                // Add an epsilon transition from current state to new start state
                let start_transition = Transition::new_epsilon_trans(new_state);
                autom.add_transition(*state, start_transition);

                // Construct each of the statements in the branch
                let mut branch_state = new_state;
                for branch_stmt in branch {
                    construct_stmt(autom, &mut branch_state, branch_stmt);
                }

                // Construct an epsilon transition from the final state of this branch to the common final state
                let end_transition = Transition::new_epsilon_trans(final_state);
                autom.add_transition(branch_state, end_transition);
            }

            // Update the current state to the common final state
            *state = final_state;
        },

        _ => panic!("Can't construct this type of statement yet!"),
    }
}

fn construct_conditional_transitions(autom : &mut Autom, state : &mut State, conditional : ast::Cond) {
    match conditional {
        ast::Cond::Read(char) => {

        },

        ast::Cond::NotRead(char) => {

        },

        ast::Cond::CheckZero() => {

        },

        ast::Cond::CheckNotZero() => {

        },

        ast::Cond::And(left, right) => {

        },

        ast::Cond::Or(left, right) => {

        },

        ast::Cond::Not(stmt) => {

        },
    }
}