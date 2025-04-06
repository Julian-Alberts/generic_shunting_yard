//! This module contains predefined operators.
//! The precedence is based on the JavaScript definition.
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence

/// Common math operators
#[allow(missing_docs)]
pub mod math {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum MathOperator {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl crate::Operator for MathOperator {
        fn precedence(&self) -> usize {
            match self {
                MathOperator::Add | MathOperator::Sub => 11,
                MathOperator::Mul | MathOperator::Div => 12,
            }
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Add;
    impl crate::Operator for Add {
        fn precedence(&self) -> usize {
            11
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Sub;
    impl crate::Operator for Sub {
        fn precedence(&self) -> usize {
            11
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Mult;
    impl crate::Operator for Mult {
        fn precedence(&self) -> usize {
            12
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Div;
    impl crate::Operator for Div {
        fn precedence(&self) -> usize {
            12
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }
}
