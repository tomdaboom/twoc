// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod sugar_parse {
    use std::fs;
    use crate::grammar_rules::TwocParser;

    #[test]
    fn test() {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        let file_path = "./twocprogs/sugar/sugar_test.twoc";

        println!("Parsing {:?}\n", file_path);

        // Load file
        let test_prog = fs::read_to_string(file_path).expect("File not found");

        // Parse the file
        let test = parser.parse(&test_prog);
        let prog = match test {
            // Output any parse errors
            Err(ref err) => panic!("Parse Error:\n{:?}", err),
            Ok(prog) => prog,
        };

        // Print AST
        println!("\nAST:");
        prog.print();
    }
}