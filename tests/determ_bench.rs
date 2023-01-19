// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

#[cfg(test)]
mod determ_bench {
    use std::{fs, thread};
    use crate::grammar_rules::TwocParser;
    use twoc::automaton::determ_construction; 
    use twoc::simulation::glueck::glueck_procedure;
    use std::time::Instant;

    // Generic test function that runs a program on a list of examples and compares the outputs
    fn generic_test(filename : &str, example : (&str, bool)) {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string(filename).expect("File not found");

        // Parse the file
        let test = parser.parse(&test_prog);
        let mut prog = match test {
            // Output any parse errors
            Err(ref err) => panic!("Parse Error:\n{:?}", err),
            Ok(prog) => prog,
        };

        // Contract the AST
        prog.contract();

        // Construct the automaton from the program
        let autom = determ_construction::construct_from_prog(prog);

        // Check that the word gives the correct answer
        let (word, expected) = example;
        let glueck_output = glueck_procedure(&autom, word);
        assert_eq!(glueck_output, expected);
    }

    #[test]
    pub fn string_length_performance_test() {
        let start = 1000;
        let step = 500;
        let tests = 10;

        for n in (start..(start + step*tests)).step_by(step) {
            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n) + &"1".repeat(n);

            // Start timing
            let now = Instant::now();

            // Declare a caller thread to run the test with a stack that's way bigger than neccesary
            let caller = thread::Builder::new()
                .stack_size(100 * n * 0xFF)
                .spawn(move || 
                    generic_test(
                        "./twocprogs/zeros_then_ones.twoc", 
                        (test_word.as_str(), true),
                    )
                ).unwrap();

            caller.join().unwrap();

            // Output time taken
            println!("n = {:?}, t = {:?}", n, now.elapsed().as_secs_f32());
        }
    }
}