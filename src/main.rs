pub mod parser;

#[macro_use] extern crate lalrpop_util;

use parser::ast;

lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

fn main() {
    // Declare Stmt parser
    let stmt_list_parser = grammar_rules::StmtListParser::new();

    // Declare test string
    let test_prog = "while ( read == 'C' || read == 'C' && !(c == 0) ) { c -= 12; }";

    // Parse test string
    let test = stmt_list_parser.parse(test_prog);

    match test {
        Ok(ref ast) => 
            println!("AST:\n{:?}", ast),

        Err(_) => 
            println!("Parse erorr detected!"),
    }
}
