// IMPORTS
use std::{env, fs};

extern crate hashbrown;

// Import grammar
#[macro_use] extern crate lalrpop_util;
//lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

// Import parser methods and types
pub mod parser;
use twoc::parser::sugar::convert_sugar::convert_sugar;

// Import automaton methods and types
pub mod automaton;
pub mod simulation;
use twoc::automaton::{determ_construction, construction};
use twoc::simulation::glueck::glueck_procedure;
//use twoc::simulation::ahu::ahu_procedure;
use twoc::simulation::naive_nondeterm::naive;

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

    // Parse the file
    let test = parser.parse(&test_prog);
    let sugar_prog = match test {
        // Output any parse errors
        Err(ref err) => panic!("Parse Error:\n{:?}", err),
        Ok(prog) => prog,
    };

    // Desugar the program
    let mut prog = convert_sugar(sugar_prog);

    // Panic if the input string isn't consistent with the parsed alphabet
    if !prog.check_if_input_in_alphabet(&test_word) {
        panic!("{:?} contains characters that aren't in the program's alphabet!", test_word);
    }

    // Output if the program is deterministic or not
    match prog.deterministic() {
        true  => print!("Deterministic"),
        false => print!("Nondeterministic"),
    }
    println!(" program detected\n");

    // Print AST
    println!("AST:");
    prog.print();

    if prog.deterministic() {
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
        let accepting = glueck_procedure(&autom, test_word);

        match accepting {
            true  => print!("\n{:?} is accepted", test_word),
            false => print!("\n{:?} isn't accepted", test_word),
        }

    } else {
        // Construct the automaton from the program
        let autom = construction::construct_from_prog(prog);

        // Print the automaton
        println!("\nAutomaton:");
        autom.print();

        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = naive(&autom, test_word);

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} isn't accepted", test_word),
        }
    }
}
