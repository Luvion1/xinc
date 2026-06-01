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
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
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

use std::fmt;

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{lit}"),
            Expression::Identifier(name) => write!(f, "{name}"),
            Expression::Binary { left, op, right } => write!(f, "({} {} {})", &**left, op, &**right),
            Expression::Unary { op, operand } => match op {
                UnaryOp::Neg => write!(f, "-{operand}"),
                UnaryOp::Not => write!(f, "!{operand}"),
                UnaryOp::BitNot => write!(f, "~{operand}"),
            },
            Expression::Call { callee, args } => {
                let args_str = args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "{callee}({args_str})")
            }
            Expression::Ternary { cond, then_expr, else_expr } => {
                write!(f, "{} ? {} : {}", cond, then_expr, else_expr)
            }
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Neq => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests;
