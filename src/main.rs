// IMPORTS
use std::{env, fs};

// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

// Import parser methods and types
pub mod parser;

// Import automaton methods and types
pub mod automaton;
use twoc::automaton::{determ_construction, glueck};

fn main() {
    // Declare parser for Twoc rule
    let parser = grammar_rules::TwocParser::new();

    // Get name of file from command line args
    let args : Vec<String> = env::args().collect();
    let file_path = &args[1];
    let test_word = &args[2];
    println!("Parsing {:?}\n", file_path);

    // Load file
    let test_prog = fs::read_to_string(file_path).expect("File not found");

    // Parse string
    let test = parser.parse(&test_prog);

    // Output any parse errors
    if let Err(ref err) = test {
        panic!("Parse Error:\n{:?}", err);
    }

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
    let autom = determ_construction::construct_from_prog(prog);

    // Print the automaton
    println!("\nAutomaton:");
    autom.print();

    // Test that the automaton accepts an example word via the glueck procedure
    let accepting = glueck::glueck_procedure(autom, test_word);

    if accepting {
        print!("\n{:?} is accepted", test_word);
    } else {
        print!("\n{:?} isn't accepted", test_word);
    }
    
}
