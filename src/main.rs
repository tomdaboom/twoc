// IMPORTS
use std::fs;

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

// Clap import
use clap::Parser;

// Cli arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    file : String,

    #[arg(short, long)]
    word : String,

    #[arg(short, long, default_value_t = false)]
    verbose : bool,
}

fn main() {
    // Declare parser for Twoc rule
    let parser = grammar_rules::TwocParser::new();

    // Parse command line arguments 
    let args = CliArgs::parse();

    let file_path = &args.file;
    let test_word = &args.word;
    let verbose = args.verbose;

    if verbose { 
        println!("Parsing {:?}\n", file_path); 
    }
    
    // Load file
    let test_prog = fs::read_to_string(file_path).expect("File not found");

    // Parse the file
    let test = parser.parse(&test_prog);
    let sugar_prog = match test {
        // Output any parse errors
        Err(ref err) => panic!("Parse Error:\n{:?}", err),
        Ok(prog) => prog,
    };

    if verbose {
        // Output sugared AST
        println!("Sugared AST:");
        sugar_prog.print();
    }

    // Desugar the program
    let mut prog = convert_sugar(sugar_prog);

    // Panic if the input string isn't consistent with the parsed alphabet
    if !prog.check_if_input_in_alphabet(&test_word) {
        panic!("{:?} contains characters that aren't in the program's alphabet!", test_word);
    }

    if verbose {
        // Output if the program is deterministic or not
        match prog.deterministic() {
            true  => print!("\nDeterministic"),
            false => print!("\nNondeterministic"),
        }
        println!(" program detected\n");

        // Print AST
        println!("AST:");
        prog.print();
    }

    // Contract AST
    prog.contract();

    if verbose {
        // Print contracted AST
        println!("\nContracted AST:");
        prog.print();
    }

    if prog.deterministic() {
        // Construct the automaton from the program
        let autom = determ_construction::construct_from_prog(prog);

        // Print the automaton
        if verbose {
            println!("\nAutomaton:");
            autom.print();
        }

        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = glueck_procedure(&autom, test_word);

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} is rejected", test_word),
        }
    } 
    
    else {
        // Construct the automaton from the program
        let autom = construction::construct_from_prog(prog);

        if verbose {
            // Print the automaton
            println!("\nAutomaton:");
            autom.print();
        }
        
        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = naive(&autom, test_word);

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} isn't accepted", test_word),
        }
    }
}
