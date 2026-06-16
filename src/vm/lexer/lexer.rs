use std::sync::LazyLock;

use crate::share::code::CODE;
use crate::vm::error::error::{CustomError, ErrorKind, error};

#[derive(Debug)]
enum TOKENS {
    PLUS,
    MiNUS,
    MUL,
    DIV,
    NUMBER,
    EQ,
    LT,
    GT,
    LTE,
    GTE,
    EE
}

#[derive(Debug)]
pub struct Token {
    type_: TOKENS,
    value: String,
    ln: usize,
    col: usize,
    index: usize,
}

#[derive(Debug)]
pub struct Lexer {
    ln: usize,
    col: usize,
    index: usize,
    pub tokens: Vec<Token>
}

impl Lexer {

    pub fn new() -> Self {
        Lexer {
            ln: 1,
            col: 0,
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn advance(&mut self) -> u8 {
        let chart = CODE.read().unwrap()[self.index];
        self.col += 1;
        if chart == b'\n' {
            self.ln += 1;
            self.col = 0;
        }
        self.index += 1;
        return chart;
    }

    fn peek(&self) -> Option<u8> {
        CODE.read().unwrap().get(self.index).copied()
    }

    fn peekNext(&self) -> Option<u8> {
        CODE.read().unwrap().get(self.index +  1).copied()
    }


    pub fn tokenize(&mut self) -> Option<CustomError>{
        while let Some(chart) = self.peek() {
            let currentIndex = self.index;
            if chart == b'\n' || chart == b'\r' || chart == b'\t' || chart == b' ' {
                self.advance();
                continue;
            }
            if let Some(err) = self.numbers() {
                return Some(err);
            }

            self.one_character();

            if currentIndex == self.index {
                return Some(error(ErrorKind::IllegalCharacter, self.ln, self.col, self.index, 1));
            }
        }
        None
    }

    fn create_token(&mut self, value: &str,type_: TOKENS){
        self.tokens.push(Token{
            type_,
            value: value.to_string(),
            col: self.col,
            ln: self.ln,
            index: self.index,
        })
    }


    fn numbers(&mut self) -> Option<CustomError> {
        let mut dot = false;
        let mut number = String::new();
        while let Some(ch) = self.peek() {
            if ch == b'.' && dot {
                return Some(error(ErrorKind::Dot, self.ln, self.col, self.index, 1));
            }


            if ch == b'.' {
                dot = true;
            } else if !ch.is_ascii_digit() {
                break;
            }

            number.push(ch as char);
            self.advance();
        }


        if number != "" {
            self.create_token(&number,TOKENS::NUMBER);
        }
        None
    }


    fn one_character(&mut self) {
        let Some(ch) = self.peek() else { return };
        let token = match ch {
            b'+' => Some(TOKENS::PLUS),
            b'-' => Some(TOKENS::MiNUS),
            b'*' => Some(TOKENS::MUL),
            b'>' => Some(TOKENS::GT),
            b'<' => Some(TOKENS::LT),
            b'=' => Some(TOKENS::EQ),
            _ => None,
        };

        if let Some(t) = token {
            let next = self.peekNext();
            if next == Some(b'=') {
                let token2 = match t {
                    TOKENS::GT => Some(TOKENS::GTE),
                    TOKENS::LT => Some(TOKENS::LTE),
                    TOKENS::EQ => Some(TOKENS::EE),
                    _ => None,
                };
                if let Some(t2) = token2 {
                    self.create_token("",t2);
                    self.advance();
                    self.advance();
                    return
                } 
            }
            self.create_token("",t);
            self.advance();
        }
    }

}