use crate::ast;
use crate::contract;

pub struct Program {
    pub stmts : Vec<ast::Stmt>,
}

impl Program {
    pub fn new(prog : Vec<ast::Stmt> ) -> Self {
        Self { stmts : prog }
    }

    pub fn contract(&mut self) {
        self.stmts = contract::contract(&self.stmts);
    } 

    pub fn print(&self) {
        for stmt in &self.stmts {
            print!("{}", stmt.print(2));
        }
    }
}

