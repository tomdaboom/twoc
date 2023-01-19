// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

#[cfg(test)]
mod determ_bench {
    use std::io::Write;
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
        // Loop params
        let start = 1000;
        let step = 100;
        let tests = 100;

        // Initialise last
        let mut last = 0.0f32;

        // Initialise vector of output results
        let mut out_results = Vec::new();

        for n in (start..(start + step*tests)).step_by(step) {
            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n) + &"1".repeat(n);

            // Declare a thread to run the test with a stack that's way bigger than neccesary
            let thread_builder = thread::Builder::new().stack_size(0xFFFF * n);

            // Start timing
            let now = Instant::now();

            // Run test
            let caller_thread = thread_builder.spawn(move || 
                generic_test(
                    "./twocprogs/zeros_then_ones.twoc", 
                    (test_word.as_str(), true),
                )
            ).unwrap();

            caller_thread.join().unwrap();

            // Stop timing and record delta t
            let time_taken = now.elapsed().as_secs_f32();
            let delta_t = time_taken - last;

            // Output and save time taken and difference between last time and this time
            println!("n = {:?}, t = {:?}, Δt = {:?}", n, time_taken, delta_t);
            out_results.push((n, time_taken, delta_t));

            last = time_taken;
        }

        // Write results to a .txt
        let path = "./tests/bench_results/string_length_performance_test.txt";
        let mut file = fs::File::create(path).expect("File creation failed");
        for (n, t, dt) in out_results {
            let out = format!("{:?},{:?},{:?}\n", n, t, dt);
            file.write_all(out.as_bytes()).expect("File write failed");
        }
    }
}