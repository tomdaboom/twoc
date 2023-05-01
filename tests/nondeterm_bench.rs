// Nondeterministic benchmarking stuff

#![allow(unused_variables, unused_imports)]

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
    //use twoc::automaton::autom::Autom;
    use twoc::simulation::rytter;
    use twoc::simulation::rytter_matrix;
    use twoc::simulation::glueck_nondeterm;


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
            //thread::scope(|s| {
            //    thread::Builder::new().stack_size(0xFFFF * n)
            //    .spawn_scoped(s, || thread_function(&autom, test_word))
             //   .unwrap();
            //});

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
    pub fn quad_glueck() {
        // Loop params
        let start = 0;
        let step = 1;
        let tests = 1000;

        // Initialise last
        let mut last = 0.0f32;

        // Create output file
        let path = "./tests/bench_results/pl_2704_glueck_nondeterm.csv";
        let mut file = fs::File::create(path).expect("File creation failed");

        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/nondeterm/potential_loop.twoc").expect("File not found");

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
                .spawn_scoped(s, || glueck_nondeterm::glueck_procedure(&autom, &test_word))
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
    pub fn quadratic_performance_test() {
        // Loop params
        let start = 0;
        let step = 1;
        let tests = 100;

        // Initialise last
        let mut last_hashmap = 0.0f32;
        let mut last_matrix = 0.0f32;

        // Create output files
        let path1 = "./tests/bench_results/qpt_2204_rytter.csv";
        let mut file1 = fs::File::create(path1).expect("File creation failed");

        let path2 = "./tests/bench_results/qpt_2204_rytter_matrix.csv";
        let mut file2 = fs::File::create(path2).expect("File creation failed");

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
            println!("n = {:?}", n);

            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n);

            // HASHMAP TEST
            let now = Instant::now();
            rytter::rytter_procedure(&autom, &test_word);
            let time_taken_hashmap = now.elapsed().as_secs_f32();
            let delta_t_hashmap = time_taken_hashmap - last_hashmap;


            // MATRIX TEST
            let now = Instant::now();
            rytter_matrix::rytter_procedure(&autom, &test_word);
            let time_taken_matrix = now.elapsed().as_secs_f32();
            let delta_t_matrix = time_taken_matrix - last_matrix;

            // Output and save time taken and difference between last time and this time
            let to_file = format!("{:?},{:?},{:?}\n", n, time_taken_hashmap, delta_t_hashmap);
            file1.write_all(to_file.as_bytes()).expect("File write failed");

            let to_file = format!("{:?},{:?},{:?}\n", n, time_taken_matrix, delta_t_matrix);
            file2.write_all(to_file.as_bytes()).expect("File write failed");

            last_hashmap = time_taken_hashmap;
            last_matrix = time_taken_matrix;
        }
    }

    #[test]
    pub fn awful_performance_test() {
        // Loop params
        let start = 0;
        let step = 1;
        let tests = 100;

        // Initialise last
        let mut last_hashmap = 0.0f32;
        let mut last_matrix = 0.0f32;

        // Create output files
        let path1 = "./tests/bench_results/awful_2204_rytter.csv";
        let mut file1 = fs::File::create(path1).expect("File creation failed");

        let path2 = "./tests/bench_results/awful_2204_rytter_matrix.csv";
        let mut file2 = fs::File::create(path2).expect("File creation failed");

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
            println!("n = {:?}", n);

            // Generate a string of n 0s and n 1s
            let test_word = "0".repeat(n);

            // HASHMAP TEST
            let now = Instant::now();
            rytter::rytter_procedure(&autom, &test_word);
            let time_taken_hashmap = now.elapsed().as_secs_f32();
            let delta_t_hashmap = time_taken_hashmap - last_hashmap;


            // MATRIX TEST
            let now = Instant::now();
            rytter_matrix::rytter_procedure(&autom, &test_word);
            let time_taken_matrix = now.elapsed().as_secs_f32();
            let delta_t_matrix = time_taken_matrix - last_matrix;

            // Output and save time taken and difference between last time and this time
            let to_file = format!("{:?},{:?},{:?}\n", n, time_taken_hashmap, delta_t_hashmap);
            file1.write_all(to_file.as_bytes()).expect("File write failed");

            let to_file = format!("{:?},{:?},{:?}\n", n, time_taken_matrix, delta_t_matrix);
            file2.write_all(to_file.as_bytes()).expect("File write failed");

            last_hashmap = time_taken_hashmap;
            last_matrix = time_taken_matrix;
        }
    }
}