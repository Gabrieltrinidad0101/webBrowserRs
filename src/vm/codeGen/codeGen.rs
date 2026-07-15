use crate::vm::parser::parser::Expr;


#[derive(Debug,Clone)]
pub enum OpCommand {
    Command(String),
    Value(f64),
}

#[derive(Debug)]
pub struct CodeGen{
    pub code: Vec<OpCommand>
}


impl CodeGen {
    pub fn new() -> Self {
        return CodeGen{
            code: Vec::new()
        }
    }

    fn generate_(&mut self,expr: Box<Expr>){
        match *expr {
            Expr::BinOP{ left, rigth, op } => {
                self.generate_(rigth);
                self.generate_(left);
                self.code.push(OpCommand::Command(format!("{:?}", op)));

            },
            Expr::Number{value, ..} => {
                let num: f64 = value.parse().unwrap();
                self.code.push(OpCommand::Value(num));
            }
        }
    }


    pub fn generate(&mut self,expr: Box<Expr>){
        self.generate_(expr);
        self.code.push(OpCommand::Command("HALT".to_string()));
    }


}