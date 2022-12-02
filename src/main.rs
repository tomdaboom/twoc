#[macro_use] extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    // Declare Stmt parser
    let stmt_parser = parser::StmtParser::new();

    let test1 = stmt_parser.parse("move(6);").unwrap();

    let expected1 = ast::Stmt::Move(6);

    println!("{:?} = {:?}", test1, expected1);
}
