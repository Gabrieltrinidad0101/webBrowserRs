mod error;
mod lexer;
mod share;

use crate::share::code::CODE;
use crate::lexer::lexer::Lexer;

fn main() {
    *CODE.write().unwrap() = b"1.43 + 1.43 - >=  = == <= > 1..2".to_vec();
    let mut lexer = Lexer::new();
    if let Some(err) = lexer.tokenize() {
        println!("Error: {:#?}", err.kind);
        println!("description: {}", err.description);
        println!("{}", err.body);
        return;
    }
    println!("{:#?}",lexer.tokens);
}