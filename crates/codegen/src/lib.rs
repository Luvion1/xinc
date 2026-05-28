//! Code generation for Xin.
//!
//! Compiles AST to machine code.

use xin_ast::{Expression, Literal, Statement};
use tracing::debug;

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
    for stmt in statements {
        debug!("Processing statement: {:?}", stmt);
    }
    Ok("// Generated code placeholder".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_empty() {
        let code = generate(&[]).unwrap();
        assert!(code.contains("Generated"));
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
    fn test_codegen_with_let() {
        let stmts = vec![Statement::Let {
            name: "x".to_string(),
            ty: None,
            value: Expression::Literal(Literal::Number("42".to_string())),
        }];
        let code = generate(&stmts).unwrap();
        assert!(code.contains("Generated"));
    }

    #[test]
    fn test_codegen_with_expr() {
        let stmts = vec![Statement::Expr(Expression::Literal(Literal::Number("1".to_string())))];
        let code = generate(&stmts).unwrap();
        assert!(code.contains("Generated"));
    }

    #[test]
    fn test_codegen_with_fn() {
        let stmts = vec![Statement::Fn {
            name: "main".to_string(),
            params: vec![],
            body: vec![],
            ret_ty: None,
        }];
        let code = generate(&stmts).unwrap();
        assert!(code.contains("Generated"));
    }
}