// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

#[cfg(test)]
mod determ_tests {
    // Import grammar
    use crate::grammar_rules;

    // Import file system
    use std::fs;

    // Import automaton construction and glueck procedure
    use twoc::automaton::{construction, glueck::glueck_procedure};

    #[test]
    pub fn equal_zeros_ones() {
        // Declare parser for Twoc rule
        let parser = grammar_rules::TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string("./twocprogs/equal_zeros_ones.twoc").expect("File not found");

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

        // Some test examples
        let test_words = [
            ("0011", true), 
            ("11001", false),
            ("0101101010", true),
            ("11110101101110011110011111111111", false),
        ];

        // Check that each of the words gives the correct answer
        for (word, expected) in test_words {
            let glueck_output = glueck_procedure(autom.clone(), word);
            assert_eq!(glueck_output, expected);
        }

    }
}