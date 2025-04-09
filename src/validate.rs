use crate::InputToken;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Token {
    Value,
    LeftParen,
    RightParen,
    Function,
    ArgSeperator,
    Operator,
}

impl<V, F, O> From<&InputToken<V, F, O>> for Token {
    fn from(value: &InputToken<V, F, O>) -> Self {
        match value {
            InputToken::Value(_) => Self::Value,
            InputToken::LeftParen => Self::LeftParen,
            InputToken::RightParen => Self::RightParen,
            InputToken::Function(_) => Self::Function,
            InputToken::ArgSeperator => Self::ArgSeperator,
            InputToken::Operator(_) => Self::Operator,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InvalidTokenError<'a, V, F, O> {
    found: &'a InputToken<V, F, O>,
    expected: &'static [Token],
    pos: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Error<'a, V, F, O> {
    InvalidToken(InvalidTokenError<'a, V, F, O>),
    ParenMissMatch(isize),
}

impl<'a, V, F, O> From<InvalidTokenError<'a, V, F, O>> for Error<'a, V, F, O> {
    fn from(value: InvalidTokenError<'a, V, F, O>) -> Self {
        Error::InvalidToken(value)
    }
}

struct ValidationContext {
    function_level: usize,
    paren_count: isize,
    function_local_paren_count: Vec<usize>,
}

impl ValidationContext {
    fn new() -> Self {
        Self { function_level: 0, paren_count: 0, function_local_paren_count: vec![0] }
    }
    fn enter_fn_args(&mut self) {
        self.function_level += 1;
        self.paren_count += 1;
        self.function_local_paren_count.push(1);
    }
    fn left_paren(&mut self) {
        self.paren_count += 1;
        debug_assert!(self.function_local_paren_count.len() >= 1);
        *unsafe { self.function_local_paren_count.last_mut().unwrap_unchecked() } += 1;
    }
    fn right_paren(&mut self) {
        self.paren_count -= 1;
        debug_assert!(self.function_local_paren_count.len() >= 1);
        let fn_local_count = unsafe { self.function_local_paren_count.last_mut().unwrap_unchecked() };
        *fn_local_count -= 1;
        if *fn_local_count == 0 {
            self.function_local_paren_count.pop();
        }
    }
    fn allow_end_of_fn_arg(&self) -> bool {
        let fn_local_count = *unsafe { self.function_local_paren_count.last().unwrap_unchecked() };
        self.function_level > 0 && fn_local_count == 1
    }
}

pub fn validate<'a, V, F, O>(
    tokens: impl Iterator<Item = &'a InputToken<V, F, O>>,
) -> Result<(), Error<'a, V, F, O>> {
    let mut state: &dyn ValidationState = &Expression;
    let mut ctx = ValidationContext::new();
    for (pos, in_token) in tokens.enumerate() {
        let token = in_token.into();
        state = state
            .validate(token, &mut ctx)
            .map_err(|expected| InvalidTokenError {
                found: in_token,
                expected,
                pos,
            })?;
    }
    if ctx.paren_count != 0 {
        return Err(Error::ParenMissMatch(ctx.paren_count));
    }
    Ok(())
}

macro_rules! new_val_state {
    (
        $ty: ty {$($pat: ident $(if $cond:expr)? => $new_state: expr,)*} $ctx: ident
    ) => {
        impl ValidationState for $ty {
            fn validate<'a>(
                &self,
                token: Token,
                #[allow(unused)]
                $ctx: &mut ValidationContext,
            ) -> Result<&'static dyn ValidationState, &'static [Token]> {
                use Token::*;
                match token {
                    $($pat $(if $cond)? => Ok(&$new_state),)*
                    _ => Err(&[$($pat,)*])
                }
            }
        }
    };
}

trait ValidationState {
    fn validate<'a>(
        &self,
        token: Token,
        stack: &mut ValidationContext,
    ) -> Result<&'static dyn ValidationState, &'static [Token]>;
}

//
// expr := Value after_value
// after_value := Op expr
//

struct Expression;
new_val_state!(Expression { 
    Value => AfterValue,
    LeftParen => { ctx.left_paren(); Expression },
    Function => FunctionArgsStart,
} ctx);

struct AfterValue;
new_val_state!(AfterValue { 
    Operator => Expression,
    RightParen => { ctx.right_paren(); AfterValue},
    ArgSeperator if ctx.allow_end_of_fn_arg() => Expression,
} ctx);

struct FunctionArgsStart;
new_val_state!(FunctionArgsStart {
    LeftParen => { ctx.enter_fn_args(); Expression },
} ctx);


mod tests {
    use crate::{InputToken, op::Math};

    use super::{Error, InvalidTokenError, Token, validate};

    fn mostly_eq<'a, V, F, O>(e1: &Error<'a, V, F, O>, e2: &Error<'a, V, F, O>) -> bool
    where
        V: std::cmp::PartialEq,
        F: std::cmp::PartialEq,
        O: std::cmp::PartialEq,
    {
        match (e1, e2) {
            (Error::InvalidToken(e1), Error::InvalidToken(e2))
                if e1.found == e2.found && e1.pos == e2.pos =>
            {
                true
            }
            (Error::ParenMissMatch(e1), Error::ParenMissMatch(e2)) if e1 == e2 => true,
            _ => false,
        }
    }

    #[test]
    fn validate_expressions() {
        let inputs: &[&[InputToken<_, &str, Math>]] = &[
            // 1
            &[InputToken::Value(1)],
            // 1 + 1
            &[
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(1),
            ],
            // 1 + 1 + 1
            &[
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(1),
            ],
            // 1 + ( 1 + 1 )
            &[
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::LeftParen,
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(1),
                InputToken::RightParen,
            ],
            // 1 + sin ( 1 + 1 )
            &[
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Function("sin"),
                InputToken::LeftParen,
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Value(1),
                InputToken::RightParen,
            ],
            // 1 + f ( 1 , 1 )
            &[
                InputToken::Value(1),
                InputToken::Operator(Math::Add),
                InputToken::Function("f"),
                InputToken::LeftParen,
                InputToken::Value(1),
                InputToken::ArgSeperator,
                InputToken::Value(1),
                InputToken::RightParen,
            ],
        ];
        inputs
            .into_iter()
            .enumerate()
            .for_each(|(id, infix)| assert_eq!(validate(infix.iter()), Ok(()), "Test {id} failed"));
    }

    #[test]
    fn invalidate_expressions() {
        let inputs: &[(&[InputToken<_, &str, Math>], Error<_, _, _>)] = &[
            (
                // +
                &[InputToken::Operator(Math::Add)],
                InvalidTokenError {
                    found: &InputToken::Operator(Math::Add),
                    expected: &[],
                    pos: 0,
                }
                .into(),
            ),
            (
                // 12 13
                &[InputToken::Value(12), InputToken::Value(13)],
                InvalidTokenError {
                    found: &InputToken::Value(13),
                    expected: &[],
                    pos: 1,
                }
                .into(),
            ),
            (
                // )
                &[InputToken::RightParen],
                InvalidTokenError {
                    found: &InputToken::RightParen,
                    expected: &[],
                    pos: 0,
                }
                .into(),
            ),
            (
                // (
                &[InputToken::LeftParen],
                Error::ParenMissMatch(1),
            ),
            (
                // (((1)
                &[
                    InputToken::LeftParen,
                    InputToken::LeftParen,
                    InputToken::LeftParen,
                    InputToken::Value(1),
                    InputToken::RightParen,
                ],
                Error::ParenMissMatch(2),
            ),
            (
                // f ( ( 1 , 2 ) )
                &[
                    InputToken::Function("f"),
                    InputToken::LeftParen,
                    InputToken::LeftParen,
                    InputToken::Value(1),
                    InputToken::ArgSeperator,
                    InputToken::Value(2),
                    InputToken::RightParen,
                    InputToken::RightParen,
                ],
                Error::InvalidToken(InvalidTokenError { 
                    found: &InputToken::ArgSeperator, 
                    expected: &[], 
                    pos: 4 
                }),
            ),
        ];
        inputs.into_iter().enumerate().for_each(|(id, (infix, e))| {
            let res = validate(infix.iter());
            assert!(res.is_err(), "Test {id} failed successfully");
            let res = res.unwrap_err();
            // I dont care about `InvalidTokenError::expected`
            assert!(mostly_eq(&res, e), "Test {id} failed {:?} {e:?}", res)
        });
    }
}
