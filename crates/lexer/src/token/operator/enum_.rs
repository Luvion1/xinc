//! Operator enum definitions.

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
