use crate::expr::ExprParts;

pub struct Engine {
    stack: Vec<ExprParts>,
    top: usize
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            stack: Vec::new(),
            top: 0usize
        }
    }

    pub fn eval(&mut self, expr: &[ExprParts]) -> ExprParts {
        let mut i = expr.len() - 1;
        loop {
            if !expr[i].is_op() {
                self.push(expr[i]);
            }

            else {
                match expr[i] {
                    ExprParts::AddOp => {
                        let s1 = self.pop();
                        let s2 = self.pop();
                        self.push(s1 + s2);
                    }

                    ExprParts::SubOp => {
                        let s1 = self.pop();
                        let s2 = self.pop();
                        self.push(s2 - s1);
                    }

                    ExprParts::MulOp => {
                        let s1 = self.pop();
                        let s2 = self.pop();
                        self.push(s1 * s2);
                    }

                    ExprParts::DivOp => {
                        let s1 = self.pop();
                        let s2 = self.pop();
                        self.push(s1 / s2);
                    }
                    _ => unreachable!() // not possible
                }
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }
        self.pop()
    }

    fn pop(&mut self) -> ExprParts {
        //self.top -= 1;
        match self.stack.pop() {
            Some(s) => s,
            None => unreachable!("popping from empty stack")
        }
    }

    fn push(&mut self, arg: ExprParts) {
        self.top += 1;
        self.stack.push(arg);
    }
}