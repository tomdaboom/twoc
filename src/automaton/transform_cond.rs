use crate::parser::ast::Cond;

pub fn transform_cond(cond : Cond) -> Vec<Cond> {
    match cond {
        Cond::Read(_) => vec![cond],

        Cond::CheckZero() => vec![cond],

        Cond::CheckNotZero() => vec![cond],


    }
}
