#![allow(unused_imports)]

// IMPORTS
use std::fs;

// Import grammar
#[macro_use] extern crate lalrpop_util;
//lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

// Import parser methods and types
use twoc::parser::sugar::convert_sugar::convert_sugar;

// Import automaton methods and types
use twoc::automaton::{determ_construction, construction};
use twoc::simulation::{glueck, glueck_nondeterm, glueck_array, rytter};

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

    #[arg(long, default_value_t = false)]
    use_glueck_nondeterm : bool,
}

fn main() -> Result<(), ()> {
    // Declare parser for Twoc rule
    let parser = grammar_rules::TwocParser::new();

    // Parse command line arguments 
    let args = CliArgs::parse();

    let file_path = &args.file;
    let test_word = match args.word.as_str() {
        "//EMPTY//" => "",
        _ => &args.word,
    };
    let verbose = args.verbose;
    let use_glueck_nondeterm = args.use_glueck_nondeterm;

    if use_glueck_nondeterm {
        println!("Warning: using the --use-glueck-nondeterm flag might lead to incorrect results on some nondeterministic programs!");
    }

    if verbose { 
        println!("\nParsing {:?}\n", file_path); 
    }
    
    // Load file
    let test_prog = match fs::read_to_string(file_path) {
        Ok(str) => str,
        Err(_) => {
            println!("Couldn't find {:?}!", file_path);
            return Err(());
        },
    };

    // Parse the file
    let test = parser.parse(&test_prog);
    let sugar_prog = match test {
        // Output any parse errors
        Err(ref err) => {
            println!("Parse Error:\n{:?}", err);
            return Err(());
        },
        Ok(prog) => prog,
    };

    if verbose {
        // Output sugared AST
        println!("Sugared AST:");
        sugar_prog.print();
    }

    // Desugar the program
    let mut prog = convert_sugar(sugar_prog);

    // Crash if the input string isn't consistent with the parsed alphabet
    if !prog.check_if_input_in_alphabet(&test_word) {
        println!("{:?} contains characters that aren't in the program's alphabet!", test_word);
        return Err(());
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

    if false{//verbose {
        // Print contracted AST
        println!("\nContracted AST:");
        prog.print();
    }

    // Check whether or not the program is deterministic
    let determ = prog.deterministic();

    // Construct the automaton from the program
    let autom = determ_construction::construct_from_prog(prog);

    // Print the automaton
    if verbose {
        println!("\nAutomaton:");
        autom.print();
    }

    if determ {
        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = glueck_array::glueck_procedure(&autom, test_word);

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} is rejected", test_word),
        }

        return Ok(());
    } 
    
    else {
        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = match use_glueck_nondeterm {
            true  => glueck_nondeterm::glueck_procedure(&autom, test_word),
            false => rytter::rytter_procedure(&autom, test_word),
        };

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} is rejected", test_word),
        }

        return Ok(());
    }
}
