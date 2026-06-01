//! Statement code generation.
//!
//! Currently only [`Statement::Let`] and [`Statement::Expr`] are
//! supported. Every other variant (function bodies, control flow,
//! assignments) returns [`CodegenError::InvalidStatement`].
//!
//! ## Output format
//!
//! - A `let` is rendered as `let <name> = <expr>;\n` where `<expr>` is
//!   the parenthesized form from [`super::expr::generate_expression`].
//!   Type annotations are dropped — the textual emitter works at the
//!   syntactic level, not the type level.
//! - An expression statement is rendered as `<expr>;\n`.
//!
//! ## Future direction
//!
//! When the IR pipeline lands, this module becomes the *last* stage
//! that touches the AST: the IR lowering passes consume [`Statement`]
//! once at the top, and subsequent stages operate on HIR/MIR/LIR.

use super::expr::generate_expression;
use crate::CodegenError;
use xin_ast::Statement;

/// Generate code for a single statement.
///
/// # Errors
///
/// [`CodegenError::InvalidStatement`] for any statement variant that
/// has not yet been wired up: `Fn`, `If`, `While`, `Return`, `Block`,
/// `Assign`.
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
