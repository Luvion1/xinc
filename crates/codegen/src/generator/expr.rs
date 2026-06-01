//! Expression code generation.
//!
//! Walks an [`Expression`] and produces a parenthesized infix
//! representation. The output is plain source text — no SSA, no
//! registers, no instruction set. The IR layers ([`crate::hir`],
//! [`crate::mir`], [`crate::lir`]) are independent and consume the
//! HIR, not the [`Expression`] tree.
//!
//! # Output format
//!
//! Each [`Expression::Binary`] is rendered as `(left op right)` with
//! the operator symbol from [`binary_op_str`]. Each [`Expression::Unary`]
//! is `op<operand>` (no parentheses — the operator char disambiguates
//! from a binary application). Literals, identifiers, and calls
//! follow their natural Rust-like forms.
//!
//! # Errors
//!
//! Returns [`CodegenError::InvalidStatement`] when a [`Expression::Call`]
//! has a non-identifier callee — call expressions with a function-valued
//! expression as the callee are not yet supported.

use crate::CodegenError;
use xin_ast::{BinaryOp, Expression, Literal, UnaryOp};

/// Generate code for an expression.
///
/// Recursively walks the tree. The traversal order is left-to-right,
/// depth-first; the binary form is rendered as `(left op right)`.
pub fn generate_expression(expr: &Expression) -> Result<String, CodegenError> {
    match expr {
        Expression::Literal(lit) => generate_literal(lit),
        Expression::Identifier(name) => Ok(name.clone()),
        Expression::Binary { left, op, right } => {
            let left_code = generate_expression(left)?;
            let right_code = generate_expression(right)?;
            let op_str = binary_op_str(op);
            Ok(format!("({} {} {})", left_code, op_str, right_code))
        }
        Expression::Unary { op, operand } => {
            let op_str = unary_op_str(op);
            let operand_code = generate_expression(operand)?;
            Ok(format!("{}{}", op_str, operand_code))
        }
        Expression::Call { callee, args } => {
            let callee_str = match callee.as_ref() {
                Expression::Identifier(name) => name.clone(),
                _ => return Err(CodegenError::InvalidStatement),
            };
            let args_code: Vec<String> =
                args.iter().map(generate_expression).collect::<Result<Vec<_>, _>>()?;
            Ok(format!("{}({})", callee_str, args_code.join(", ")))
        }
        Expression::Ternary { cond, then_expr, else_expr } => {
            let cond_code = generate_expression(cond)?;
            let then_code = generate_expression(then_expr)?;
            let else_code = generate_expression(else_expr)?;
            Ok(format!("({}? {}: {})", cond_code, then_code, else_code))
        }
    }
}

/// Generate code for a literal.
pub fn generate_literal(lit: &Literal) -> Result<String, CodegenError> {
    Ok(match lit {
        Literal::Number(n) => n.clone(),
        Literal::String(s) => format!("\"{}\"", s),
        Literal::Boolean(b) => b.to_string(),
        Literal::Null => "null".to_string(),
    })
}

fn binary_op_str(op: &BinaryOp) -> &'static str {
    match op {
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
    }
}

fn unary_op_str(op: &UnaryOp) -> &'static str {
    match op {
        UnaryOp::Neg => "-",
        UnaryOp::Not => "!",
        UnaryOp::BitNot => "~",
    }
}
