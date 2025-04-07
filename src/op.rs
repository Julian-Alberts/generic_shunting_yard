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
        Exponent,
    }

    impl crate::Operator for MathOperator {
        fn precedence(&self) -> usize {
            match self {
                MathOperator::Add | MathOperator::Sub => 11,
                MathOperator::Mul | MathOperator::Div => 12,
                Self::Exponent => 13,
            }
        }

        fn is_left_associative(&self) -> bool {
            *self != Self::Exponent
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

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Exponent;
    impl crate::Operator for Exponent {
        fn precedence(&self) -> usize {
            13
        }

        fn is_left_associative(&self) -> bool {
            false
        }
    }
}

#[allow(missing_docs)]
pub mod bool_logic {

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Lt;
    impl crate::Operator for Lt {
        fn precedence(&self) -> usize {
            9
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Le;
    impl crate::Operator for Le {
        fn precedence(&self) -> usize {
            9
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Ge;
    impl crate::Operator for Ge {
        fn precedence(&self) -> usize {
            9
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Gt;
    impl crate::Operator for Gt {
        fn precedence(&self) -> usize {
            9
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Eq;
    impl crate::Operator for Eq {
        fn precedence(&self) -> usize {
            8
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Ne;
    impl crate::Operator for Ne {
        fn precedence(&self) -> usize {
            8
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Xor;
    impl crate::Operator for Xor {
        fn precedence(&self) -> usize {
            6
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct And;
    impl crate::Operator for And {
        fn precedence(&self) -> usize {
            4
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Or;
    impl crate::Operator for Or {
        fn precedence(&self) -> usize {
            3
        }

        fn is_left_associative(&self) -> bool {
            true
        }
    }
}
