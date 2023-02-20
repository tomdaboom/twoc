#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Arith<A> {
    Var(A),

    Zero(),

    Succ(Box<Arith<A>>),

    Plus(Box<Arith<A>>, Box<Arith<A>>),

    Minus(Box<Arith<A>>, Box<Arith<A>>),    
}

pub fn substitute_arith<A : Eq + PartialEq + Clone>(formula : Arith<A>, from : Arith<A>, to : Arith<A>) -> Arith<A> {
    if formula.clone() == from { return to; }

    match formula {
        Arith::Var(x) => Arith::Var(x),

        Arith::Zero() => Arith::Zero(),

        Arith::Succ(x) => {
            let x_sub = substitute_arith(*x, from.clone(), to.clone());

            Arith::Succ(Box::new(x_sub))
        },

        Arith::Plus(x, y) => {
            let x_sub = substitute_arith(*x, from.clone(), to.clone());
            let y_sub = substitute_arith(*y, from.clone(), to.clone());

            Arith::Plus(Box::new(x_sub), Box::new(y_sub))
        },

        Arith::Minus(x, y) => {
            let x_sub = substitute_arith(*x, from.clone(), to.clone());
            let y_sub = substitute_arith(*y, from.clone(), to.clone());

            Arith::Minus(Box::new(x_sub), Box::new(y_sub))
        },
    }
}