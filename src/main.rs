pub mod parser;

#[macro_use] extern crate lalrpop_util;

use std::env;
use std::fs;

use parser::ast;

lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

fn main() {
    // Declare parser for StmtList rule
    let parser = grammar_rules::TwocParser::new();

    // Get name of file from command line args
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Parsing {:?}\n", file_path);

    // Load file
    let test_prog = fs::read_to_string(file_path).expect("File not found");

    // Parse string
    let test = parser.parse(&test_prog);

    // Output result of parse
    match test {
        Ok(ref ast) => 
            println!("AST:\n{:?}", ast),

        Err(ref err) => 
            println!("Parse Error:\n{:?}", err),
    }
}
