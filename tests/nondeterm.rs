#![allow(unused_imports)]

// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod nondeterm_tests {
    use std::fs;
    use crate::grammar_rules::TwocParser;
    use twoc::parser::sugar::convert_sugar::convert_sugar;
    use twoc::automaton::construction::construct_from_prog; 

    use twoc::simulation::glueck_nondeterm::glueck_procedure;
    use twoc::simulation::rytter;
    use twoc::simulation::rytter_matrix;
    use twoc::simulation::glueck_nondeterm;

    // Generic test function that runs a program on a single word and compares the outputs
    fn generic_test(filename : &str, examples : &[(&str, bool)]) {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        // Load file
        let test_prog = fs::read_to_string(filename).expect("File not found");

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

        // Check that each of the words gives the correct answer
        for (word, expected) in examples {
            //let output = rytter::rytter_procedure(&autom, word);
            let output = rytter::rytter_procedure(&autom, word);
            assert_eq!(output, *expected);
        }
    }


    // DETERMINISTIC PROGRAMS

    #[test]
    pub fn equal_zeros_ones() {
        let test_words = [
            ("", true),
            ("0011", true), 
            ("11001", false),
            ("0101101010", true),
            ("11110101101110011110011111111111", false),
            ("1111111111111111000000000000000011111111111111110000000000000000", true),
            ("11111111111111110000000000000000111111111111111100000000000000000", false),
            ("111111111111111100000000100000000111111111111111100000000000000000", true),
        ];

        generic_test("./twocprogs/determ/equal_zeros_ones.twoc", &test_words);
    }

    #[test]
    pub fn zeros_then_ones() {
        let test_words = [
            ("", true),
            ("00000000001111111111", true), 
            ("00001111", true),
            ("0001111", false),
            ("000001111", false),
            ("1010101010101010", false),
            ("1001010101101010111111010101", false),
        ];

        generic_test("./twocprogs/determ/zeros_then_ones.twoc", &test_words);
    }

    #[test]
    pub fn x_plus_y_is_z() {
        let test_words = [
            ("", true),
            ("xxxyyyyyzzzzzzzz", true),
            ("xzyxzzyyzxzyzxzyzzyxzyzxzz", true), 
            ("xyyzzzz", false),
            ("xyxyxyxyxyzzzzzzzz", false),
            ("zzzzxyzxyzxyzxyzxyzxyzxyzxyzzzzz", true),
            ("zzzzxyzxyzxyzxyzxyzxyzxyzxyzzzxzz", false),
            ("zzzzxyyzxyzxyzxyzzxyzxyzxyzxyzzzxzz", false),
        ];

        generic_test("./twocprogs/determ/x_plus_y_is_z.twoc", &test_words);
    }

    #[test]
    pub fn x_plus_y_is_z_sugar() {
        let test_words = [
            ("", true),
            ("xz", true),
            ("yyzz", true),
            ("xyzz", true),
            ("xyz", false),
            ("xyzzz", false),
            ("xxyyzzzz", true),
            ("xyzxyzzz", false),
        ];

        generic_test("./twocprogs/determ/x_plus_y_is_z_sugar.twoc", &test_words);
    }

    #[test]
    pub fn loops_forever() {
        let test_words = [
            ("", false),
            ("0", false),
            ("00", false),
            ("0000000", false),
            ("0000000000000000000000000000000", false),
            ("00000000000000000000000000000000", false),
            ("00000000000000000000000000000000000000000000000000000", false),
            ("00000000000000000000000000000000000000000000", false),
        ];

        generic_test("./twocprogs/determ/loops_forever.twoc", &test_words);
    }

    #[test]
    pub fn upower() {
        let test_words = [
            ("", false),
            ("0", true),
            ("00", true),
            ("000", false),
            ("0000", true),
            ("00000", false),
            ("000000", false),
            ("0000000", false),
            ("00000000", true),
            ("0000000000", false),
            ("0000000000000000", true),
            ("00000000000000000", false),
            ("000000000000000000", false),
            ("0000000000000000000", false),
            ("00000000000000000000", false),
            ("000000000000000000000", false),
            ("00000000000000000000000000000000", true),
            ("0000000000000000000000000000000000", false),
            ("0000000000000000000000000000", false),
            ("0000000000000000000000000000000000000000000000000000000000000000", true),
        ];

        generic_test("./twocprogs/determ/upower.twoc", &test_words);
    }

    // NONDETERMINISTIC PROGRAMS

    #[test]
    pub fn branch_and_while_choose() {
        let test_words = [
            ("", true),
            ("0", true),
            ("00", true),
            ("000", true),
            ("0000", true),
            ("00000", true),
            ("000000", true),
            ("0000000", true),
            ("00000000", true),
            ("000000000", true),
            ("0000000000", true),
        ];

        generic_test("./twocprogs/nondeterm/branch_and_while_choose.twoc", &test_words);
    }

    #[test]
    pub fn equal_or_double_zeros_ones() {
        let test_words = [
            // Equal
            ("", true),
            ("01", true),
            ("0011", true),
            ("000111", true),
            ("00001111", true),
            ("0000011111", true),
            ("000000111111", true),
            ("00000001111111", true),

            // Double
            ("100", true),
            ("110000", true),
            ("111000000", true),
            ("111100000000", true),
            ("111110000000000", true),
            ("111111000000000000", true),

            // False cases
            ("0", false),
            ("00011", false),
            ("0001111", false),
            ("0000011", false),
            ("00011111", false),
            ("0000000001111", false),
            ("0011110011101111", false),
        ];

        generic_test("./twocprogs/nondeterm/equal_or_double_zeros_ones.twoc", &test_words);
    }

    #[test]
    pub fn potential_loop() {
        let test_words = [
            ("", false),
            ("0", false),
            ("00", false),
            ("000", false),
            ("0000", false),
            ("00000", false),
            ("000000", false),
            ("0000000", false),
            ("00000000", false),
            ("000000000", false),
            ("0000000000", false),

            ("1", true),
            ("01", true),
            ("010", true),
            ("0010", true),
            ("011000", true),
            ("000100", true),
            ("00100100", true),
            ("00100000", true),
            ("000000100", true),
            ("0001000000", true),
            ("000000110000", true),
        ];

        generic_test("./twocprogs/nondeterm/potential_loop.twoc", &test_words);
    }

    #[test]
    pub fn while_choose() {
        let test_words = [
            ("1", true),
            ("10", true),
            ("100", true),
            ("10000000", true),
            ("00001", true),
            ("010", true),
            ("0010", true),
            ("011000", true),
            ("000100", true),
            ("00100100", true),
            ("00100000", true),
            ("000000100", true),
            ("0001000000", true),
            ("000000110000", true),
            

            ("", false),
            ("0", false),
            ("00", false),
            ("000", false),
            ("0000", false),
            ("00000", false),
            ("000000", false),
            ("0000000", false),
            ("00000000", false),
            ("000000000", false),
            ("0000000000", false),
        ];

        generic_test("./twocprogs/nondeterm/while_choose.twoc", &test_words);
    }
}