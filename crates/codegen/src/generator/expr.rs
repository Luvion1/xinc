//! Expression code generation.

use crate::CodegenError;
use xin_ast::{BinaryOp, Expression, Literal, UnaryOp};

/// Generate code for an expression.
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
        BinaryOp::Gt => ">",
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
