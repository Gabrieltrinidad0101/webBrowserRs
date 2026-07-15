mod render;
mod share;
mod vm;

use crate::share::code::CODE;
use crate::vm::lexer::lexer::Lexer;
use crate::vm::parser::parser::Parser;
use crate::vm::codeGen::codeGen::CodeGen;
use crate::vm::vm::vm::VM;

fn main() {
    render::render::run();
}

#[allow(dead_code)]
fn vm_demo() {
    *CODE.write().unwrap() = b"25 >= 10 + 2".to_vec();
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
            println!("{:#?}", ast);
            codeGen.generate(ast);
            let mut vm = VM::new(codeGen.code);
            vm.run();
            println!("{:#?}", vm.stack);
        },
        Err(e) => {
            println!("Error: {:#?}", e.kind);
            println!("description: {}", e.description);
            println!("{}", e.body);
        }
    }

}
