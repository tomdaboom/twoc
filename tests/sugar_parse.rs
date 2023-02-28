// Import grammar
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar_rules, "/parser/sugar/sugar_grammar.rs");

#[cfg(test)]
mod sugar_parse {
    use std::fs;
    use crate::grammar_rules::TwocParser;
    use twoc::parser::sugar::convert_sugar::convert_sugar;
    use twoc::automaton::determ_construction::construct_from_prog;
    //use twoc::simulation::glueck::glueck_procedure;

    #[test]
    fn test() {
        // Declare parser for Twoc rule
        let parser = TwocParser::new();

        let file_path = "./twocprogs/sugar/petersen_2.twoc";

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

        // Desugar AST
        let mut desugared_prog = convert_sugar(prog);
        desugared_prog.contract();

        println!("\nDesugared AST:");
        desugared_prog.print();

        // Construct automaton 
        let autom = construct_from_prog(desugared_prog);

        println!("\nAutomaton:");
        autom.print();

        /*
        let test_words = [
            "00001111",
            "0001111", 
            "01010101",
            "",
        ];

        // Print test case outputs
        println!("");
        for word in test_words {
            print!("{:?} ", word);
            if glueck_procedure(&autom, word) {
                println!("is accepted");
            } else {
                println!("is not accepted");
            }
        }
        */

        panic!("panic to show stdout; don't worry about me hoo hoo hee hee")
    }
}