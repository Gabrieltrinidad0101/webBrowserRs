mod share;
mod vm;

use crate::share::code::CODE;
use crate::vm::lexer::lexer::Lexer;
use crate::vm::parser::parser::Parser;
use crate::vm::codeGen::codeGen::CodeGen;

fn main() {
    *CODE.write().unwrap() = b"1 * 2 * 3 + 2 * 5".to_vec();
    let mut lexer = Lexer::new();
    if let Some(err) = lexer.tokenize() {
        println!("Error: {:#?}", err.kind);
        println!("description: {}", err.description);
        println!("{}", err.body);
        return;
    }
    let mut parser = Parser::new(lexer.tokens);
    match parser.parse() {
        Ok(ast) => {
            let mut codeGen = CodeGen::new();
            codeGen.generate(ast);
            println!("{:#?}",codeGen.code);
        },
        Err(e) => {
            println!("Error: {:#?}", e.kind);
            println!("description: {}", e.description);
            println!("{}", e.body);
        }
}
}


// push 10
// push 20
// add
