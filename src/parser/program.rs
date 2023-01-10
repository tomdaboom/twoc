use std::collections::HashSet;

use crate::ast;
use crate::contract::contract;

pub struct Program {
    pub stmts : Vec<ast::Stmt>,
    pub alpha : HashSet<char>,
}

impl Program {
    // Construct a Program given an AST and an alphabet
    pub fn new(prog : Vec<ast::Stmt>, char_list : Vec<char>) -> Self {
        // Convert the alphabet from a vector to a HashSet
        let mut char_set = HashSet::new();
        for char in char_list {
            char_set.insert(char);
        }

        // Construct the Program object
        Self { stmts : prog, alpha : char_set }
    }

    // Contract the statements in the program
    pub fn contract(&mut self) {
        self.stmts = contract(&self.stmts);
    } 

    // Print out the program
    pub fn print(&self) {
        for stmt in &self.stmts {
            print!("{}", stmt.print(2));
        }
    }

    // Check if this program is deterministic
    pub fn deterministic(&self) -> bool {
        return Program::no_branches(self.stmts.clone());
    }

    // True if prog contains no branches, false otherwise
    fn no_branches(prog : Vec<ast::Stmt>) -> bool {
        for stmt in prog {
            match stmt {
                // Return false if we find a branch statement
                ast::Stmt::Branch(_) => return false,

                // Recursively check each of the branches of an if statement
                ast::Stmt::If(_, if_branch, else_branch) => { 
                    if !Program::no_branches(if_branch) || !Program::no_branches(else_branch) {
                        return false;
                    }
                    
                    continue;
                },

                // Recursively check a while statement
                ast::Stmt::While(_, while_branch) => {
                    if !Program::no_branches(while_branch) {
                        return false;
                    }

                    continue;
                },

                // Otherwise, continue
                _ => continue,
            }
        }

        true
    }
}

