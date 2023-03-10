// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

#[cfg(test)]
mod determ_bench {
    use std::io::Write;
    use std::{fs, thread};
    use std::time::Instant;

    use crate::grammar_rules::TwocParser;
    use twoc::automaton::determ_construction; 
    use twoc::automaton::determ_autom::Autom;
    use twoc::simulation::glueck::glueck_procedure;

    // Function used by threads
    pub fn thread_function(autom : &Autom, word : String) {
        assert_eq!(glueck_procedure(&autom, &word), true);
    }

    #[test]
    pub fn string_length_performance_test() {
        // Loop params
        let start = 10000;
        let step = 500;
        let tests = 100;

        // Initialise last
        let mut last = 0.0f32;

        // Create output file
        let path = "./tests/bench_results/string_length_performance_test.txt";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/deterministic/equal_zeros_ones.twoc").expect("File not found");

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

        for n in (start..(start + step*tests + 1)).step_by(step) {
            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n) + &"1".repeat(n);

            // Start timing
            let now = Instant::now();

            // Run test with a massive stack
            thread::scope(|s| {
                thread::Builder::new().stack_size(0xFFFF * n)
                .spawn_scoped(s, || thread_function(&autom, test_word))
                .unwrap();
            });

            // Stop timing and record delta t
            let time_taken = now.elapsed().as_secs_f32();
            let delta_t = time_taken - last;

            // Output and save time taken and difference between last time and this time
            println!("n = {:?}, t = {:?}, Δt = {:?}", n*2, time_taken, delta_t);
            let to_file = format!("{:?},{:?},{:?}\n", n*2, time_taken, delta_t);
            file.write_all(to_file.as_bytes()).expect("File write failed");

            last = time_taken;
        }
    }

    #[test]
    pub fn quadratic_performance_test() {
        // Loop params
        let start = 10000;
        let step = 500;
        let tests = 100;

        // Initialise last
        let mut last = 0.0f32;

        // Create output file
        let path = "./tests/bench_results/quadratic_performance_test.txt";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/deterministic/very_long.twoc").expect("File not found");

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

        for n in (start..(start + step*tests + 1)).step_by(step) {
            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n);

            // Start timing
            let now = Instant::now();

            // Run test with a massive stack
            thread::scope(|s| {
                thread::Builder::new().stack_size(0xFFFF * n)
                .spawn_scoped(s, || thread_function(&autom, test_word))
                .unwrap();
            });

            // Stop timing and record delta t
            let time_taken = now.elapsed().as_secs_f32();
            let delta_t = time_taken - last;

            // Output and save time taken and difference between last time and this time
            println!("n = {:?}, t = {:?}, Δt = {:?}", n, time_taken, delta_t);
            let to_file = format!("{:?},{:?},{:?}\n", n, time_taken, delta_t);
            file.write_all(to_file.as_bytes()).expect("File write failed");

            last = time_taken;
        }
    }
}