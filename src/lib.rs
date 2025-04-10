//! A generic Shunting yard algorithm implementation
//! This crate only provides the shunting yard algorithm.
//! You need to write your own parser and convert its result to `InputToken`s
//! The types for values, functions and operators are generic. Operators must implement the
//! `Operator` trait.
//!
//! This crate contains definitions for some operators in the `op` module.
//!
//! ```rust
//! use generic_shunting_yard::{InputToken, OutputToken, op::Math, to_postfix};
//! // 5 + 2 * sin(123)
//! let infix = [
//!     InputToken::Value(5),
//!     InputToken::Operator(Math::Add),
//!     InputToken::Value(2),
//!     InputToken::Operator(Math::Mul),
//!     InputToken::Function("sin"),
//!     InputToken::LeftParen,
//!     InputToken::Value(123),
//!     InputToken::RightParen,
//! ];
//! let postfix = to_postfix(infix);
//! assert_eq!(postfix, Ok(vec![
//!     OutputToken::Value(5),
//!     OutputToken::Value(2),
//!     OutputToken::Value(123),
//!     OutputToken::Function("sin"),
//!     OutputToken::Operator(Math::Mul),
//!     OutputToken::Operator(Math::Add),
//! ]));
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::missing_safety_doc)]
#![warn(missing_docs)]

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
    LeftParen(usize),
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

/// This error is returned if the parentheses inside a expression do not match.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParenMissmatchError {
    pos: usize,
}

impl ParenMissmatchError {
    /// The error position
    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl std::fmt::Display for ParenMissmatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected parenthese at position {}", self.pos)
    }
}

/// Convert a infix expression into a postfix expression.
/// It is highly recomended to wrap function arguments in parentheses as the result may be
/// unexpected otherwise.
/// Example:
///
/// ```rust
/// use generic_shunting_yard::{InputToken, OutputToken, op::Math, to_postfix};
/// // 5 + 2 * sin(123)
/// let infix = [
///     InputToken::Value(5),
///     InputToken::Operator(Math::Add),
///     InputToken::Value(2),
///     InputToken::Operator(Math::Mul),
///     InputToken::Function("sin"),
///     InputToken::LeftParen,
///     InputToken::Value(123),
///     InputToken::RightParen,
/// ];
/// let postfix = to_postfix(infix);
/// assert_eq!(postfix, Ok(vec![
///     OutputToken::Value(5),
///     OutputToken::Value(2),
///     OutputToken::Value(123),
///     OutputToken::Function("sin"),
///     OutputToken::Operator(Math::Mul),
///     OutputToken::Operator(Math::Add),
/// ]));
/// ```
///
pub fn to_postfix<V, F, O>(
    infix: impl IntoIterator<Item = InputToken<V, F, O>>,
) -> Result<Vec<OutputToken<V, F, O>>, ParenMissmatchError>
where
    O: Operator,
{
    let mut out_queue: Vec<OutputToken<V, F, O>> = Vec::new();
    let mut stack: Vec<StackToken<F, O>> = Vec::new();
    let mut paren_count: isize = 0;

    for (pos, token) in infix.into_iter().enumerate() {
        match token {
            InputToken::Value(value) => out_queue.push(OutputToken::Value(value)),
            InputToken::LeftParen => {
                paren_count += 1;
                stack.push(StackToken::LeftParen(pos))
            }
            InputToken::RightParen if paren_count == 0 => return Err(ParenMissmatchError { pos }),
            InputToken::RightParen => {
                paren_count -= 1;
                while let Some(StackToken::Operator(_)) = stack.last() {
                    let Some(StackToken::Operator(op)) = stack.pop() else {
                        // SAFETY:
                        // This has been checked in the while condition
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    out_queue.push(OutputToken::Operator(op))
                }
                stack.pop();
                if let Some(StackToken::Function(_)) = stack.last() {
                    let Some(StackToken::Function(func)) = stack.pop() else {
                        // SAFETY:
                        // This has been checked in the if condition
                        unsafe { std::hint::unreachable_unchecked() }
                    };
                    out_queue.push(OutputToken::Function(func));
                }
            }
            InputToken::Function(func) => stack.push(StackToken::Function(func)),
            InputToken::ArgSeperator => {
                while let Some(StackToken::Operator(_)) = stack.last() {
                    let Some(StackToken::Operator(o)) = stack.pop() else {
                        // SAFETY:
                        // This has been checked in the while condition
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
                            // SAFETY:
                            // This has been checked in the while condition
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
            StackToken::LeftParen(pos) => return Err(ParenMissmatchError { pos }),
            StackToken::Function(func) => OutputToken::Function(func),
            StackToken::Operator(o) => OutputToken::Operator(o),
        };
        out_queue.push(out);
    }
    Ok(out_queue)
}

#[cfg(test)]
mod tests {
    use crate::{
        op::{Logical, Math},
        to_postfix, InputToken, OutputToken,
    };

    #[test]
    fn value_only() {
        let post_fix = to_postfix([InputToken::<_, (), Math>::Value(1)]);
        assert_eq!(post_fix, Ok(vec![OutputToken::Value(1)]));
    }

    #[test]
    fn simple_addition() {
        let post_fix = to_postfix::<_, (), _>([
            InputToken::Value(1),
            InputToken::Operator(Math::Add),
            InputToken::Value(2),
        ]);
        assert_eq!(
            post_fix,
            Ok(vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Operator(Math::Add)
            ])
        );
    }

    #[test]
    fn precedence_0() {
        let post_fix = to_postfix::<_, (), _>([
            InputToken::Value(1),
            InputToken::Operator(Math::Mul),
            InputToken::Value(2),
            InputToken::Operator(Math::Add),
            InputToken::Value(3),
        ]);
        assert_eq!(
            post_fix,
            Ok(vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Operator(Math::Mul),
                OutputToken::Value(3),
                OutputToken::Operator(Math::Add)
            ])
        )
    }

    #[test]
    fn precedence_1() {
        let post_fix = unsafe {
            to_postfix::<_, (), _>([
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(2),
                InputToken::Operator(Math::Mul),
                InputToken::Value(3),
            ])
        };
        assert_eq!(
            post_fix,
            Ok(vec![
                OutputToken::Value(1),
                OutputToken::Value(2),
                OutputToken::Value(3),
                OutputToken::Operator(Math::Mul),
                OutputToken::Operator(Math::Add)
            ])
        )
    }

    #[test]
    fn wikipedia_example() {
        let post_fix = to_postfix([
            InputToken::Function("sin"),
            InputToken::LeftParen,
            InputToken::Function("max"),
            InputToken::LeftParen,
            InputToken::Value(2),
            InputToken::ArgSeperator,
            InputToken::Value(3),
            InputToken::RightParen,
            InputToken::Operator(Math::Div),
            InputToken::Value(3),
            InputToken::Operator(Math::Mul),
            InputToken::Value(4),
            InputToken::RightParen,
        ]);
        assert_eq!(
            post_fix,
            Ok(vec![
                OutputToken::Value(2),
                OutputToken::Value(3),
                OutputToken::Function("max"),
                OutputToken::Value(3),
                OutputToken::Operator(Math::Div),
                OutputToken::Value(4),
                OutputToken::Operator(Math::Mul),
                OutputToken::Function("sin"),
            ])
        )
    }

    #[test]
    fn unary_operator_1() {
        // ! true
        let postfix = to_postfix([
            InputToken::<_, (), _>::Operator(Logical::Not),
            InputToken::Value(true),
        ]);
        assert_eq!(
            postfix,
            Ok(vec![
                OutputToken::Value(true),
                OutputToken::Operator(Logical::Not)
            ])
        )
    }

    #[test]
    fn unexpected_closing_paren() {
        let postfix = to_postfix([
            InputToken::Value(false),
            InputToken::RightParen,
            InputToken::Operator(Logical::And),
            InputToken::<_, (), _>::Operator(Logical::Not),
            InputToken::Value(true),
        ]);
        assert_eq!(postfix, Err(crate::ParenMissmatchError { pos: 1 }))
    }

    #[test]
    fn missing_closing_paren() {
        let postfix = to_postfix([
            InputToken::Value(false),
            InputToken::LeftParen,
            InputToken::Operator(Logical::And),
            InputToken::<_, (), _>::Operator(Logical::Not),
            InputToken::Value(true),
        ]);
        assert_eq!(postfix, Err(crate::ParenMissmatchError { pos: 1 }))
    }

    #[test]
    fn unary_operator_2() {
        // false && ! true
        let postfix = to_postfix([
            InputToken::Value(false),
            InputToken::Operator(Logical::And),
            InputToken::<_, (), _>::Operator(Logical::Not),
            InputToken::Value(true),
        ]);
        assert_eq!(
            postfix,
            Ok(vec![
                OutputToken::Value(false),
                OutputToken::Value(true),
                OutputToken::Operator(Logical::Not),
                OutputToken::Operator(Logical::And),
            ])
        )
    }

    #[test]
    fn function_call_without_paren() {
        let postfix1 = to_postfix([
            InputToken::<_, _, Math>::Function("fn"),
            InputToken::Value(1),
        ]);
        let postfix2 = to_postfix([
            InputToken::Function("fn"),
            InputToken::LeftParen,
            InputToken::Value(1),
            InputToken::RightParen,
        ]);
        assert_eq!(postfix1, postfix2);
    }

    #[test]
    fn function_call_without_paren_multi_arg() {
        let postfix1 = to_postfix([
            InputToken::<_, _, Math>::Function("fn"),
            InputToken::Value(1),
            InputToken::ArgSeperator,
            InputToken::Value(2),
            InputToken::Operator(Math::Add),
            InputToken::Value(2),
        ]);
        let postfix2 = to_postfix([
            InputToken::Function("fn"),
            InputToken::LeftParen,
            InputToken::Value(1),
            InputToken::ArgSeperator,
            InputToken::Value(2),
            InputToken::Operator(Math::Add),
            InputToken::Value(2),
            InputToken::RightParen,
        ]);
        assert_eq!(postfix1, postfix2);
    }

    #[test]
    fn function_call_without_paren_multi_arg_following_op() {
        // fn 1 , 2 + 2 == fn ( 1 , 2 + 2 )
        let postfix1 = to_postfix([
            InputToken::<_, _, Math>::Function("fn"),
            InputToken::Value(1),
            InputToken::ArgSeperator,
            InputToken::Value(2),
            InputToken::Operator(Math::Add),
            InputToken::Value(2),
        ]);
        // fn ( 1 , 2 ) + 2
        let postfix2 = to_postfix([
            InputToken::Function("fn"),
            InputToken::LeftParen,
            InputToken::Value(1),
            InputToken::ArgSeperator,
            InputToken::Value(2),
            InputToken::RightParen,
            InputToken::Operator(Math::Add),
            InputToken::Value(2),
        ]);
        assert_ne!(postfix1, postfix2);
    }
}
