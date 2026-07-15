use crate::vm::codeGen::codeGen::OpCommand;

pub struct VM {
    pub stack: Vec<f64>,
    code: Vec<OpCommand>,
    ip: usize,
}

impl VM {
    pub fn new(code: Vec<OpCommand>) -> Self {
        VM {
            code,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn fetch(&mut self) -> OpCommand {
        let command = self.code[self.ip].clone();
        self.ip += 1;
        return command;
    }

    pub fn run(&mut self) {
        loop {
            let command = self.fetch();
            match command {
                OpCommand::Value(v) => {
                    self.stack.push(v);
                }
                OpCommand::Command(cmd) => match cmd.as_str() {
                    "PLUS" => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a + b);
                    }
                    "SUB" => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a - b);
                    }
                    "MUL" => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a * b);
                    }
                    "DIV" => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(b / a);
                    }
                    "GTE" => {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(if b > a { 1.0 } else { 0.0 });
                    }
                    "HALT" => break,
                    _ => panic!("comando desconocido: {}", cmd),
                },
            }
        }
    }
}
