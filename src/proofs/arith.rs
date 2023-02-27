#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Arith<A> {
    Var(A),

    Zero(),

    Succ(Box<Arith<A>>),

    Plus(Box<Arith<A>>, Box<Arith<A>>),

    Mult(Box<Arith<A>>, Box<Arith<A>>),    
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

        Arith::Mult(x, y) => {
            let x_sub = substitute_arith(*x, from.clone(), to.clone());
            let y_sub = substitute_arith(*y, from.clone(), to.clone());

            Arith::Mult(Box::new(x_sub), Box::new(y_sub))
        },
    }
}

pub fn get_arith_vars<A : Eq + PartialEq + Clone>(formula : Arith<A>) -> Vec<A> {
    let mut vars = match formula {
        Arith::Var(v) => vec![v],

        Arith::Succ(x) => get_arith_vars(*x),

        Arith::Plus(left, right) => {
            let mut vars = get_arith_vars(*left);
            vars.append(&mut get_arith_vars(*right));
            vars
        },

        Arith::Mult(left, right) => {
            let mut vars = get_arith_vars(*left);
            vars.append(&mut get_arith_vars(*right));
            vars
        },

        Arith::Zero() => vec![],
    };

    vars.dedup();
    vars
}