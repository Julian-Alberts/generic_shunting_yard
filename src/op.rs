//! This module contains predefined operators.
//! The precedence is based on the JavaScript definition.
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence

use crate::Operator;

macro_rules! new_op {
    ($ty: ty {$($pat: pat => ($prec: literal, $left: literal),)*} $(into $conv_ty: ident :: $conv_var:ident)?) => {
        impl Operator for $ty {
            fn precedence(&self) -> usize {
                #[allow(unused)]
                use $ty::*;
                match self {
                    $($pat => $prec,)*
                }
            }

            fn is_left_associative(&self) -> bool {
                #[allow(unused)]
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
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
    Exponent,
}

/// Common compare operators
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Compare {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,
}

/// Common logical operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[allow(missing_docs)]
pub enum Logical {
    Xor,
    And,
    Or,
}

/// All predefined operators
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[allow(missing_docs)]
pub enum All {
    Math(Math),
    Compare(Compare),
    Logical(Logical),
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
