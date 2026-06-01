//! Statement code generation.

use super::expr::generate_expression;
use crate::CodegenError;
use xin_ast::Statement;

/// Generate code for a single statement.
pub fn generate_statement(stmt: &Statement) -> Result<String, CodegenError> {
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
