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
        let expr = self.binOP()?;
        if self.peek().type_ != TOKENS::EOF {
            return Err(error(ErrorKind::IllegalSyntax,self.peek().ln,self.peek().col,self.peek().index - 1,self.peek().value.len()));
        }
        return Ok(expr);
    }


    fn expect(&mut self, token: TOKENS) -> Result<Token, CustomError> {
        let t = self.peek();
        if t.type_ != token {
            return Err(error(
                ErrorKind::IllegalSyntax,
                t.ln,
                t.col,
                t.index - 1,
                t.value.len(),
            ));
        }
        Ok(self.advance())   
    }


    fn advance(&mut self) -> Token {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    fn peek(&self) -> Token {
        return self.tokens[self.index].clone();
    }

    fn binOP(&mut self) -> Result<Box<Expr>, CustomError> {
        let mut left = self.plusAndMinus()?;
        while self.peek().type_ == TOKENS::GTE || self.peek().type_ == TOKENS::GT {
            let op = self.advance().type_;
            let rigth = self.plusAndMinus()?;
            left = Box::new(Expr::BinOP{
                left,
                rigth,
                op
            });
        }
        return Ok(left)
    }

    fn plusAndMinus(&mut self) -> Result<Box<Expr>, CustomError> {
        let mut left = self.term()?;
        while self.peek().type_ == TOKENS::PLUS || self.peek().type_ == TOKENS::Minus {
            let op = self.advance().type_;
            let rigth = self.term()?;
            left = Box::new(Expr::BinOP{
                left,
                rigth,
                op
            });
        }
        return Ok(left)
    }

    fn term(&mut self) -> Result<Box<Expr>, CustomError> {
        let mut left = self.atom()?;
        while self.peek().type_ == TOKENS::MUL || self.peek().type_ == TOKENS::DIV {
            let op = self.advance().type_;
            let rigth = self.atom()?;
            left = Box::new(Expr::BinOP{
                left,
                rigth,
                op
            });
        }
        return Ok(left);
    }

    fn atom(&mut self) -> Result<Box<Expr>, CustomError> {
        let token = self.peek();

        if token.type_ == TOKENS::LPAREN {
            self.advance();
            let exp = self.binOP()?;
            self.expect(TOKENS::RPAREN)?;
            return Ok(exp);
        } 

        if token.type_ == TOKENS::NUMBER {
            self.advance();
            return Ok(Box::new(Expr::Number {
                ln: token.ln,
                col: token.col,
                value: token.value
            }))
        }
        return Err(error(
                ErrorKind::IllegalSyntax,
                token.ln,
                token.col,
                token.index - 1,
                token.value.len(),
            )); 
    }
}