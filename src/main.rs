#[macro_use] extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    let test1 = parser::ParseStmt::new().parse("move(6);");

    //println!("{:?}", test1);
}
