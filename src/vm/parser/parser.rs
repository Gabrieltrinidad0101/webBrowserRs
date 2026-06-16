use crate::vm::lexer::lexer::TOKEN;


struct NodeNumber {
    ln: usize,
    col: usize,
    value: String
}


struct BinOP {
    left: NodeNumber,
    rigth: NodeNumber,
    op: String
}


struct Parser {
    index: 0,
    tokens: vec<TOKEN>
}

impl Parser {

    fn new(&self,tokens: vec<TOKEN>){
        return Parser {
            index: 0,
            tokens
        }
    }

    pub fn parse(&self,tokens: vec<TOKEN>) {
        
    }


    fn advance(&self) -> TOKEN {
        let token = self.tokens[self.index];
        self.index++;
        return token;
    }

    fn peek(){
        return self.tokens[self.index];
    }


    fn binOP(&self){
        let mut binOP = BinOP{};
        binOP.left = self.term();
        binOP.op = self.advance();
        if(binOP.op.type_ == TOKEN::MUL) {
            binOP.op.rigth = self.binOP();
            return binOP;
        }
        binOP.rigth = self.term();
        return binOP;
    }

    fn term(&self, token: TOKEN){
        if token.type_ == "Number" {
            return NodeNumber {
                ln: token.ln,
                col: token.col,
                value: token.value.parse::<f64>().wrap()
            }
        }
        self.advance();
    }
}