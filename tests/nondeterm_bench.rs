// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");
lalrpop_mod!(pub sugar_grammar, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod nondeterm_bench {
    use std::io::Write;
    use std::{fs, thread};
    use std::time::Instant;

    use crate::grammar_rules::TwocParser;
    use crate::sugar_grammar::TwocParser as SugarTwocParser;
    use twoc::parser::sugar::convert_sugar::convert_sugar;
    use twoc::automaton::construction; 
    use twoc::automaton::autom::Autom;
    use twoc::simulation::rytter::rytter_procedure;

    // Function used by threads
    pub fn thread_function(autom : &Autom, word : String) {
        rytter_procedure(&autom, &word);
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
        let path = "./tests/bench_results/nondeterm_torture_boogaloo.txt";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/determ/equal_zeros_ones.twoc").expect("File not found");

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
        let autom = construction::construct_from_prog(prog);

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
        let start = 0;
        let step = 1;
        let tests = 120;

        // Initialise last
        let mut last = 0.0f32;

        // Create output file
        let path = "./tests/bench_results/quadratic_performance_test_rytter_hashmaps_2_2.txt";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/determ/very_long.twoc").expect("File not found");

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
        let autom = construction::construct_from_prog(prog);

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

    #[test]
    pub fn awful_performnce_test() {
        // Loop params
        let start = 0;
        let step = 1;
        let tests = 1000;

        // Initialise last
        let mut last = 0.0f32;

        // Create output file
        let path = "./tests/bench_results/nondeterm_torture_rytter_hashmap.txt";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = SugarTwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/nondeterm/branch_while.twoc").expect("File not found");

        // Parse the file
        let test = parser.parse(&test_prog);
        let sugar_prog = match test {
            // Output any parse errors
            Err(ref err) => panic!("Parse Error:\n{:?}", err),
            Ok(prog) => prog,
        };

        // Desugar
        let mut prog = convert_sugar(sugar_prog);

        // Contract the AST
        prog.contract();

        // Construct the automaton from the program
        let autom = construction::construct_from_prog(prog);

        for n in (start..(start + step*tests + 1)).step_by(step) {
            // Generate a string of n 0s
            let test_word = "0".repeat(n);

            // Start timing
            let now = Instant::now();

            // Run test with a massive stack
            thread_function(&autom, test_word);

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