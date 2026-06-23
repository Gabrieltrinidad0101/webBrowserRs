use crate::vm::parser::parser::Expr;


#[derive(Debug)]
pub enum Command {
    command(String),
    value(i32),
}

#[derive(Debug)]
pub struct CodeGen{
    pub code: Vec<Command>
}


impl CodeGen {
    pub fn new() -> Self {
        return CodeGen{
            code: Vec::new()
        }
    }


    pub fn generate(&mut self,expr: Box<Expr>){
        match *expr {
            Expr::BinOP{ left, rigth, op } => {
                self.generate(rigth);
                self.generate(left);
                self.code.push(Command::command(format!("{:?}", op)));

            },
            Expr::Number{value, ..} => {
                self.code.push(Command::command(value));
            }
        }
    }


}