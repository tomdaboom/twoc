use crate::parser::{program::Program, ast};

use crate::automaton::determ_autom::{Autom, Transition};
use crate::automaton::generic_autom::{State, TransitionTrait};

// Turn a program into an automaton
pub fn construct_from_prog(prog : Program) -> Autom {
    // Initialise the automaton
    let mut autom = Autom::new(prog.alpha, prog.decr_zero);

    // Introduce a start state and record it
    let mut state = autom.introduce();

    // Construct the statements in the program
    for stmt in prog.stmts {
        construct_stmt(&mut autom, &mut state, stmt);
    }

    // Remove transitions from all halting states
    autom.clean_halting_states();

    // Add counter-emptying transitions to the automaton
    autom.empty_accept_states();

    // Return the constructed automaton
    autom
}

// Introduce the neccesary states and transitions to represent a single statement
fn construct_stmt(autom : &mut Autom, state : &mut State, stmt : ast::Stmt) {
    match stmt {
        // Turn the current state into an accept/reject state
        ast::Stmt::Accept() => autom.make_accept_state(*state),
        ast::Stmt::Reject() => autom.make_reject_state(*state),

        // Add a new state/transition for a basic block
        ast::Stmt::BasicBlock(move_by, incr_by) => {
            // Make a new state
            let mut new_state = autom.introduce();

            // Create a new transition that executes the move instruction
            let move_transition = Transition::new_basic_block_trans(
                new_state, 
                move_by, 
                0
            );

            // Add the transition to the automaton
            autom.add_transition(*state, move_transition);

            // Add transitions to increment/decrement the counter
            
            // Variable to store the last state introduced to the automaton in the for loop
            let mut last_state;

            for _ in 0..incr_by.abs() {
                // Update last_state and introduce a new state to the automaton
                last_state = new_state;
                new_state = autom.introduce();

                // Create a transition that changes the counter
                let incr_transition = Transition::new_basic_block_trans(
                    new_state, 
                    0, 
                    if incr_by > 0 {1} else {-1}
                );

                // Add the transition to the automaton
                autom.add_transition(last_state, incr_transition);
            } 

            // Update the current state to the new state
            *state = new_state;
        },

        // Recursively construct an if statement 
        ast::Stmt::If(cond, if_body, else_body) => {
            // Introduce states for true and false branches
            let mut true_state  = autom.introduce();
            let mut false_state = autom.introduce();

            // Introduce a common final state
            let final_state = autom.introduce();

            // Construct the negation of cond
            let neg_cond = ast::Cond::Not(Box::new(cond.clone()));

            // Create transition to check the condition of the if branch
            let true_trans = Transition::new_cond_trans(true_state, cond);
            autom.add_transition(*state, true_trans);

            // Construct the statements in the if branch
            for true_stmt in if_body {
                construct_stmt(autom, &mut true_state, true_stmt);
            }

            // Create transition to check the condition of the else branch
            let false_trans = Transition::new_cond_trans(false_state, neg_cond);
            autom.add_transition(*state, false_trans);

            // Construct the statements in the else branch
            for false_stmt in else_body {
                construct_stmt(autom, &mut false_state, false_stmt);
            }

            // Add epsilon transitions from each of the blocks to the final state
            autom.add_transition(true_state, Transition::new_epsilon_trans(final_state));
            autom.add_transition(false_state, Transition::new_epsilon_trans(final_state));

            // Set the current state to the final state
            *state = final_state;
        },

        // Recursively construct a while statement
        ast::Stmt::While(cond, while_body) => {
            // Variables to keep track of state in the while block and after breaking out
            let mut while_state = autom.introduce();
            let     break_state = autom.introduce();

            // Construct the negation of cond
            let neg_cond = ast::Cond::Not(Box::new(cond.clone()));
            
            // Construct transitions to check whether or not to enter the while block 
            let while_trans = Transition::new_cond_trans(while_state, cond);
            autom.add_transition(*state, while_trans);

            // Construct the statements in the while block
            for while_stmt in while_body {
                construct_stmt(autom, &mut while_state, while_stmt);
            }

            // Add an epsilon transition back to the start state
            let restart_transition = Transition::new_epsilon_trans(*state);
            autom.add_transition(while_state, restart_transition);

            // Construct transitions to check whether or not to break out of the while block 
            let break_trans = Transition::new_cond_trans(break_state, neg_cond);
            autom.add_transition(*state, break_trans);

            // Update the current state to the state reached after breaking out of the loop
            *state = break_state;  
        },

        _ => panic!("Branch or while-choose statement in deterministic program!"),
    }
}