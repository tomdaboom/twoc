# twoc

## About twoc

twoc is a programming language for [two-way one-counter automata](https://www.sciencedirect.com/science/article/pii/0304397582900871). Any program in twoc can be represented by an equivalent two-way one-counter automaton, and vice versa. twoc was designed to aid others in studying these automata, and at some point in the future will also comes equipped with a small Hoare-style proof system (twop) in order to help others demonstrate their power.

## Usage

1. Make sure that you have Rust installed on your machine with the Cargo build manager (follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))
2. Type ```cargo run -- --file <FILENAME> --word <INPUT_STRING>``` into the terminal to run a twoc program on a specified input. Use the ```--verbose``` flag to see the different transformations the program makes to the program. Use the ```--use-glueck-nondeterm``` and ```--use-rytter-matrix``` flags to change which algorithms are used to simulate the program (if the program is nondeterministic).  

## The codebase

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

- ```twoc/src/proofs``` *will* contain all of the code used to embed and check Hoare-logic proofs concerning these automata

If you wish to study the codebase, I recommend that you do so with VSCode after cloning the repo to your machine. I'd also recommend you install the following VSCode extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) (this will give you type annotations for the code, as well as lots of other useful language sevrer features for Rust)
- [LALRPOP Highlighting](https://marketplace.visualstudio.com/items?itemName=mnxn.lalrpop-highlight) (this will give you syntax highlighting for the .lalrpop grammars in ```twoc/src/parser```)

I also reccommend that you use Java's syntax highlighting for the programs in ```twoc/twocprogs```. In VSCode, you can turn this on by modifying the ```files.associations``` field in settings (```Ctrl + ,```). Add a new attribute with ```Item``` set to ```*.twoc``` and ```Value``` set to ```java```.
