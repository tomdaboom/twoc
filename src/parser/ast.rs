// AST nodes for statements 
#[derive(Debug)]
pub enum Stmt {
    // accept
    Accept(),

    // reject
    Reject(),

    // move(i)
    Move(i32),

    // c += j
    Incr(i32), 

    // if-else
    If(Cond, Vec<Stmt>, Vec<Stmt>),

    // while
    While(Cond, Vec<Stmt>),

    // branch
    Branch(Vec<Vec<Stmt>>),
}

// AST nodes for conditions
#[derive(Debug)]
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

// Enum for things on the rhs of a read condition (either a character or lend/rend)
#[derive(Debug)]
pub enum Readable { Char(char), LEnd(), REnd(), }

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
