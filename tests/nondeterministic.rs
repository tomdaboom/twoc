// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/grammar_rules.rs");

#[cfg(test)]
mod determ_tests {
    use std::fs;
    use crate::grammar_rules::TwocParser;
    use twoc::automaton::construction; 
    use twoc::simulation::glueck_nondeterm::glueck_procedure;

    // Generic test function that runs a program on a single word and compares the outputs
    fn generic_test(filename : &str, examples : &[(&str, bool)]) {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string(filename).expect("File not found");

        // Parse the file
        let test = parser.parse(&test_prog);
        let prog = match test {
            // Output any parse errors
            Err(ref err) => panic!("Parse Error:\n{:?}", err),
            Ok(prog) => prog,
        };

        // Construct the automaton from the program
        let autom = construction::construct_from_prog(prog);

        // Check that each of the words gives the correct answer
        for (word, expected) in examples {
            let glueck_output = glueck_procedure(&autom, word);
            assert_eq!(glueck_output, *expected);
        }
    }

    #[test]
    pub fn equal_zeros_ones() {
        let test_words = [
            ("0011", true), 
            ("11001", false),
            ("0101101010", true),
            ("11110101101110011110011111111111", false),
        ];

        generic_test("./twocprogs/deterministic/equal_zeros_ones.twoc", &test_words);
    }

    #[test]
    pub fn zeros_then_ones() {
        let test_words = [
            ("00000000001111111111", true), 
            ("00001111", true),
            ("0001111", false),
            ("000001111", false),
            ("1010101010101010", false),
            ("1001010101101010111111010101", false),
        ];

        generic_test("./twocprogs/deterministic/zeros_then_ones.twoc", &test_words);
    }

    #[test]
    pub fn x_plus_y_is_z() {
        let test_words = [
            ("xxxyyyyyzzzzzzzz", true),
            ("xzyxzzyyzxzyzxzyzzyxzyzxzz", true), 
            ("xyyzzzz", false),
            ("xyxyxyxyxyzzzzzzzz", false),
        ];

        generic_test("./twocprogs/deterministic/x_plus_y_is_z.twoc", &test_words);
    }

    #[test]
    pub fn loops_forever() {
        let test_words = [
            ("0", false),
        ];

        generic_test("./twocprogs/deterministic/loops_forever.twoc", &test_words);
    }
}