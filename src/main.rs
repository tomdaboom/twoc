#[macro_use] extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    // Declare Stmt parser
    let stmt_list_parser = parser::StmtListParser::new();

    let test_prog = "while (cond) { c += 2; move(-6); reject; }";

    let test1 = stmt_list_parser.parse(test_prog).unwrap();

    println!("AST:\n{:?}", test1);
}
