//! Operator tokens.
//! Arithmetic, logical, comparison, and assignment operators.

// Operators are grouped by precedence and associativity

/// All operator tokens in Xin.
///
/// This enum covers every operator token that the lexer
/// can produce. Operators may have one or more characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Arithmetic operators
    /// Addition: +
    Add,
    /// Subtraction: -
    Sub,
    /// Multiplication: *
    Mul,
    /// Division: /
    Div,
    /// Remainder: %
    Rem,
    // Comparison operators
    /// Equals: ==
    Eq,
    /// Not equals: !=
    Ne,
    /// Less than: <
    Lt,
    /// Greater than: >
    Gt,
    /// Less or equal: <=
    Le,
    /// Greater or equal: >=
    Ge,
    // Logical operators
    /// Logical AND: &&
    And,
    /// Logical OR: ||
    Or,
    /// Logical NOT: !
    Not,
    // Bitwise operators
    /// Bitwise AND: &
    BitAnd,
    /// Bitwise OR: |
    BitOr,
    /// Bitwise XOR: ^
    BitXor,
    /// Bitwise NOT: ~
    BitNot,
    // Shift operators
    /// Left shift: <<
    Shl,
    /// Right shift: >>
    Shr,
    // Assignment operators
    /// Assignment: =
    Assign,
    /// Addition assignment: +=
    AddAssign,
    /// Subtraction assignment: -=
    SubAssign,
    /// Multiplication assignment: *=
    MulAssign,
    /// Division assignment: /=
    DivAssign,
    /// Remainder assignment: %=
    RemAssign,
    // Compound assignment bitwise
    /// AND assignment: &=
    AndAssign,
    /// OR assignment: |=
    OrAssign,
    /// XOR assignment: ^=
    XorAssign,
    /// Left shift assignment: <<=
    ShlAssign,
    /// Right shift assignment: >>=
    ShrAssign,
    // Range operators
    /// Range inclusive: ..
    Range,
    /// Range exclusive: ..<
    RangeExclusive,
    // Other operators
    /// Arrow (function type): ->
    Arrow,
    /// Fat arrow (match arm): =>
    FatArrow,
    /// Concatenation: ~
    Concat,
    /// Null coalescing: ??
    Coalesce,
    /// Optional chaining: ?.
    OptionChain,
}

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
        '+' | '-'
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
        // Assignment (lowest)
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
        // Arrow is special
        Operator::Arrow | Operator::FatArrow => 2,
        // Logical OR
        Operator::Or => 3,
        // Logical AND
        Operator::And => 4,
        // Bitwise OR
        Operator::BitOr => 5,
        // Bitwise XOR
        Operator::BitXor => 6,
        // Bitwise AND
        Operator::BitAnd => 7,
        // Equality
        Operator::Eq | Operator::Ne => 8,
        // Comparison
        Operator::Lt | Operator::Gt | Operator::Le | Operator::Ge => 9,
        // Range operators (non-associative, but we give some precedence)
        Operator::Range | Operator::RangeExclusive => 10,
        // Shift
        Operator::Shl | Operator::Shr => 11,
        // Arithmetic
        Operator::Add | Operator::Sub => 12,
        Operator::Mul | Operator::Div | Operator::Rem => 13,
        // Unary operators (higher)
        Operator::Not | Operator::BitNot => 14,
        // Concatenation (highest for its category)
        Operator::Concat => 15,
        // Null coalescing
        Operator::Coalesce => 4,
        // Optional chaining
        Operator::OptionChain => 14,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator_char() {
        assert!(is_operator_char('+'));
        assert!(is_operator_char('='));
        assert!(!is_operator_char('a'));
    }

    #[test]
    fn test_precedence() {
        assert!(precedence(Operator::Add) > precedence(Operator::Assign));
        assert!(precedence(Operator::Mul) > precedence(Operator::Add));
        assert!(precedence(Operator::And) > precedence(Operator::Or));
    }

    #[test]
    fn test_is_binary() {
        assert!(is_binary(Operator::Add));
        assert!(!is_binary(Operator::Not));
    }
}
