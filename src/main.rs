#[macro_use] extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    // Declare Stmt parser
    let stmt_list_parser = parser::StmtListParser::new();

    // Declare test string
    let test_prog = "while ( read == 'C' || read == 'C' && !(c == 0) ) { }";

    // Parse test string
    let test = stmt_list_parser.parse(test_prog).unwrap();

    // Print test ast
    println!("AST:\n{:?}", test);
}
