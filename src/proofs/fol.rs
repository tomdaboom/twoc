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

    use crate::proofs::arith::{Arith, substitute_arith, get_arith_vars};
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

    // get all variables
    pub fn vars<A : Eq + PartialEq + Clone>(proof : PropFormula<Fol<A>>) -> Vec<A> {
        match proof {
            PropFormula::Var(x) => match x {
                Fol::ForAll(x, y) => {
                    let mut vars = bound_vars(*y);
                    vars.push(x);
                    vars
                }, 

                Fol::Exists(x, y) => {
                    let mut vars = bound_vars(*y);
                    vars.push(x);
                    vars
                },

                Fol::Eq(x, y) => {
                    let mut vars = get_arith_vars(*x);
                    vars.append(&mut get_arith_vars(*y));
                    vars
                },
            },

            PropFormula::Not(inner) => vars(*inner),

            PropFormula::And(x, y) => {
                let mut x_vars = vars(*x);
                x_vars.append(&mut vars(*y));
                x_vars
            },

            PropFormula::Or(x, y) => {
                let mut x_vars = vars(*x);
                x_vars.append(&mut vars(*y));
                x_vars
            },

            PropFormula::Imp(x, y) => {
                let mut x_vars = vars(*x);
                x_vars.append(&mut vars(*y));
                x_vars
            },
        }
    }

    // get bound variables
    pub fn bound_vars<A : Eq + PartialEq + Clone>(proof : PropFormula<Fol<A>>) -> Vec<A> {
        let mut vars = match proof {
            PropFormula::Var(v) => match v {
                Fol::ForAll(x, y) => {
                    let mut other_vars = bound_vars(*y);
                    other_vars.push(x);
                    other_vars
                }, 

                Fol::Exists(x, y) => {
                    let mut other_vars = bound_vars(*y);
                    other_vars.push(x);
                    other_vars
                },

                _ => vec![],
            },

            _ => vec![],
        };

        vars.dedup();
        vars
    }

    // specification
    pub fn specification<A : Eq + PartialEq + Clone>(expr : Arith<A>, proof : PropFormula<Fol<A>>) -> PropFormula<Fol<A>> {
        match proof {
            PropFormula::Var(formula) => match formula {
                Fol::ForAll(x, y) => {
                    let arith_vars = get_arith_vars(expr.clone());
                    let bound_vars = bound_vars(*y.clone());

                    for v in arith_vars {
                        if bound_vars.contains(&v) { panic!("specification used incorrecty"); }
                    }

                    substitute(*y, Arith::Var(x), expr)
                },

                _ => panic!("specification used incorrectly"),
            },

            _ => panic!("specification used incorrectly"),
        }
    }

    // generalisation
    pub fn generalisation<A : Eq + PartialEq + Clone>(x : A, premises : Vec<PropFormula<Fol<A>>>, y : PropFormula<Fol<A>>) -> PropFormula<Fol<A>> {
        if bound_vars(y.clone()).contains(&x) { 
            panic!("generalisation used incorrectly"); 
        }

        let mut free_vars = vec![];
        for p in premises {
            let p_bound_vars = bound_vars(p.clone());
            let p_vars = vars(p);

            for v in p_vars {
                if !p_bound_vars.contains(&v) { free_vars.push(v); }
            }

        }

        if free_vars.contains(&x) {
            panic!("generalisation used incorrectly");
        }

        PropFormula::Var(Fol::ForAll(x, Box::new(y)))
    }

    // interchange
    pub fn forall_to_negext<A : Eq + PartialEq + Clone>(proof : PropFormula<Fol<A>>) -> PropFormula<Fol<A>> {
        if let PropFormula::Var(inner) = proof {
            if let Fol::ForAll(x, y) = inner {
                if let PropFormula::Not(negated) = *y {
                    let existential = PropFormula::Var(Fol::Exists(x, negated));
                    
                    return PropFormula::Not(Box::new(existential));
                }
            }
        }

        panic!("forall_to_negext applied incorrectly");
    }

    pub fn negext_to_forall<A: Eq + PartialEq + Clone>(proof : PropFormula<Fol<A>>) -> PropFormula<Fol<A>> {
        if let PropFormula::Not(inner) = proof {
            if let PropFormula::Var(ext) = *inner {
                if let Fol::Exists(x, y) = ext {
                    let negated = PropFormula::Not(y);
                    let universal = Fol::ForAll(x, Box::new(negated));
                    
                    return PropFormula::Var(universal);
                }
            }
        }

        panic!("negext_to_forall applied incorrectly");
    }

    // existence

    // symmetry

    // transitivity

    // add s

    // drop s

    // induction
}