use std::fmt::Debug;

use gyard::{InputToken, Operator, OutputToken};

#[derive(Debug, PartialEq)]
pub struct MathAdd;
#[derive(Debug, PartialEq)]
pub struct MathSub;
#[derive(Debug, PartialEq)]
pub struct MathMul;
#[derive(Debug, PartialEq)]
pub struct MathDiv;

impl gyard::Operator for MathAdd {
    fn precedence(&self) -> usize {
        11
    }

    fn is_left_associative(&self) -> bool {
        true
    }
}
impl Eval for MathAdd {
    fn eval(&self, value_stack: &mut Vec<f64>) {
        let a = value_stack.pop().unwrap();
        let b = value_stack.pop().unwrap();
        let r = a + b;
        value_stack.push(r);
    }
}

impl gyard::Operator for MathSub {
    fn precedence(&self) -> usize {
        11
    }

    fn is_left_associative(&self) -> bool {
        true
    }
}
impl Eval for MathSub {
    fn eval(&self, value_stack: &mut Vec<f64>) {
        let a = value_stack.pop().unwrap();
        let b = value_stack.pop().unwrap();
        let r = a - b;
        value_stack.push(r);
    }
}

impl gyard::Operator for MathMul {
    fn precedence(&self) -> usize {
        12
    }

    fn is_left_associative(&self) -> bool {
        true
    }
}
impl Eval for MathMul {
    fn eval(&self, value_stack: &mut Vec<f64>) {
        let a = value_stack.pop().unwrap();
        let b = value_stack.pop().unwrap();
        let r = a * b;
        value_stack.push(r);
    }
}

impl gyard::Operator for MathDiv {
    fn precedence(&self) -> usize {
        12
    }

    fn is_left_associative(&self) -> bool {
        true
    }
}
impl Eval for MathDiv {
    fn eval(&self, value_stack: &mut Vec<f64>) {
        let a = value_stack.pop().unwrap();
        let b = value_stack.pop().unwrap();
        let r = a / b;
        value_stack.push(r);
    }
}

trait Eval: Operator {
    fn eval(&self, value_stack: &mut Vec<f64>);
}

impl gyard::Operator for Box<dyn Eval> {
    fn precedence(&self) -> usize {
        self.as_ref().precedence()
    }

    fn is_left_associative(&self) -> bool {
        self.as_ref().is_left_associative()
    }
}
impl Eval for Box<dyn Eval> {
    fn eval(&self, value_stack: &mut Vec<f64>) {
        self.as_ref().eval(value_stack);
    }
}

fn main() {
    // 1 + 2 - f ( 3 ) * 4 / 5
    let infix = [
        InputToken::Value(1.),
        InputToken::Operator(Box::new(MathAdd) as Box<dyn Eval>),
        InputToken::Value(2.),
        InputToken::Operator(Box::new(MathSub)),
        InputToken::Function("f"),
        InputToken::LeftParen,
        InputToken::Value(3.),
        InputToken::RightParen,
        InputToken::Operator(Box::new(MathMul)),
        InputToken::Value(4.),
        InputToken::Operator(Box::new(MathDiv)),
        InputToken::Value(5.),
    ];
    let postfix = gyard::to_postfix(infix).unwrap();

    let mut value_stack = Vec::<f64>::new();
    for token in postfix {
        match token {
            OutputToken::Function("f") => {
                let v = value_stack.pop().unwrap();
                value_stack.push(v * 2. + 1.)
            }
            OutputToken::Function(_) => unimplemented!(),
            OutputToken::Value(v) => value_stack.push(v),
            OutputToken::Operator(o) => o.eval(&mut value_stack),
        }
    }
    assert_eq!(value_stack.len(), 1);
    assert_eq!(value_stack[0], -2.6)
}
