// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod auto_tests {
    use std::{fs, thread};
    use crate::grammar_rules::TwocParser;
    use twoc::parser::sugar::convert_sugar::convert_sugar;
    use twoc::automaton::determ_construction::construct_from_prog; 
    use twoc::simulation::glueck::glueck_procedure;

    #[test]
    fn evens() {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/deterministic/evens.twoc").expect("File not found");

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

        // Construct the automaton from the program
        let autom = construct_from_prog(prog);

        // Shitton of tests
        for i in 0..1000 {
            // The automaton should accept if i is even
            let should_accept = i % 2 == 0;

            // Print i occasionally
            if i % 1 == 0 {
                println!("{:?}", i);
            }

            // Make input string
            let input = "0".repeat(i);

            // Run test with a massive stack of size O(n)
            thread::scope(|s| {
                thread::Builder::new().stack_size(0xFFFF * i)
                .spawn_scoped(s, || assert_eq!(glueck_procedure(&autom, &input), should_accept))
                .unwrap();
            });
        }
    }
}