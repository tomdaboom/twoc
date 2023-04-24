// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod auto_tests {
    use std::{fs, thread};

    use crate::grammar_rules::TwocParser;

    type DetermTransition = twoc::automaton::autom::Transition;
    use twoc::automaton::generic_autom::GenericAutom;

    use twoc::parser::sugar::convert_sugar::convert_sugar;
    use twoc::automaton::determ_construction::construct_from_prog; 
    use twoc::simulation::glueck::glueck_procedure;

    // Function to load an automaton
    fn load_autom(path : &str) -> GenericAutom<DetermTransition> {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string(path).expect("File not found");

        // Parse the file
        let test = parser.parse(&test_prog);
        let sugared_prog = match test {
            // Output any parse errors
            Err(ref err) => panic!("Parse Error:\n{:?}", err),
            Ok(prog) => prog,
        };

        // Desugar
        let mut prog = convert_sugar(sugared_prog);

        // Contract the AST
        prog.contract();

        // Return
        construct_from_prog(prog)
    }

    #[test]
    fn evens() {
        // Construct the automaton from the program
        let autom = load_autom("./twocprogs/determ/evens.twoc");

        // Shitton of tests
        for i in 0..5000 {
            // The automaton should accept iff i is even
            let should_accept = i % 2 == 0;

            // Make input string
            let input = "0".repeat(i);

            // Run test with a massive stack of size O(n)
            thread::scope(|s| {
                thread::Builder::new().stack_size(0xFFFF * i)
                .spawn_scoped(s, 
                    || assert_eq!(glueck_procedure(&autom, &input), should_accept)
                ).unwrap();
            });

            // Print i occasionally
            if i % 100 == 0 {
                println!("{:?} tests passed", i);
            }
        }
    }

    #[test]
    fn equal_zeros_ones() {
        // Construct the automaton from the program
        let autom = load_autom("./twocprogs/determ/equal_zeros_ones.twoc");

        // Shitton of tests
        for i in 0..5000 {
            // The automaton should accept iff i is even
            let should_accept = i % 2 == 0;

            // Make input string
            let input = "0".repeat(i);

            // Run test with a massive stack of size O(n)
            thread::scope(|s| {
                thread::Builder::new().stack_size(0xFFFF * i)
                .spawn_scoped(s, 
                    || assert_eq!(glueck_procedure(&autom, &input), should_accept)
                ).unwrap();
            });

            // Print i occasionally
            if i % 100 == 0 {
                println!("{:?} tests passed", i);
            }
        }
    }
}