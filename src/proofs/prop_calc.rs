#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropFormula<A> {
    Var(A),

    Not(Box<PropFormula<A>>),

    And(Box<PropFormula<A>>, Box<PropFormula<A>>),

    Or(Box<PropFormula<A>>, Box<PropFormula<A>>),

    Imp(Box<PropFormula<A>>, Box<PropFormula<A>>),
}

mod rules {
    use super::PropFormula;

    pub fn join<A : Clone>(x : PropFormula<A>, y : PropFormula<A>) -> PropFormula<A> {
        PropFormula::And(Box::new(x), Box::new(y))
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

    pub fn fantasy<A : Clone, F>(x : PropFormula<A>, f : F) -> PropFormula<A> 
        where F : Fn(PropFormula<A>) -> PropFormula<A> {
        PropFormula::Imp(Box::new(x.clone()), Box::new(f(x)))
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