//! This module contains predefined operators.
//! The precedence is based on the JavaScript definition.
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence

use crate::Operator;

impl Operator for Box<dyn Operator> {
    fn precedence(&self) -> usize {
        self.as_ref().precedence()
    }

    fn is_left_associative(&self) -> bool {
        self.as_ref().is_left_associative()
    }
}

macro_rules! new_op {
    ($ty: ty {$($pat: pat => ($prec: literal, $left: literal),)*} $(into $conv_ty: ident :: $conv_var:ident)?) => {
        impl Operator for $ty {
            fn precedence(&self) -> usize {
                #[allow(unused, reason = "This import might not be used in the macro")]
                use $ty::*;
                match self {
                    $($pat => $prec,)*
                }
            }

            fn is_left_associative(&self) -> bool {
                #[allow(unused, reason = "This import might not be used in the macro")]
                use $ty::*;
                match self {
                    $($pat => $left,)*
                }
            }
        }

        $(
        impl From<$ty> for $conv_ty {
            fn from(value: $ty) -> Self {
                Self::$conv_var(value)
            }
        }
        )?
    };
}

/// Common math operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Math {
    /// The addition operator
    Add,
    /// The subtraction operator
    Sub,
    /// The multiplication operator
    Mul,
    /// The divison operator
    Div,
    /// The exponent operator
    Exponent,
}

/// Common compare operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Compare {
    /// The less than operator
    Lt,
    /// The less or equals operator
    Le,
    /// The equal operator
    Eq,
    /// The not equal operator
    Ne,
    /// The greater or equals operator
    Ge,
    /// The greater than operator
    Gt,
}

/// Common logical operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Logical {
    Xor,
    And,
    Or,
    Not,
}

/// All predefined operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum All {
    /// Math operators
    Math(Math),
    /// Compare operators
    Compare(Compare),
    /// Logicial operators
    Logical(Logical),
}

impl Operator for All {
    fn precedence(&self) -> usize {
        match self {
            All::Math(math) => math.precedence(),
            All::Compare(compare) => compare.precedence(),
            All::Logical(logical) => logical.precedence(),
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            All::Math(math) => math.is_left_associative(),
            All::Compare(compare) => compare.is_left_associative(),
            All::Logical(logical) => logical.is_left_associative(),
        }
    }
}

new_op!(Math {
    Add | Sub => (11, true),
    Mul | Div => (12, true),
    Exponent => (13, false),
} into All::Math);

new_op!(Compare {
    Lt | Le | Ge | Gt => (9, true),
    Eq | Ne => (8, true),
} into All::Compare);

new_op!(Logical {
    Xor => (6,true),
    And => (4, true),
    Or => (3, true),
    Not => (14, false),
} into All::Logical);

#[cfg(test)]
mod tests {
    #[test]
    fn convert_op_type() {
        let mut _op: super::All = super::Math::Div.into();
        _op = super::Logical::Or.into();
        _ = _op;
    }
}
