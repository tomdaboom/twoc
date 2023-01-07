// IMPORTS
use std::{env, fs};

// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

// Import parser methods and types
pub mod parser;
use parser::{ast, contract, program};

// Import automaton methods and types
pub mod automaton;
use automaton::{autom, construction};

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

    // Output any parse errors
    if let Err(ref err) = test {
        panic!("Parse Error:\n{:?}", err);
    }

    // Output result of parse
    let mut prog = test.unwrap();

    // Print AST
    println!("AST:");
    prog.print();

    // Contract AST
    prog.contract();

    // Print contracted AST
    println!("\nContracted AST:");
    prog.print();

    // Construct the automaton from the program
    let autom = construction::construct_from_prog(prog);

    // Print the automaton (TODO: make this look nicer)
    println!("\nAutomaton:");
    autom.print();
    //println!("{:?}", autom);
}
