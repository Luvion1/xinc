//! Operator utility functions.

use crate::token::Operator;

/// Check if a character can start an operator.
///
/// # Arguments
/// * `c` - Character to check
///
/// # Returns
/// True if it might be part of an operator
pub fn is_operator_char(c: char) -> bool {
    matches!(
        c,
        '+'
            | '-'
            | '*'
            | '/'
            | '%'
            | '='
            | '!'
            | '<'
            | '>'
            | '&'
            | '|'
            | '^'
            | '~'
            | ':'
            | '.'
            | '?'
    )
}

/// Get operator precedence (higher number = tighter binding).
///
/// Precedence levels follow common programming language conventions.
///
/// # Arguments
/// * `op` - The operator
///
/// # Returns
/// Precedence level (1-15)
pub fn precedence(op: Operator) -> u8 {
    match op {
        Operator::Assign
        | Operator::AddAssign
        | Operator::SubAssign
        | Operator::MulAssign
        | Operator::DivAssign
        | Operator::RemAssign
        | Operator::AndAssign
        | Operator::OrAssign
        | Operator::XorAssign
        | Operator::ShlAssign
        | Operator::ShrAssign => 1,
        Operator::Arrow | Operator::FatArrow => 2,
        Operator::Or => 3,
        Operator::BitOr => 5,
        Operator::BitXor => 6,
        Operator::BitAnd => 7,
        Operator::Eq | Operator::Ne => 8,
        Operator::Lt | Operator::Gt | Operator::Le | Operator::Ge => 9,
        Operator::Range | Operator::RangeExclusive => 10,
        Operator::Shl | Operator::Shr => 11,
        Operator::Add | Operator::Sub => 12,
        Operator::Mul | Operator::Div | Operator::Rem => 13,
        Operator::Not | Operator::BitNot | Operator::OptionChain => 14,
        Operator::And | Operator::Coalesce => 4,
        Operator::Concat => 15,
    }
}

/// Check if operator is binary.
///
/// Binary operators take two operands.
///
/// # Arguments
/// * `op` - The operator
///
/// # Returns
/// True if binary
pub fn is_binary(op: Operator) -> bool {
    !matches!(op, Operator::Not | Operator::BitNot)
}
