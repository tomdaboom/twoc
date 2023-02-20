use crate::proofs::arith::Arith;
use crate::proofs::prop_calc::PropFormula;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fol<A> {
    Eq(Box<Arith<A>>, Box<Arith<A>>),

    ForAll(A, Box<PropFormula<Fol<A>>>),

    Exists(A, Box<PropFormula<Fol<A>>>),
}

mod rules {
    use super::Fol;

    use crate::proofs::arith::{Arith, substitute_arith};
    use crate::proofs::prop_calc::PropFormula;

    // substitution
    pub fn substitute<A : Eq + PartialEq + Clone>(proof : PropFormula<Fol<A>>, from : Arith<A>, to : Arith<A>) -> PropFormula<Fol<A>> {
        match proof {
            PropFormula::Var(var) => {
                match var {
                    Fol::Eq(lhs, rhs) => {
                        let new_lhs = substitute_arith(*lhs, from.clone(), to.clone());
                        let new_rhs = substitute_arith(*rhs, from, to);

                        PropFormula::Var(Fol::Eq(Box::new(new_lhs), Box::new(new_rhs)))
                    },

                    Fol::ForAll(par, inner) => {
                        let new_inner = substitute(*inner, from, to);

                        PropFormula::Var(Fol::ForAll(par, Box::new(new_inner)))
                    },

                    Fol::Exists(par, inner) => {
                        let new_inner = substitute(*inner, from, to);

                        PropFormula::Var(Fol::Exists(par, Box::new(new_inner)))
                    },
                }
            },

            PropFormula::Not(x) => {
                let new_x = substitute(*x, from, to);

                PropFormula::Not(Box::new(new_x))
            },

            PropFormula::And(x, y) => {
                let new_x = substitute(*x, from.clone(), to.clone());
                let new_y = substitute(*y, from, to);

                PropFormula::And(Box::new(new_x), Box::new(new_y))
            },

            PropFormula::Or(x, y) => {
                let new_x = substitute(*x, from.clone(), to.clone());
                let new_y = substitute(*y, from, to);

                PropFormula::Or(Box::new(new_x), Box::new(new_y))
            },

            PropFormula::Imp(x, y) => {
                let new_x = substitute(*x, from.clone(), to.clone());
                let new_y = substitute(*y, from, to);

                PropFormula::Imp(Box::new(new_x), Box::new(new_y))
            },
        }
    }

    // specification

    // generalisation

    // interchange

    // existence

    // symmetry

    // transitivity

    // add s

    // drop s

    // induction
}