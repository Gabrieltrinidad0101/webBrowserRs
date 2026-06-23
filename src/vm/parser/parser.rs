use crate::vm::lexer::lexer::TOKENS;
use crate::vm::lexer::lexer::Token;
use crate::vm::error::error::{CustomError,ErrorKind, error};


struct NodeNumber {
    ln: usize,
    col: usize,
    value: String
}

#[derive(Debug)]
pub enum Expr {
    BinOP {
        left: Box<Expr>,
        rigth: Box<Expr>,
        op: TOKENS,
    },
    Number {
        ln: usize,
        col: usize,
        value: String
    }
}


pub struct Parser {
    index: usize,
    tokens: Vec<Token>
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            index: 0,
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>,CustomError> {
        let expr = self.binOP();
        if self.peek().type_ != TOKENS::EOF {
            return Err(error(ErrorKind::IllegalSyntax,self.peek().ln,self.peek().col,self.peek().index,self.peek().value.len()));
        }
        return Ok(expr);
    }


    fn advance(&mut self) -> Token {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    fn peek(&self) -> Token {
        return self.tokens[self.index].clone();
    }


    fn binOP(&mut self) -> Box<Expr> {
        let mut left = self.term();
        while self.peek().type_ == TOKENS::PLUS {
            let op = self.advance().type_;
            let rigth = self.term();
            left = Box::new(Expr::BinOP{
                left,
                rigth,
                op
            });
        }
        return left
    }

    fn term(&mut self) -> Box<Expr> {
        let mut left = self.atom();
        while self.peek().type_ == TOKENS::MUL {
            let op = self.advance().type_;
            let rigth = self.atom();
            left = Box::new(Expr::BinOP{
                left,
                rigth,
                op
            });
        }
        return left;
    }

    fn atom(&mut self) -> Box<Expr> {
        let token = self.peek();
        if token.type_ == TOKENS::NUMBER {
            self.advance();
            return Box::new(Expr::Number {
                ln: token.ln,
                col: token.col,
                value: token.value
            })
        }
        return Box::new(Expr::Number{
            ln: 0,
            col: 0,
            value: "1".to_string()
        });
    }
}