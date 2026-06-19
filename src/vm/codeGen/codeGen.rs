use crate::vm::parser::parser::Expr;


struct Command {
    command: String,
    value: i32,
}

struct code_gen{
    expr: Expr
    code: Vec<Command>
}


impl code_gen {
    pub fn new() -> Self {
        return code_gen{
            expr
        }
    }

    generate(&mut self,expr: Expr){
        match expr {
            Expr::BinOP => {
                self.generate(expr.rigth);
                self.generate(expr.left);
                self.generate(expr.op);
            },
            Expr::BinOP => {
                self.generate(expr.rigth);
                self.generate(expr.left);
                self.generate(expr.op);
            },
            Expr::BinOP => {
                self.generate(expr.rigth);
                self.generate(expr.left);
                self.generate(expr.op);
            }
        }
    }


}