//! Code generation implementation.
//!
//! Generates code from AST nodes.

mod expr;
mod stmt;
#[cfg(test)]
mod tests;

pub use expr::generate_expression;
pub use stmt::generate_statement;

use crate::CodegenError;
use tracing::debug;
use xin_ast::Statement;

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
