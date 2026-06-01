//! HIR types.

use xin_ast::BinaryOp;

/// HIR binary operator.
#[derive(Debug, Clone, Copy)]
pub enum HirBinaryOp {
    /// Arithmetic: +, -, *, /, %.
    Add,
    /// Subtract.
    Sub,
    /// Multiply.
    Mul,
    /// Divide.
    Div,
    /// Modulo.
    Mod,
    /// Equal.
    Eq,
    /// Not equal.
    Neq,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
    /// Bitwise AND.
    BitAnd,
    /// Bitwise OR.
    BitOr,
    /// Bitwise XOR.
    BitXor,
    /// Shift left.
    Shl,
    /// Shift right.
    Shr,
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
}

/// Convert binary operator to HIR.
pub fn binary_op_to_hir(op: &BinaryOp) -> HirBinaryOp {
    match op {
        BinaryOp::Add => HirBinaryOp::Add,
        BinaryOp::Sub => HirBinaryOp::Sub,
        BinaryOp::Mul => HirBinaryOp::Mul,
        BinaryOp::Div => HirBinaryOp::Div,
        BinaryOp::Mod => HirBinaryOp::Mod,
        BinaryOp::Eq => HirBinaryOp::Eq,
        BinaryOp::Neq => HirBinaryOp::Neq,
        BinaryOp::Lt => HirBinaryOp::Lt,
        BinaryOp::Le => HirBinaryOp::Le,
        BinaryOp::Gt => HirBinaryOp::Gt,
        BinaryOp::Ge => HirBinaryOp::Ge,
        BinaryOp::BitAnd => HirBinaryOp::BitAnd,
        BinaryOp::BitOr => HirBinaryOp::BitOr,
        BinaryOp::BitXor => HirBinaryOp::BitXor,
        BinaryOp::Shl => HirBinaryOp::Shl,
        BinaryOp::Shr => HirBinaryOp::Shr,
        BinaryOp::And => HirBinaryOp::And,
        BinaryOp::Or => HirBinaryOp::Or,
    }
}
