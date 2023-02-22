use std::collections::{HashSet, HashMap};
use std::iter::zip;

use crate::parser::sugar::ast;

pub struct Program {
    pub stmts  : Vec<ast::Stmt>,
    pub alpha  : HashSet<char>,
    pub pars   : Vec<String>,
    pub parmap : HashMap<String, char>,
    pub decr_zero : bool,
}

impl Program {
    // Construct a Program given an AST and an alphabet
    pub fn new(prog : Vec<ast::Stmt>, char_list : Vec<char>, par_list : Vec<String>, decr_zero : bool) -> Self {
        // Convert the alphabet from a vector to a HashSet
        let mut char_set = HashSet::new();
        for char in char_list.clone() {
            char_set.insert(char);
        }

        // Check that the alphabet and parameter list are of the same size
        if par_list.len() != char_set.len() {
            panic!("Different number of parameters to characters in alphabet!");
        }

        let mut map = HashMap::new();

        // Construct the parmap
        for (p, c) in zip(par_list.clone(), char_list) {
            map.insert(p, c);
        }

        // Construct the Program object
        Self { stmts : prog, alpha : char_set, pars : par_list, parmap : map, decr_zero }
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