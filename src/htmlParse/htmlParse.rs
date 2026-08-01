use std::collections::HashMap;


pub struct HtmlParse {
    index: usize,
    pub html: html,
    htmls: Vec<*mut html>,
    html_code: String
}


#[derive(Debug)]
pub struct html {
    pub tag: String,
    pub properties: HashMap<String,String>,
    children: Vec<html>
}


impl HtmlParse {

    pub fn new(html_code: String) -> Self{
        HtmlParse{
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

    fn advance(&mut self) -> Option<char> {
        let chart = self.peek();
        if chart.is_some() {
            self.index += 1;
        }
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
        while let Some(chart) = self.peek()  {
            if (chart == ' ' || chart == '>' || chart == '=' || chart == '"') && self.index < self.html_code.len() {
                break;
            }
            current_label.push(chart);
            self.advance();
        }
        return current_label
    }

    fn get_properties(&mut self) -> HashMap<String,String> {
        let mut properties = HashMap::<String,String>::new();
        while let Some(chart) = self.peek() {
            if chart == '>' {
                break;
            }
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
        self.htmls.push(&mut self.html);
        let mut current: *mut html = &mut self.html;
        while self.peek().is_some()  {
            self.advance_space();
            if self.peek() == Some('<') {
                self.advance_space();
                self.advance();

                if self.peek() == Some('/') {
                    self.advance();
                    self.get_label();
                    self.advance_space();
                    self.advance();
                    self.advance_space();
                    self.htmls.pop();
                    if let Some(parent) = self.htmls.last() {
                        current = *parent;
                    }
                    continue;
                }

                // println!("{:?} {:?}", self.index,self.html_code.len());
                let tag = self.get_label();
                // unsafe { println!("{:#?} {:#?}", tag,(*current).tag); }
                self.advance_space();

                let properties = self.get_properties();

                let html = html {
                    tag,
                    properties,
                    children: Vec::new()
                };

                unsafe {
                    (*current).children.push(html);
                    current = (*current)
                        .children
                        .last_mut()
                        .expect("children is not empty: the node was just pushed");
                    self.htmls.push(current);
                }
            }
            self.advance_space();
            self.advance();

        }
    }
}
