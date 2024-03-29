// Construction algorithm for nondeterministic programs

use crate::parser::{program::Program, ast};

use crate::automaton::autom::{Autom, Transition};
use crate::automaton::generic_autom::{State, TransitionTrait};

// New add_transition function for GenericAutom<Transition> that ensures 
// all transitions either decrement or increment the counter
impl Autom {
    pub fn add_transition_pop_push(&mut self, source : State, trans : Transition) {
        // If the transition already pushes or pops, don't bother with an intermediary state
        if trans.incr_by != 0 {
            self.add_transition(source, trans);
            return;
        }

        // Introduce intermediary state
        let intermediary = self.introduce();

        // Create push trans
        let mut push_trans = trans.clone();
        push_trans.incr_by = 1;
        push_trans.goto = intermediary;

        // Create pop trans
        let pop_trans = Transition::new_basic_block_trans(trans.goto, 0, -1);

        // Find the source state in the adjacency list
        let search_map = self.state_map.get_mut(&source);
        
        // Push the push transition to the adjacency list or panic
        match search_map {
            Some(trans_vec) => trans_vec.push(push_trans),

            None => panic!("State {} doesn't exist in the automaton!", source),
        }

        // Find the intermediary state in the adjacency list (we just added it so it has to exist)
        let trans_vec = self.state_map.get_mut(&intermediary).unwrap();
        trans_vec.push(pop_trans);
    }
}

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
            let mut new_state = *state;

            if move_by != 0 {
                new_state = autom.introduce();

                // Create a new transition that executes the move instruction
                let move_transition = Transition::new_basic_block_trans(
                    new_state, 
                    move_by, 
                    0
                );

                // Add the transition to the automaton
                autom.add_transition_pop_push(*state, move_transition);
            }

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
                autom.add_transition_pop_push(last_state, incr_transition);
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
            autom.add_transition_pop_push(*state, true_trans);

            // Construct the statements in the if branch
            for true_stmt in if_body {
                construct_stmt(autom, &mut true_state, true_stmt);
            }

            // Create transition to check the condition of the else branch
            let false_trans = Transition::new_cond_trans(false_state, neg_cond);
            autom.add_transition_pop_push(*state, false_trans);

            // Construct the statements in the else branch
            for false_stmt in else_body {
                construct_stmt(autom, &mut false_state, false_stmt);
            }

            // Add epsilon transitions from each of the blocks to the final state
            autom.add_transition_pop_push(true_state, Transition::new_epsilon_trans(final_state));
            autom.add_transition_pop_push(false_state, Transition::new_epsilon_trans(final_state));

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
            autom.add_transition_pop_push(*state, while_trans);

            // Construct the statements in the while block
            for while_stmt in while_body {
                construct_stmt(autom, &mut while_state, while_stmt);
            }

            // Add an epsilon transition back to the start state
            let restart_transition = Transition::new_epsilon_trans(*state);
            autom.add_transition_pop_push(while_state, restart_transition);

            // Construct transitions to check whether or not to break out of the while block 
            let break_trans = Transition::new_cond_trans(break_state, neg_cond);
            autom.add_transition_pop_push(*state, break_trans);

            // Update the current state to the state reached after breaking out of the loop
            *state = break_state;  
        },
        
        ast::Stmt::Branch(branches) => {
            // Introduce a common final state for each of the branches
            let final_state = autom.introduce();

            for branch in branches {
                // Introduce a new start state for each of the branches
                let new_state = autom.introduce();

                // Add an epsilon transition from current state to new start state
                let start_transition = Transition::new_epsilon_trans(new_state);
                autom.add_transition_pop_push(*state, start_transition);

                // Construct each of the statements in the branch
                let mut branch_state = new_state;
                for branch_stmt in branch {
                    construct_stmt(autom, &mut branch_state, branch_stmt);
                }

                // Construct an epsilon transition from the final state of this branch to the common final state
                let end_transition = Transition::new_epsilon_trans(final_state);
                autom.add_transition_pop_push(branch_state, end_transition);
            }

            // Update the current state to the common final state
            *state = final_state;
        },


        ast::Stmt::WhileChoose(while_body) => {
            // Variables to keep track of state in the while block and after breaking out
            let mut while_state = autom.introduce();
            let break_state = autom.introduce();
            
            // Construct the transition to enter the while statement
            let entry_transition = Transition::new_epsilon_trans(while_state);
            autom.add_transition_pop_push(*state, entry_transition);

            // Construct the statements in the while block
            for while_stmt in while_body {
                construct_stmt(autom, &mut while_state, while_stmt);
            }

            // Add an epsilon transition back to the start state
            let restart_transition = Transition::new_epsilon_trans(*state);
            autom.add_transition_pop_push(while_state, restart_transition);

            // Construct the condition for breaking out of the while statement
            let exit_transition = Transition::new_epsilon_trans(break_state);
            autom.add_transition_pop_push(*state, exit_transition);

            // Update the current state to the state reached after breaking out of the loop
            *state = break_state;  
        },

        _ => panic!("Move or Incr statement in deterministic program!"),
    }
}