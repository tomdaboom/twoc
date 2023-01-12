// AST nodes for statements 
#[derive(Debug, Clone)]
pub enum Stmt {
    // accept
    Accept(),

    // reject
    Reject(),

    // move(i)
    Move(i32),

    // c += j
    Incr(i32), 

    // (move(i), c += j), only present after contraction
    BasicBlock(i32, i32),

    // if-else
    If(Cond, Vec<Stmt>, Vec<Stmt>),

    // while
    While(Cond, Vec<Stmt>),

    // branch
    Branch(Vec<Vec<Stmt>>),
}

// AST nodes for conditions
#[derive(Debug, Clone)]
pub enum Cond {
    // read == X
    Read(Readable),

    // read != X
    NotRead(Readable),

    // c == 0
    CheckZero(),

    // c != 0
    CheckNotZero(),

    // X && Y
    And(Box<Cond>, Box<Cond>),

    // X || Y
    Or(Box<Cond>, Box<Cond>),

    // !X
    Not(Box<Cond>),
}

impl Cond {
    // Check that a condition is true given a certain character at the readhead and a certain counter value 
    pub fn check(&self, read : Readable, counter : i32) -> bool {
        match self {
            // Compare the character at the readhead to the character in the condition
            Cond::Read(char) => read == *char,
            Cond::NotRead(char) => read != *char,

            // Check the value of the counter
            Cond::CheckZero() => counter == 0,
            Cond::CheckNotZero() => counter != 0,

            // Recurse on and statements
            Cond::And(left, right) => left.check(read, counter) && right.check(read, counter),

            // Recurse on or statements
            Cond::Or(left, right) => left.check(read, counter) || right.check(read, counter),

            // Recurse on not statements
            Cond::Not(inner) => !inner.check(read, counter),
        }
    }
}

// Enum for things on the rhs of a read condition (either a character or lend/rend)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Readable { Char(char), LEnd(), REnd(), }

pub type Input = Vec<Readable>;

impl Readable {
    pub fn from_input_str(input : &str) -> Vec<Self> {
        let mut out_vector = Vec::new();

        out_vector.push(Self::LEnd());
        for c in input.chars() {
            out_vector.push(Self::Char(c));
        }
        out_vector.push(Self::REnd());

        out_vector
    }
}

impl Stmt {
    // AST Printer method
    pub fn print(&self, offset : usize) -> String {
        // Generate whitespace buffer
        let buffer = " ".repeat(offset);

        // Declare output string
        let mut out = "".to_owned();

        match self {
            // Print accept statement
            Stmt::Accept() => {
                out.push_str(&buffer);
                out.push_str("accept\n");
            },

            // Print reject statement
            Stmt::Reject() => {
                out.push_str(&buffer);
                out.push_str("reject\n");
            },

            // Print move statement
            Stmt::Move(move_by) => {
                out.push_str(&buffer);
                out.push_str(&format!("move({:?})\n", move_by));
            },

            // Print increment/decrement statement
            Stmt::Incr(incr_by) => {
                out.push_str(&buffer);
                out.push_str(&format!("c += {:?}\n", incr_by));
            },

            // Print basic block
            Stmt::BasicBlock(move_by, incr_by) => {
                out.push_str(&buffer);
                out.push_str(&format!("move({:?})\n", move_by));
                
                out.push_str(&buffer);
                out.push_str(&format!("c += {:?}\n", incr_by));
            },

            // Print if statement
            Stmt::If(cond, if_body, else_body) => {
                // Print if (condition printing just uses the debug trait for now)
                out.push_str(&buffer);
                out.push_str(&format!("if ({:?})\n", cond));

                // Print each statement in the if block
                for stmt in if_body.iter() {
                    let line = stmt.print(offset + 2);
                    out.push_str(&line);
                }

                // Do the same for the else block if it exists
                if !else_body.is_empty() {
                    out.push_str(&buffer);
                    out.push_str(&format!("else\n"));

                    for stmt in else_body.iter() {
                        let line = stmt.print(offset + 2);
                        out.push_str(&line);
                    } 
                }
            },

            // Print while block
            Stmt::While(cond, while_body) => {
                // Print while
                out.push_str(&buffer);
                out.push_str(&format!("while ({:?})\n", cond));

                // Print each statement in the while block
                for stmt in while_body.iter() {
                    let line = stmt.print(offset + 2);
                    out.push_str(&line);
                }
            },

            // Print branch block
            Stmt::Branch(blocks) => {
                // For each branch block
                for block in blocks.iter() {
                    // Print branch
                    out.push_str(&buffer);
                    out.push_str(&format!("branch\n"));

                    // Print each statement in the branch block
                    for stmt in block.iter() {
                        let line = stmt.print(offset + 2);
                        out.push_str(&line);
                    }
                }
            }
            
        }

        // Return
        out
    }
}


