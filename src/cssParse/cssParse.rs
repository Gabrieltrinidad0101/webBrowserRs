use std::collections::HashMap;

#[derive(Debug)]
pub struct Selector {
    pub type_: String,
    pub op: String,
    pub value: String
}

#[derive(Debug)]
pub struct Query {
    pub selector: Vec<Selector>,
    pub properties: HashMap<String,String>
}


pub struct CssParse {
    index: usize,
    code: String
}


impl CssParse {

    pub fn new(code: String) -> Self {
        CssParse {
            index: 0,
            code
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

    fn getSelector(&mut self) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = Vec::new();
        while self.peek().is_some() {
            if self.peek() == Some('{') {
                self.advance_space();
                return selectors;
            }
            let mut type_ = String::from("class");
            if self.peek() == Some('#') {
                type_ = String::from("id");
                self.advance();
            }
            let mut value = String::from(self.peek().unwrap());
            while self.peek().is_some() {

                let chart = self.advance();
                if chart == Some(' '){
                    break;
                }
                value.push_str(&chart.to_string());
            }
            let mut op = String::from("");
            if self.peek() == Some('>') {
                op = String::from(">");
                self.advance();
            }

            selectors.push(Selector{
                type_,
                op,
                value
            });
            print!("{:#?}",selectors);
        }
        return selectors; 
    }


    fn properties(&mut self) -> HashMap::<String,String>{
        let mut properties=  HashMap::<String,String>::new();
        loop {
            if self.peek() == Some('}') {
                self.advance_space();
                return properties;
            }
            let mut property = String::from("");
            while self.peek() != Some(' '){
                if self.peek() == Some(':') {
                    self.advance();
                    self.advance_space();
                    break;
                }
                if let Some(chart) = self.advance(){
                    property.push_str(&chart.to_string());
                }
            }

            let mut value = String::from("");
            while self.peek() != Some(' '){
                if self.peek() == Some(';') {
                    self.advance();
                    self.advance_space();
                    break;
                }
                if let Some(chart) = self.advance(){
                    value.push_str(&chart.to_string());
                }
            }
            properties.insert(property, value);

        }
    }

    pub fn parse(&mut self) -> Vec<Query>{
        let mut queries = Vec::<Query>::new();
        while self.peek().is_some()  {
            self.advance_space();
            let selector = self.getSelector();
            let properties = self.properties();
            queries.push(Query { selector, properties });
        }
        return queries;
    }

}
