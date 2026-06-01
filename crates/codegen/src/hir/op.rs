//! HIR unary operators.

use xin_ast::UnaryOp;

/// HIR unary operator.
#[derive(Debug, Clone, Copy)]
pub enum HirUnaryOp {
    /// Negation.
    Neg,
    /// Logical NOT.
    Not,
    /// Bitwise NOT.
    BitNot,
}

/// Convert unary operator to HIR.
pub fn unary_op_to_hir(op: &UnaryOp) -> HirUnaryOp {
    match op {
        UnaryOp::Neg => HirUnaryOp::Neg,
        UnaryOp::Not => HirUnaryOp::Not,
        UnaryOp::BitNot => HirUnaryOp::BitNot,
    }
}
