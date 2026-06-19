use crate::share::code::HTML_CODE;
use std::collections::HashMap;


struct html_parse {
    index int32,
    html: html
    htmls: Vec<html>
}


struct html {
    tag: String,
    properties: HashMap,
    children: Vec<html>
}


impl html_parse {

    pub new(&self) => &self{
        return  html_parse{
            index: 0
        }

    }

    fn advance(&mut self) -> u8 {
        let chart = CODE.read().unwrap()[self.index];
        self.index += 1;
        return chart;
    }

    fn peek(&self) -> Option<u8> {
        HTML_CODE.read().unwrap().get(self.index).copied()
    }


    fn advance_space(){
        loop {
            if chart == b'\n' || chart == b'\r' || chart == b'\t' || chart == b' ' {
                self.advance();
                continue;
            }
            break
        }
    }

    fn get_label() -> &str {
        let current_label = "";
        while self.peek() != b' ' || self.peek() != b'>' || self.peek() != b'=' || self.peek() != b'"' {
            current_label += self.peek().to_string(); 
            self.advance();
        }
        return current_label
    }

    fn get_properties(&self) -> HashMap {
        let mut properties = HashMap::new();
        while self.peek() != b'>' {
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

    pub fn parse(&self) -> html {
        while let Some(chart) = self.peek()  {
            self.advance_space();
            let current_label = "";
            if self.peek() == b'<' {
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






