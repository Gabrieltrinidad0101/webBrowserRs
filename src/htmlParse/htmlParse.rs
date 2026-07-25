use std::collections::HashMap;


pub struct HtmlParse {
    index: usize,
    pub html: html,
    htmls: Vec<html>,
    html_code: String
}


#[derive(Debug)]
pub struct html {
    tag: String,
    properties: HashMap<String,String>,
    children: Vec<html>
}


impl HtmlParse {
    
    pub fn new(html_code: String) -> Self{
        return  HtmlParse{
            index: 0,
            html_code,
            htmls: Vec::new(),
            html: html {
                tag: "root".to_string(),
                properties: HashMap::new(),
                children: Vec::new()
            }
        }
    }

    fn advance(&mut self) -> u8 {
        let chart = self.html_code.as_bytes()[self.index];
        self.index += 1;
        return chart;
    }

    fn peek(&self) -> Option<char> {
        self.html_code.chars().nth(self.index)
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

    fn get_label(&mut self) -> String {
        let mut current_label = String::new();
        while let Some(chart) = self.peek() {
            if chart == ' ' || chart == '>' || chart == '=' || chart == '"' {
                break;
            }
            current_label.push(chart);
            self.advance();
        }
        return current_label
    }

    fn get_properties(&mut self) -> HashMap<String,String> {
        let mut properties = HashMap::<String,String>::new();
        while self.peek() != Some('>') {
            let property = self.get_label();
            self.advance_space();
            self.advance(); // to-do
            self.advance_space();
            self.advance(); // to-do
            let value = self.get_label();
            self.advance(); // to-do
            properties.insert(property,value);
        }
        return properties;
    }

    pub fn parse(&mut self) {
        while let Some(chart) = self.peek()  {
            self.advance_space();
            let mut current_label = String::new();
            if self.peek() == Some('<') {
                self.advance_space();
                current_label = self.get_label();
                self.advance_space();
                let properties = self.get_properties();
            }
            self.advance_space();
            self.advance();
        }
    }
}






