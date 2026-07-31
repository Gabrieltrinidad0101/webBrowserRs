use std::collections::HashMap;

#[derive(Debug)]
pub struct Operation {
    pub type_: String,
    pub op: String,
    pub value: String
}

#[derive(Debug)]
pub struct Rule {
    pub selector: Vec<Operation>,
    pub properties: HashMap<String,String>
}


pub struct CssParse {
    index: usize,
    code: String,
    pub rules: Vec<Rule>
}


impl CssParse {

    pub fn new(code: String) -> Self {
        CssParse {
            index: 0,
            code,
            rules: Vec::new()
        }
    }

    fn peek(&self) -> Option<char> {
        self.code.chars().nth(self.index)
    }

    fn advance(&mut self) -> Option<char> {
        let chart = self.peek();
        self.index += 1;
        return chart;
    }

    fn advance_space(&mut self){
        loop {
            let chart = self.peek();
            if chart == Some('\n') || chart == Some('\r') || chart == Some('\t') || chart == Some(' ') {
                self.advance();
                continue;
            }
            break;
        }
    }

    fn getQuery(&mut self) -> Vec<Operation> {
        let mut ops: Vec<Operation> = Vec::new();
        loop {
            if self.peek() == Some('{') {
                return ops;
            }
            let mut type_ = String::from("class");
            if self.peek() == Some('#') {
                type_ = String::from("id");
                self.advance();
            }
            let mut value = String::from("");
            while self.peek() == Some(' '){
                if let Some(chart) = self.advance(){
                    value.push_str(&chart.to_string());
                }
            }
            let mut op = String::from("");
            if self.peek() == Some('>') {
                op = String::from(">");
                self.advance();
            }

            ops.push(Operation{
                type_,
                op,
                value
            });
        } 
    }

    pub fn parse(&mut self){
        loop {
            self.advance_space();
        }
    }

}
