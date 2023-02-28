use std::collections::HashMap;
use crate::parser::ast::{Cond, Readable};

// Type aliases for sugared programs
type SugarProg = crate::parser::sugar::program::Program;
type SugarStmt = crate::parser::sugar::ast::Stmt;

// Type aliases for unsugared programs
type Prog = crate::parser::program::Program;
type Stmt = crate::parser::ast::Stmt;     

// Convert all the macros in a given sugared program
pub fn convert_sugar(in_prog : SugarProg) -> Prog {
    // Get alphabet
    let alpha = in_prog.alpha.clone();

    // Declare vector to hold new program
    let mut stmts = Vec::new();

    // Insert code to check that the program satisfies alpha[0]* + alpha[1]* + ... 
    if in_prog.pars.len() > 0 {
        // Find the order in which characters should show up
        let mut char_order = Vec::new();
        for p in in_prog.pars.into_iter().rev() {
            // Get the character that the param is looking for
            let c = in_prog.parmap.get(&p).unwrap();
            char_order.push(c);
        }

        // Move off the left endmarker
        stmts.push(Stmt::Move(1));

        for c in char_order {        
            // Read each occurence of c
            stmts.push(Stmt::While(
                Cond::Read(Readable::Char(*c)),
                vec![Stmt::Move(1)],
            ));
        }

        // Check that we're at rend
        stmts.push(Stmt::If(
            Cond::NotRead(Readable::REnd()),
            vec![Stmt::Reject()],
            vec![],
        ));

        // Return to lend
        stmts.push(Stmt::While(
            Cond::NotRead(Readable::LEnd()), 
            vec![Stmt::Move(-1)],            
        ));
    }
    
    // Convert old program
    for stmt in in_prog.stmts {
        stmts.append(&mut convert_statement(stmt, &in_prog.parmap));
    }

    Prog { stmts, alpha : alpha.clone(), decr_zero : in_prog.decr_zero }
}

// Convert a single statement into it's desugared equivalent
fn convert_statement(sugar : SugarStmt, parmap : &HashMap<String, char>) -> Vec<Stmt> {
    match sugar {
        // Accept, reject and move statements don't need any fancy logic
        SugarStmt::Accept() => vec![Stmt::Accept()],
        SugarStmt::Reject() => vec![Stmt::Reject()],
        SugarStmt::Move(i) => vec![Stmt::Move(i)],

        // Increments
        SugarStmt::Incr(incr) => match incr {
            // Incr statements on literals don't need fancy logic either
            super::ast::Value::Lit(j) => vec![Stmt::Incr(j)],

            // Incr statements on parameters do

            super::ast::Value::Par(par) => {
                // Get alphabet character
                let c = parmap.get(&par).expect(&format!("Parameter {:?} Undeclared!", par));
                
                // Move to left endmarker
                let move_to_lend = Stmt::While(
                    Cond::NotRead(Readable::LEnd()), // while (read != lend)
                    vec![Stmt::Move(-1)],            //     move(-1)
                );

                // Move to character index
                let move_to_char = Stmt::While(
                    Cond::And(
                        Box::new(Cond::NotRead(Readable::Char(*c))), 
                        Box::new(Cond::NotRead(Readable::REnd()))
                    ),
                    vec![Stmt::Move(1)],
                );

                // Increment counter
                let load_from_char = Stmt::While(
                    Cond::Read(Readable::Char(*c)),
                    vec![Stmt::Move(1), Stmt::Incr(1)],
                );

                vec![move_to_lend.clone(), move_to_char, load_from_char, move_to_lend]
            },

            super::ast::Value::NegPar(par) => {
                // Get alphabet character
                let c = parmap.get(&par).expect(&format!("Parameter {:?} Undeclared!", par));
                
                // Move to left endmarker
                let move_to_lend = Stmt::While(
                    Cond::NotRead(Readable::LEnd()), // while (read != lend)
                    vec![Stmt::Move(-1)],            //     move(-1)
                );

                // Move to character index
                let move_to_char = Stmt::While(
                    Cond::And(
                        Box::new(Cond::NotRead(Readable::Char(*c))), 
                        Box::new(Cond::NotRead(Readable::REnd()))
                    ),
                    vec![Stmt::Move(1)],
                );

                // Decrement counter
                let load_from_char = Stmt::While(
                    Cond::Read(Readable::Char(*c)),
                    vec![Stmt::Move(1), Stmt::Incr(-1)],
                );

                vec![move_to_lend.clone(), move_to_char, load_from_char, move_to_lend]
            },
        },

        // Counter assignments
        SugarStmt::Asgn(incr) => match incr {
            // Assigning to literal values 
            super::ast::Value::Lit(j) => {
                // Panic if negative
                if j < 0 {
                    panic!("{}", &format!("Counter can't contain negative value {:?}!", j));
                }

                // Empty counter
                let empty = Stmt::While(
                    Cond::CheckNotZero(),
                    vec![Stmt::Incr(-1)],
                );

                // Increment counter
                let incr = Stmt::Incr(j);

                vec![empty, incr]
            },

            // Assigning to parameters
            super::ast::Value::Par(par) => {
                // Get alphabet character
                let c = parmap.get(&par).expect(&format!("Parameter {:?} Undeclared!", par));

                // Empty counter
                let empty = Stmt::While(
                    Cond::CheckNotZero(),
                    vec![Stmt::Incr(-1)],
                );

                // Move to left endmarker
                let move_to_lend = Stmt::While(
                    Cond::NotRead(Readable::LEnd()), // while (read != lend)
                    vec![Stmt::Move(-1)],            //     move(-1)
                );

                // Move to character index
                let move_to_char = Stmt::While(
                    Cond::And(
                        Box::new(Cond::NotRead(Readable::Char(*c))), 
                        Box::new(Cond::NotRead(Readable::REnd()))
                    ),
                    vec![Stmt::Move(1)],
                );

                // Increment counter
                let load_from_char = Stmt::While(
                    Cond::Read(Readable::Char(*c)),
                    vec![Stmt::Move(1), Stmt::Incr(1)],
                );
 
                vec![empty, move_to_lend.clone(), move_to_char, load_from_char, move_to_lend]
            },

            // Counter can't be negative
            super::ast::Value::NegPar(par) => 
                panic!("{}", &format!("Counter can't contain negative value -{:?}!", par)),
        },
        
        // If statements
        SugarStmt::If(cond, if_block, else_block) => {
            // Recursively convert if block
            let mut converted_if = Vec::new();
            for stmt in if_block {
                converted_if.append(&mut convert_statement(stmt, parmap));
            }

            // Recursively convert else block
            let mut converted_else = Vec::new();
            for stmt in else_block {
                converted_else.append(&mut convert_statement(stmt, parmap));
            }

            vec![Stmt::If(cond, converted_if, converted_else)]
        },

        SugarStmt::While(cond, while_block) => {
            // Recursively convert while block
            let mut converted_while = Vec::new();
            for stmt in while_block {
                converted_while.append(&mut convert_statement(stmt, parmap));
            }

            vec![Stmt::While(cond, converted_while)]
        },

        SugarStmt::Branch(branches) => {
            // Recursively convert each branch

            let mut converted_branches = Vec::new();

            for branch in branches {
                let mut converted_branch = Vec::new();
                for stmt in branch {
                    converted_branch.append(&mut convert_statement(stmt, parmap));
                }

                converted_branches.push(converted_branch);
            }

            vec![Stmt::Branch(converted_branches)]
        },

        SugarStmt::Repeat(k, block) => {
            // Recursively convert the block's contents
            let mut converted_block = Vec::new();
            for stmt in block {
                converted_block.append(&mut convert_statement(stmt, parmap));
            }

            // Repeat the converted contents k times
            let mut repeated_block = Vec::new();
            for _ in 0..k {
                for stmt in &converted_block {
                    repeated_block.push(stmt.clone());
                }
            }

            repeated_block
        },

        // Comments should do nothing 
        // in fact, they probably shouldn't be in the AST in the first place, 
        // but I'm too lazy to write my own lexer
        SugarStmt::Comment() => Vec::new(),
    }
}