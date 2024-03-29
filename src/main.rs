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
use twoc::simulation::{glueck_nondeterm, glueck_array, rytter, rytter_matrix};

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

    #[arg(long, default_value_t = false)]
    use_rytter_matrix : bool,
}

fn main() -> Result<(), String> {
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
    let use_rytter_matrix = args.use_rytter_matrix;

    if use_glueck_nondeterm {
        println!("Warning: using the --use-glueck-nondeterm flag might lead to incorrect results on some programs!");
    }

    if verbose { 
        println!("\nParsing {:?}\n", file_path); 
    }
    
    // Load file
    let test_prog = match fs::read_to_string(file_path) {
        Ok(str) => str,
        Err(_) => return Err(format!("Couldn't find {:?}!", file_path)),
    };

    // Parse the file
    let test = parser.parse(&test_prog);
    let sugar_prog = match test {
        // Output any parse errors
        Err(ref err) 
            => return Err(format!("Parsing failed with the following error message:\n{:?}", err)),
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
        return Err(format!("{:?} contains characters that aren't in the program's alphabet!", test_word));
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


    if prog.deterministic() {
        // Construct the automaton from the program
        let autom = determ_construction::construct_from_prog(prog);

        // Print the automaton
        if verbose {
            println!("\nAutomaton:");
            autom.print();
        }

        // Test that the automaton accepts an example word via the glueck procedure
        let accepting = glueck_array::glueck_procedure(&autom, test_word);

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} is rejected", test_word),
        }

        return Ok(());
    } 
    
    else {
        // Construct the automaton from the program
        let autom = construction::construct_from_prog(prog);

        // Print the automaton
        if verbose {
            println!("\nAutomaton:");
            autom.print();
        }

        // Test that the automaton accepts an example word via the chosen procedure
        let accepting = match (use_glueck_nondeterm, use_rytter_matrix) {
            (true, true)   => return Err("--use-glueck-nondeterm and --use-rytter-matrix are mutually exclusive!".to_string()),

            (false, true)  => rytter_matrix::rytter_procedure(&autom, test_word),

            (true, false)  => glueck_nondeterm::glueck_procedure(&autom, test_word),

            (false, false) => rytter::rytter_procedure(&autom, test_word),
        };

        match accepting {
            true  => println!("\n{:?} is accepted", test_word),
            false => println!("\n{:?} is rejected", test_word),
        }

        return Ok(());
    }
}
