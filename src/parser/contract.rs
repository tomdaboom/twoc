use crate::parser::ast;

// Function to contract sequences of moves and increments in a sequence of statements
pub fn contract(program : &Vec<ast::Stmt>) -> Vec<ast::Stmt> {
    // Declare contracted statements vector
    let mut contracted : Vec<ast::Stmt> = Vec::new();

    // Declare vector to hold the current basic block
    let mut basic_block : Vec<ast::Stmt> = Vec::new();

    // Build up the current basic block
    for stmt in program {
        match stmt {
            // If the current statement is a move or an incr, add it to the basic block
            ast::Stmt::Move(_)  => basic_block.push(stmt.clone()),
            ast::Stmt::Incr(_)  => basic_block.push(stmt.clone()),

            // If the current statement is an accept, contract the basic block
            ast::Stmt::Accept() => { 
                // Contract the basic block
                if !basic_block.is_empty() {
                    contracted.append(&mut contract_basic_block(&basic_block));
                }

                // Push accept; and clear
                contracted.push(stmt.clone());
                basic_block = Vec::new();
            }

            // If the current statement is a reject, contract the basic block
            ast::Stmt::Reject() => { 
                // Contract the basic block
                if !basic_block.is_empty() {
                    contracted.append(&mut contract_basic_block(&basic_block));
                }

                // Push reject; and clear
                contracted.push(stmt.clone());
                basic_block = Vec::new();
            },

            // If the current statement is an if-else statement, contract recursively
            ast::Stmt::If(cond, if_block, else_block) => {
                // Contract the current basic block
                if !basic_block.is_empty() {
                    contracted.append(&mut contract_basic_block(&basic_block));
                }

                // Recurse on the if and else blocks
                let if_block_contr   = contract(&if_block);
                let else_block_contr = contract(&else_block);
                
                // Reform if statement and push
                let if_stmt_contr = ast::Stmt::If(cond.clone(), if_block_contr, else_block_contr);
                contracted.push(if_stmt_contr);

                // Clear basic block
                basic_block = Vec::new();
            },

            // If the current statement is a while statement, contract recursively
            ast::Stmt::While(cond, while_block) => {
                // Contract the current basic block
                if !basic_block.is_empty() {
                    contracted.append(&mut contract_basic_block(&basic_block));
                }

                // Recurse on the while block
                let while_block_contr = contract(&while_block);

                // Reform while statement and push
                let while_stmt_contr = ast::Stmt::While(cond.clone(), while_block_contr);
                contracted.push(while_stmt_contr);

                // Clear basic block
                basic_block = Vec::new();
            }

            // If the current statement is a branch statement, contract recursively
            ast::Stmt::Branch(branch_blocks) => {
                // Contract the current basic block
                if !basic_block.is_empty() {
                    contracted.append(&mut contract_basic_block(&basic_block));
                }

                // Contract each branch block
                let branch_blocks_contr : Vec<Vec<ast::Stmt>> = 
                    branch_blocks.into_iter()
                    .map(contract)
                    .collect();

                // Reform branch statement and push
                let branch_stmt_contr = ast::Stmt::Branch(branch_blocks_contr);
                contracted.push(branch_stmt_contr);

                // Clear basic block
                basic_block = Vec::new();
            }

            // Panic if you see a basic block
            ast::Stmt::BasicBlock(_ , _) => panic!("Basic block in uncontracted ast!"),
        }
    }

    // Add final basic block
    if !basic_block.is_empty() {
        contracted.append(&mut contract_basic_block(&basic_block));
    }

    // Return
    contracted
}  

// Function to contract a single basic block
fn contract_basic_block(basic_block : &Vec<ast::Stmt>) -> Vec<ast::Stmt> {
    // Declare contracted statements vector
    let mut contracted : Vec<ast::Stmt> = Vec::new();

    // Count the total move and the total incr
    let mut total_move = 0;
    let mut total_incr = 0;
    for stmt in basic_block {
        match stmt {
            ast::Stmt::Move(i) => total_move += i,
            ast::Stmt::Incr(j) => total_incr += j,
            _ => panic!("{:?} isn't a valid element of a basic block!", stmt),
        }
    }

    // TODO: see if it makes sense to wrap ifs around this
    //contracted.push(ast::Stmt::Move(total_move));
    //contracted.push(ast::Stmt::Incr(total_incr));

    contracted.push(ast::Stmt::BasicBlock(total_move, total_incr));

    // Return
    contracted
}