// A struct for desugared programs

use std::collections::HashSet;

use crate::parser::{ast, contract::contract};

pub struct Program {
    pub stmts : Vec<ast::Stmt>,
    pub alpha : HashSet<char>,
    pub decr_zero : bool,
}

impl Program {
    // Construct a Program given an AST and an alphabet
    pub fn new(prog : Vec<ast::Stmt>, char_list : Vec<char>, decr_zero : bool) -> Self {
        // Convert the alphabet from a vector to a HashSet
        let mut char_set = HashSet::new();
        for char in char_list {
            char_set.insert(char);
        }

        // Construct the Program object
        Self { stmts : prog, alpha : char_set, decr_zero }
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
                // Return false if we find a branch or a while-choose statement
                ast::Stmt::Branch(_)      => return false,
                ast::Stmt::WhileChoose(_) => return false,

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

        // Return true if no branch statements were found
        true
    }

    // Check if every character in a given input string is also in the program's alphabet
    pub fn check_if_input_in_alphabet(&self, input_string : &str) -> bool {
        for char in input_string.chars() {
            if !self.alpha.contains(&char) {
                return false;
            }
        }
        true
    }
}

