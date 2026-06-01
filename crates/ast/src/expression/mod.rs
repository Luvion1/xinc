//! Expression AST nodes.
//!
//! All expression types in the AST.

/// Expression enum.
///
/// Every value-producing construct in the Xin language is represented as
/// an `Expression` node. Expressions are composable: a binary expression
/// holds two sub-expressions, a function call holds a callee expression
/// plus an argument list, and so on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// Literal value.
    Literal(Literal),
    /// Identifier reference.
    Identifier(String),
    /// Binary operation.
    Binary { left: Box<Self>, op: BinaryOp, right: Box<Self> },
    /// Unary operation.
    Unary { op: UnaryOp, operand: Box<Self> },
    /// Function call.
    Call { callee: Box<Self>, args: Vec<Self> },
    /// Ternary conditional: `cond ? then : else`.
    Ternary { cond: Box<Self>, then_expr: Box<Self>, else_expr: Box<Self> },
}

/// Literal value.
///
/// Numeric literals are kept as `String` to preserve the source form
/// (e.g. leading zeros, arbitrary-precision hints, separator underscores)
/// without committing to a concrete numeric type at parse time. The
/// semantic analysis stage is responsible for type conversion.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Literal {
    /// String literal.
    String(String),
    /// Number literal.
    Number(String),
    /// Boolean literal.
    Boolean(bool),
    /// Null literal.
    #[default]
    Null,
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Modulo.
    Mod,
    /// Equality.
    Eq,
    /// Inequality.
    Neq,
    /// Less than.
    Lt,
    /// Greater than.
    Gt,
    /// Left shift.
    Shl,
    /// Right shift.
    Shr,
    /// Bitwise AND.
    BitAnd,
    /// Bitwise OR.
    BitOr,
    /// Bitwise XOR.
    BitXor,
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Negation.
    Neg,
    /// Not.
    Not,
    /// Bitwise NOT.
    BitNot,
}

#[cfg(test)]
mod tests;
