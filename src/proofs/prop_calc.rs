#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropFormula<A> {
    Var(A),

    Not(Box<PropFormula<A>>),

    And(Box<PropFormula<A>>, Box<PropFormula<A>>),

    Or(Box<PropFormula<A>>, Box<PropFormula<A>>),

    Imp(Box<PropFormula<A>>, Box<PropFormula<A>>),
}

// TODO: Fiddle around with these to make it work properly (i know what i'm talking about here trust me)

mod rules {
    use super::PropFormula;

    pub fn fantasy<A : Clone, F>(x : PropFormula<A>, f : F) -> PropFormula<A> 
        where F : Fn(PropFormula<A>) -> PropFormula<A> {
        PropFormula::Imp(Box::new(x.clone()), Box::new(f(x)))
    }

    pub fn contrapositive<A>(imp : PropFormula<A>) -> PropFormula<A> {
        match imp {
            PropFormula::Imp(x, y) 
                => PropFormula::Imp(Box::new(PropFormula::Not(x)), Box::new(PropFormula::Not(y))),

            _ => panic!("contrapositive applied incorrectly"),
        }
    }

    pub fn double_neg_intro<A>(x : PropFormula<A>) -> PropFormula<A> {
        PropFormula::Not(Box::new(
            PropFormula::Not(Box::new(x))
        ))
    }

    pub fn double_neg_removal<A>(x : PropFormula<A>) -> PropFormula<A> {
        if let PropFormula::Not(neg1) = x {
            if let PropFormula::Not(neg2) = *neg1 {
                return *neg2;
            }
        }

        panic!("double_neg_removal applied incorrectly");
    }

    pub fn join<A : Clone>(x : PropFormula<A>, y : PropFormula<A>) -> PropFormula<A> {
        PropFormula::And(Box::new(x), Box::new(y))
    }

    pub fn detach<A : PartialEq>(x : PropFormula<A>, x_imp_y : PropFormula<A>) -> PropFormula<A> {
        match x_imp_y {
            PropFormula::Imp(x2, y) => {
                if x != *x2 { panic!("detach applied incorrectly"); }

                *y
            },

            _ => panic!("detach applied incorrectly"),
        }
    }

    pub fn de_morgan_or<A>(disj : PropFormula<A>) -> PropFormula<A> {
        match disj {
            PropFormula::Or(x, y) => {
                let neg_x = Box::new(PropFormula::Not(x));
                let neg_y = Box::new(PropFormula::Not(y));

                PropFormula::And(neg_x, neg_y)
            },

            _ => panic!("de_morgan_or applied incorrectly"),

        }

    }

    pub fn de_morgan_and<A>(conj : PropFormula<A>) -> PropFormula<A> {
        match conj {
            PropFormula::And(x, y) => {
                let neg_x = Box::new(PropFormula::Not(x));
                let neg_y = Box::new(PropFormula::Not(y));

                PropFormula::Or(neg_x, neg_y)
            },

            _ => panic!("de_morgan_or applied incorrectly"),

        }
    }

    pub fn sep_l<A>(x : PropFormula<A>) -> PropFormula<A> {
        match x {
            PropFormula::And(l, _) => *l,
            
            _ => panic!("sep_l applied incorrectly"),
        }
    }

    pub fn sep_r<A>(x : PropFormula<A>) -> PropFormula<A> {
        match x {
            PropFormula::And(_, r) => *r,
            
            _ => panic!("sep_r applied incorrectly"),
        }
    }

    pub fn switcheroo<A>(disj : PropFormula<A>) -> PropFormula<A> {
        if let PropFormula::Or(x, y) = disj {
            let not_x = Box::new(PropFormula::Not(x));

            return PropFormula::Imp(not_x, y);
        }

        if let PropFormula::Imp(x, y) = disj {
            let not_x = Box::new(PropFormula::Not(x));

            return PropFormula::Or(not_x, y);
        }

        panic!("switcheroo applied incorrectly");
    }

}

pub enum Pos { Left, Right }

pub fn apply_prop_rule<A, F>(path : &[Pos], rule : F, formula : PropFormula<A>) -> PropFormula<A> 
    where F : Fn(PropFormula<A>) -> PropFormula<A> 
{
    let (head, tail) = match path.split_first() {
        Some(lists) => lists,

        None => return rule(formula),
    };

    match formula {
        PropFormula::Not(inner) => { 
            let new_inner = apply_prop_rule(tail, rule, *inner);
            
            PropFormula::Not(Box::new(new_inner))
        },

        PropFormula::Var(v) => return PropFormula::Var(v),

        PropFormula::And(left, right) => {
            match *head {
                Pos::Left  => {
                    let applied_left = apply_prop_rule(tail, rule, *left);
                    PropFormula::And(Box::new(applied_left), right)
                },

                Pos::Right => {
                    let applied_right = apply_prop_rule(tail, rule, *right);
                    PropFormula::And(left, Box::new(applied_right))
                },
            }
        },

        PropFormula::Or(left, right) => {
            match *head {
                Pos::Left  => {
                    let applied_left = apply_prop_rule(tail, rule, *left);
                    PropFormula::Or(Box::new(applied_left), right)
                },

                Pos::Right => {
                    let applied_right = apply_prop_rule(tail, rule, *right);
                    PropFormula::Or(left, Box::new(applied_right))
                },
            }
        },

        PropFormula::Imp(left, right) => {
            match *head {
                Pos::Left  => {
                    let applied_left = apply_prop_rule(tail, rule, *left);
                    PropFormula::Imp(Box::new(applied_left), right)
                },

                Pos::Right => {
                    let applied_right = apply_prop_rule(tail, rule, *right);
                    PropFormula::Imp(left, Box::new(applied_right))
                },
            }
        },
        
    }
}