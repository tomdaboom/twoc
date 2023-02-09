use std::collections::HashMap;
use crate::parser::ast::{Cond, Readable};

type SugarProg = crate::parser::sugar::program::Program;
type SugarStmt = crate::parser::sugar::ast::Stmt;

type Prog = crate::parser::program::Program;
type Stmt = crate::parser::ast::Stmt;     

fn convert_sugar(in_prog : SugarProg) -> Prog {
    // Get alphabet
    let alpha = in_prog.alpha.clone();

    // Convert old program
    let mut stmts = Vec::new();
    for stmt in in_prog.stmts {
        stmts.append(&mut convert_statement(stmt, &in_prog.parmap));
    }

    // Construct new program
    Prog { stmts, alpha : alpha.clone() }
}

fn convert_statement(sugar : SugarStmt, parmap : &HashMap<String, char>) -> Vec<Stmt> {
    match sugar {
        SugarStmt::Accept() => vec![Stmt::Accept()],

        SugarStmt::Reject() => vec![Stmt::Reject()],

        SugarStmt::Move(i) => vec![Stmt::Move(i)],

        SugarStmt::Incr(incr) => match incr {
            super::ast::Value::Lit(j) => vec![Stmt::Incr(j)],

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
                    Cond::NotRead(Readable::Char(*c)),
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
                    Cond::NotRead(Readable::Char(*c)),
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

        SugarStmt::Asgn(incr) => match incr {
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
                    Cond::NotRead(Readable::Char(*c)),
                    vec![Stmt::Move(1)],
                );

                // Increment counter
                let load_from_char = Stmt::While(
                    Cond::Read(Readable::Char(*c)),
                    vec![Stmt::Move(1), Stmt::Incr(1)],
                );
 
                vec![empty, move_to_lend.clone(), move_to_char, load_from_char, move_to_lend]
            },

            super::ast::Value::NegPar(par) => 
                panic!("{}", &format!("Counter can't contain negative value -{:?}!", par)),
        },
        
        SugarStmt::If(cond, if_block, else_block) => {
            let mut converted_if = Vec::new();
            let mut converted_else = Vec::new();

            for stmt in if_block {
                converted_if.append(&mut convert_statement(stmt, parmap));
            }

            for stmt in else_block {
                converted_else.append(&mut convert_statement(stmt, parmap));
            }

            vec![Stmt::If(cond, converted_if, converted_else)]
        },

        SugarStmt::While(cond, while_block) => {
            let mut converted_while = Vec::new();

            for stmt in while_block {
                converted_while.append(&mut convert_statement(stmt, parmap));
            }

            vec![Stmt::While(cond, converted_while)]
        },

        SugarStmt::Branch(branches) => {
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
    }
}