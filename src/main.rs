mod render;
mod share;
mod vm;
mod htmlParse;
mod cssParse;

use crate::share::code::CODE;
use crate::vm::lexer::lexer::Lexer;
use crate::vm::parser::parser::Parser;
use crate::vm::codeGen::codeGen::CodeGen;
use crate::vm::vm::vm::VM;
use crate::htmlParse::htmlParse::HtmlParse;
use crate::cssParse::cssParse::CssParse;

fn main() {
   let mut htmlParse = HtmlParse::new(r#"
    <h2>
        <h2></h2>
        <h3></h3>
    </h2>
    <h2></h2>
    <h3></h3>
    <h3>
        <h2></h2>
        <h3></h3>
    </h3>
    <h3></h3>
    <h3></h3>
    <h3></h3>
   "#.to_string());
   htmlParse.parse();

    let mut cssParse = CssParse::new(r#"
    .h1 {
        width: 10px;
        heigth: 10px;
        background: red;
    }
    "#.to_string());
   let css = cssParse.parse();
   println!("{:#?}", htmlParse.html);
   println!("{:#?}", css);
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
