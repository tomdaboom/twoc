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
}

