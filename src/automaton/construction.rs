use crate::parser::{program::Program, ast};

use super::autom::{Autom, State, Transition};

pub fn construct_from_prog(prog : Program) -> Autom {
    // Initialise the automaton
    let mut autom = Autom::new(prog.alpha);

    // Introduce a start state and record it
    let mut state = autom.introduce();

    // Construct the statements in the program
    for stmt in prog.stmts {
        construct_stmt(&mut autom, &mut state, stmt);
    }

    // Return the constructed automaton
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

            // Create a new transition that executes the basic block
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
            // Variables to keep track of current states for true and false branches
            let mut true_state  = *state;
            let mut false_state = *state;

            // Introduce a common final state
            let final_state = autom.introduce();

            // Construct the negation of cond
            let neg_cond = ast::Cond::Not(Box::new(cond.clone()));

            // Construct the condition for the if branch
            construct_conditional_transitions(autom, &mut true_state, cond);

            // Construct the statements in the if branch
            for true_stmt in if_body {
                construct_stmt(autom, &mut true_state, true_stmt);
            }

            // Construct the condition for the false statement
            construct_conditional_transitions(autom, &mut false_state, neg_cond);

            // Construct the statements in the true branch
            for false_stmt in else_body {
                construct_stmt(autom, &mut false_state, false_stmt);
            }

            // Add epsilon transitions from each of the blocks to the final state
            let transition = Transition::new_epsilon_trans(final_state);
            autom.add_transition(true_state, transition);
            autom.add_transition(false_state, transition);

            // Set the current state to the final state
            *state = final_state;
        },

        ast::Stmt::While(cond, while_body) => {
            // Variables to keep track of state in the while block and after breaking out
            let mut while_state = *state;
            let mut break_state = *state;

            // Construct the negation of cond
            let neg_cond = ast::Cond::Not(Box::new(cond.clone()));
            
            // Construct the condition for entering the while statement
            construct_conditional_transitions(autom, &mut while_state, cond);

            // Construct the statements in the while block
            for while_stmt in while_body {
                construct_stmt(autom, &mut while_state, while_stmt);
            }

            // Add an epsilon transition back to the start state
            let restart_transition = Transition::new_epsilon_trans(*state);
            autom.add_transition(while_state, restart_transition);

            // Construct the condition for breaking out of the while statement
            construct_conditional_transitions(autom, &mut break_state, neg_cond);

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
            // Introduce a new state
            let new_state = autom.introduce();

            // Construct a new transition from the current state that checks for the given character
            let transition = Transition::new_read_trans(new_state, char);
            autom.add_transition(*state, transition);

            // Update the current state to the new state
            *state = new_state;
        },

        ast::Cond::NotRead(char) => {
            // Introduce a new state
            let new_state = autom.introduce();

            // Create and store transitions for all of the characters not being read
            let mut transes : Vec<Transition> = Vec::new();

            for other_char in autom.alpha.iter() {
                // Convert the alphabet character to a Readable
                let readable_char = ast::Readable::Char(*other_char);

                // Skip recording this transition if it's the character we don't want to read
                if readable_char == char { continue; }
                
                // Store this transition
                transes.push(Transition::new_read_trans(
                    new_state, 
                    readable_char)
                );
            }

            // Add new transitions to the automaton
            for transition in transes {
                autom.add_transition(*state, transition);
            }

            // Add transitions to check for the end markers

            if char != ast::Readable::LEnd() {
                let transition = Transition::new_read_trans(
                    new_state,
                    ast::Readable::LEnd()
                );
                
                autom.add_transition(*state, transition);
            }

            if char != ast::Readable::REnd() {
                let transition = Transition::new_read_trans(
                    new_state,
                    ast::Readable::LEnd()
                );
                
                autom.add_transition(*state, transition);
            }

            *state = new_state;
        },

        ast::Cond::CheckZero() => {
            // Introduce a new state
            let new_state = autom.introduce();

            // Construct a new transition from the current state that checks if the counter is zero
            let transition = Transition::new_checkzero_trans(new_state, true);
            autom.add_transition(*state, transition);

            // Update the current state to the new state
            *state = new_state;
        },

        ast::Cond::CheckNotZero() => {
            // Introduce a new state
            let new_state = autom.introduce();

            // Construct a new transition from the current state that checks if the counter isn't zero
            let transition = Transition::new_checkzero_trans(new_state, false);
            autom.add_transition(*state, transition);

            // Update the current state to the new state
            *state = new_state;
        },

        ast::Cond::And(left, right) => {
            // Construct transitions for left
            construct_conditional_transitions(autom, state, *left); 

            // Construct transitions for right off of the same state
            construct_conditional_transitions(autom, state, *right);
        },

        ast::Cond::Or(left, right) => {
            // Introduce new states to track where each of the transitions for left and right end up
            let mut left_final_state = *state;
            let mut right_final_state = *state;

            // Introduce a common final state
            let final_state = autom.introduce();

            // Construct transitions for left and right
            construct_conditional_transitions(autom, &mut left_final_state, *left);
            construct_conditional_transitions(autom, &mut right_final_state, *right);

            // Add epsilon transitions from each of the unique final states to the common final state
            let transition = Transition::new_epsilon_trans(final_state);
            autom.add_transition(left_final_state, transition);
            autom.add_transition(right_final_state, transition);

        },

        ast::Cond::Not(stmt) => match *stmt {
            ast::Cond::Read(char) 
                => construct_conditional_transitions(autom, state, ast::Cond::NotRead(char)),

            ast::Cond::NotRead(char) 
                => construct_conditional_transitions(autom, state, ast::Cond::Read(char)),

            ast::Cond::CheckZero() 
                => construct_conditional_transitions(autom, state, ast::Cond::CheckNotZero()),

            ast::Cond::CheckNotZero() 
                => construct_conditional_transitions(autom, state, ast::Cond::CheckZero()),

            ast::Cond::And(left, right) => {
                // Make boxes with !left and !right in them
                let not_left = Box::new(ast::Cond::Not(left));
                let not_right = Box::new(ast::Cond::Not(right));

                // Make the new conditional statement
                let new_conditional = ast::Cond::Or(not_left, not_right);

                // Construct transitions for the new conditional
                construct_conditional_transitions(autom, state, new_conditional);
            },

            ast::Cond::Or(left, right) => {
                // Make boxes with !left and !right in them
                let not_left = Box::new(ast::Cond::Not(left));
                let not_right = Box::new(ast::Cond::Not(right));

                // Make the new conditional statement
                let new_conditional = ast::Cond::And(not_left, not_right);

                // Construct transitions for the new conditional
                construct_conditional_transitions(autom, state, new_conditional);
            },

            ast::Cond::Not(inner_stmt) 
                => construct_conditional_transitions(autom, state, *inner_stmt),
        },
    }
}