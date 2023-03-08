# twoc/twop

## About twoc
twoc is a programming language for [two-way one-counter automata](https://dl.acm.org/doi/10.1145/193820.193835). Any program in twoc can be represented by an equivalent two-way one-counter automaton, and vice versa. Twoc was designed to aid others in studying these automata, and also comes equipped with a small Hoare-style proof system (twop) in order to help others demonstrate their power.

## Usage
### twoc
1. Make sure that you have Rust installed on your machine with the Cargo build manager
2. Type ```cargo run -- --file <FILENAME> --word <INPUT_STRING>``` into the terminal to run a twoc program on a specified input. Use the ```--verbose``` flag to see the different transformations the program makes to the program. 

### twop

## The codebase:
```twoc/src``` contains all of the program source files:
- ```twoc/src/parser``` contains all of the code concerning twoc's parsing stage. In here, you'll find:
    - twoc's grammar
    - twoc's abstract syntax tree
    - the code used to strip away any macros or syntactic sugar features
    - any other transformations applied to the abstract syntax tree before it's transformed into an automaton

- ```twoc/src/automaton``` contains all of the code concerning the representation of twoc programs as automaton. In here, you'll find:
    - structs to represent the transitions of the automaton's finite state control
    - a struct to represent the automata themselves
    - the algorithm used to convert twoc syntax trees to automata

- ```twoc/src/simulation``` contains all of the code used to check if a certain input string is accepted/rejected by a given automaton

- ```twoc/src/proofs``` contains all of the code used to embed and check Hoare-logic proofs concerning these automata