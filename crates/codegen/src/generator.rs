//! Code generation implementation.
//!
//! Generates code from AST nodes.

use tracing::debug;
use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

/// Code generator error.
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    /// Invalid statement.
    #[error("Invalid statement for codegen")]
    InvalidStatement,
}

/// Generate code from statements.
pub fn generate(statements: &[Statement]) -> Result<String, CodegenError> {
    debug!("Starting code generation");
    let mut output = String::new();
    for stmt in statements {
        debug!("Processing statement: {:?}", stmt);
        match generate_statement(stmt) {
            Ok(code) => output.push_str(&code),
            Err(e) => return Err(e),
        }
    }
    Ok(output)
}

/// Generate code for a single statement.
fn generate_statement(stmt: &Statement) -> Result<String, CodegenError> {
    match stmt {
        Statement::Let { name, ty: _, value } => {
            let val_code = generate_expression(value)?;
            Ok(format!("let {} = {};\n", name, val_code))
        }
        Statement::Expr(expr) => {
            let code = generate_expression(expr)?;
            Ok(format!("{};\n", code))
        }
        _ => Err(CodegenError::InvalidStatement),
    }
}

/// Generate code for an expression.
fn generate_expression(expr: &Expression) -> Result<String, CodegenError> {
    match expr {
        Expression::Literal(lit) => generate_literal(lit),
        Expression::Identifier(name) => Ok(name.clone()),
        Expression::Binary { left, op, right } => {
            let left_code = generate_expression(left)?;
            let right_code = generate_expression(right)?;
            let op_str = match op {
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
            };
            Ok(format!("({} {} {})", left_code, op_str, right_code))
        }
        Expression::Unary { op, operand } => {
            let op_str = match op {
                UnaryOp::Neg => "-",
                UnaryOp::Not => "!",
                UnaryOp::BitNot => "~",
            };
            let operand_code = generate_expression(operand)?;
            Ok(format!("{}{}", op_str, operand_code))
        }
        Expression::Call { callee, args } => {
            let callee_code = generate_expression(callee)?;
            let args_code: Vec<String> =
                args.iter().map(generate_expression).collect::<Result<Vec<_>, _>>()?;
            Ok(format!("{}({})", callee_code, args_code.join(", ")))
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
fn generate_literal(lit: &Literal) -> Result<String, CodegenError> {
    match lit {
        Literal::Number(n) => Ok(n.clone()),
        Literal::String(s) => Ok(format!("\"{}\"", s)),
        Literal::Boolean(b) => Ok(b.to_string()),
        Literal::Null => Ok("null".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_empty() {
        let code = generate(&[]).unwrap();
        assert!(code.contains("Generated") || code.is_empty());
    }

    #[test]
    fn test_codegen_error_debug() {
        let err = CodegenError::InvalidStatement;
        assert!(format!("{:?}", err).contains("Invalid"));
    }

    #[test]
    fn test_codegen_error_display() {
        let err = CodegenError::InvalidStatement;
        assert_eq!(format!("{}", err), "Invalid statement for codegen");
    }

    #[test]
    fn test_codegen_literal() {
        let stmt = Statement::Expr(Expression::Literal(Literal::Number("42".to_string())));
        let code = generate(&[stmt]).unwrap();
        assert!(code.contains("42"));
    }

    #[test]
    fn test_codegen_binary() {
        let stmt = Statement::Expr(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        });
        let code = generate(&[stmt]).unwrap();
        assert!(code.contains("(1 + 2)"));
    }

    #[test]
    fn test_codegen_unary() {
        let stmt = Statement::Expr(Expression::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
        });
        let code = generate(&[stmt]).unwrap();
        assert!(code.contains("-5"));
    }

    #[test]
    fn test_codegen_bitwise() {
        let stmt = Statement::Expr(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            op: BinaryOp::BitAnd,
            right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        });
        let code = generate(&[stmt]).unwrap();
        assert!(code.contains("(1 & 2)"));
    }

    #[test]
    fn test_codegen_shift() {
        let stmt = Statement::Expr(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("8".to_string()))),
            op: BinaryOp::Shr,
            right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        });
        let code = generate(&[stmt]).unwrap();
        assert!(code.contains("(8 >> 2)"));
    }
}
