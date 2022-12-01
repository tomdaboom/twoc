// AST nodes for statements
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
pub enum Cond {
    Read(char),

    CheckZero(),

    CheckNotZero(),

    And(Cond, Cond),

    Or(Cond, Cond),

    Not(Cond),
}

