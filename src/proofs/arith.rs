#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Arith {
    Var(String),

    Zero(),

    Succ(Box<Arith>),

    Plus(Box<Arith>, Box<Arith>),

    Minus(Box<Arith>, Box<Arith>),    
}