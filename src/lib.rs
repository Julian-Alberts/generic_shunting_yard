#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::missing_safety_doc)]
#![warn(missing_docs)]

//! A generic Shunting yard algorithm implementation

use std::fmt::Debug;

pub mod op;

/// All valid input tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputToken<V, F, O> {
    /// A value inside of a expression. I.e. numbers or variables.
    Value(V),
    /// A left parenthesis i.e. "("
    LeftParen,
    /// A right parenthesis i.e. ")"
    RightParen,
    /// Any type of function
    Function(F),
    /// A seperator for function arguments
    ArgSeperator,
    /// A operator like "+", "-", ...
    Operator(O),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum StackToken<F, O> {
    LeftParen,
    Function(F),
    Operator(O),
}

/// All valid output tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutputToken<V, F, O> {
    /// A value inside of a expression. I.e. numbers or variables.
    Value(V),
    /// Any type of function
    Function(F),
    /// A operator like "+", "-", ...
    Operator(O),
}

/// Mark any struct or enum as an Operator. Each operator has to define its precedence and if it is
/// left associative.
pub trait Operator {
    /// Returns the precedence of an operator.
    fn precedence(&self) -> usize;
    /// Returns true if the operator is left associative.
    fn is_left_associative(&self) -> bool;
}

/// Convert a infix expression into a postfix expression.
pub fn to_postfix<V, F, O>(
    infix: impl IntoIterator<Item = InputToken<V, F, O>>,
) -> Vec<OutputToken<V, F, O>>
where
    O: Operator,
{
    let mut out_queue: Vec<OutputToken<V, F, O>> = Vec::new();
    let mut stack: Vec<StackToken<F, O>> = Vec::new();

    for token in infix.into_iter() {
        match token {
            InputToken::Value(value) => out_queue.push(OutputToken::Value(value)),
            InputToken::LeftParen => stack.push(StackToken::LeftParen),
            InputToken::RightParen => {
                while let Some(StackToken::Operator(_)) = stack.last() {
                    let Some(StackToken::Operator(op)) = stack.pop() else {
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    out_queue.push(OutputToken::Operator(op))
                }
                stack.pop();
                if let Some(StackToken::Function(_)) = stack.last() {
                    let Some(StackToken::Function(func)) = stack.pop() else {
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    out_queue.push(OutputToken::Function(func));
                }
            }
            InputToken::Function(func) => stack.push(StackToken::Function(func)),
            InputToken::ArgSeperator => {
                while let Some(StackToken::Operator(_)) = stack.last() {
                    let Some(StackToken::Operator(o)) = stack.pop() else {
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    out_queue.push(OutputToken::Operator(o))
                }
            }
            InputToken::Operator(o1) => {
                while let Some(StackToken::Operator(o2)) = stack.last() {
                    if o2.precedence() > o1.precedence()
                        || (o1.precedence() == o2.precedence() && o1.is_left_associative())
                    {
                        let Some(StackToken::Operator(o2)) = stack.pop() else {
                            unsafe { std::hint::unreachable_unchecked() }
                        };
                        out_queue.push(OutputToken::Operator(o2))
                    } else {
                        break;
                    }
                }
                stack.push(StackToken::Operator(o1));
            }
        }
    }
    for token in stack.into_iter().rev() {
        let out = match token {
            StackToken::LeftParen => todo!(),
            StackToken::Function(func) => OutputToken::Function(func),
            StackToken::Operator(o) => OutputToken::Operator(o),
        };
        out_queue.push(out);
    }
    out_queue
}

#[cfg(test)]
mod tests {
    use crate::{to_postfix, InputToken, OutputToken};

    #[test]
    fn value_only() {
        let post_fix = to_postfix([InputToken::<_, (), crate::op::math::MathOperator>::Value(1)]);
        assert_eq!(post_fix, vec![OutputToken::Value(1)]);
    }

    #[test]
    fn simple_addition() {
        let post_fix = to_postfix::<_, (), _>([
            InputToken::Value(1),
            InputToken::Operator(crate::op::math::MathOperator::Add),
            InputToken::Value(2),
        ]);
        assert_eq!(
            post_fix,
            vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Operator(crate::op::math::MathOperator::Add)
            ]
        );
    }

    #[test]
    fn precedence_0() {
        use crate::op::math::MathOperator;
        let post_fix = to_postfix::<_, (), _>([
            InputToken::Value(1),
            InputToken::Operator(MathOperator::Mul),
            InputToken::Value(2),
            InputToken::Operator(MathOperator::Add),
            InputToken::Value(3),
        ]);
        assert_eq!(
            post_fix,
            vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Operator(MathOperator::Mul),
                OutputToken::Value(3),
                OutputToken::Operator(MathOperator::Add)
            ]
        )
    }

    #[test]
    fn precedence_1() {
        use crate::op::math::MathOperator;
        let post_fix = to_postfix::<_, (), _>([
            InputToken::Value(1),
            InputToken::Operator(MathOperator::Add),
            InputToken::Value(2),
            InputToken::Operator(MathOperator::Mul),
            InputToken::Value(3),
        ]);
        assert_eq!(
            post_fix,
            vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Value(3),
                OutputToken::Operator(MathOperator::Mul),
                OutputToken::Operator(MathOperator::Add)
            ]
        )
    }

    #[test]
    fn wikipedia_example() {
        use crate::op::math::MathOperator;
        let post_fix = to_postfix([
            InputToken::Function("sin"),
            InputToken::LeftParen,
            InputToken::Function("max"),
            InputToken::LeftParen,
            InputToken::Value(2),
            InputToken::ArgSeperator,
            InputToken::Value(3),
            InputToken::RightParen,
            InputToken::Operator(MathOperator::Div),
            InputToken::Value(3),
            InputToken::Operator(MathOperator::Mul),
            InputToken::Value(4),
            InputToken::RightParen,
        ]);
        assert_eq!(
            post_fix,
            vec![
                OutputToken::Value(2),
                OutputToken::Value(3),
                OutputToken::Function("max"),
                OutputToken::Value(3),
                OutputToken::Operator(MathOperator::Div),
                OutputToken::Value(4),
                OutputToken::Operator(MathOperator::Mul),
                OutputToken::Function("sin"),
            ]
        )
    }
}
