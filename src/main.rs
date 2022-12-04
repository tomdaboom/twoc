// IMPORTS
use std::{env, fs};

// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

// Import parser methods and types
pub mod parser;
use parser::{ast, contract};


fn main() {
    // Declare parser for Twoc rule
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
        Ok(ref ast) => {
            // Print AST
            println!("AST:");
            for stmt in ast {
                print!("{}", stmt.print(2));
            }

            // Contract AST
            let contracted = contract::contract(ast);

            // Print contracted AST
            println!("\nContracted AST:");
            for stmt in contracted {
                print!("{}", stmt.print(2));
            }
        }

        Err(ref err) => 
            println!("Parse Error:\n{:?}", err),
    }
}
