// AST nodes for statements

pub mod ast {

    #[derive(Debug)]
    pub enum Stmt {
        // move(i)
        Move(i32),

        // c += j
        Incr(i32), 

        // if-else
        If(Cond, Vec<Stmt>, Vec<Stmt>),

        // while
        While(Cond, Vec<Stmt>),

        // branch
        Branch(Vec<Vec<Stmt>>),
    }

    // AST nodes for conditions
    #[derive(Debug)]
    pub enum Cond {
        Read(char),

        CheckZero(),

        CheckNotZero(),

        And(Box<Cond>, Box<Cond>),

        Or(Box<Cond>, Box<Cond>),

        Not(Box<Cond>),
    }

}