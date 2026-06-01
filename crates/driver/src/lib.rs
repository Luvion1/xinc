#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::missing_docs_in_private_items)]

//! Xin compiler pipeline driver.
//!
//! Orchestrates the full compilation pipeline:
//! 1. Lexing / tokenization
//! 2. Parsing to AST
//! 3. Semantic analysis
//! 4. Code generation
//!
//! # Example
//! ```ignore
//! use xin_driver::compile;
//! let result = compile("let x = 42;");
//! ```

use thiserror::Error;

/// Compilation pipeline errors.
#[derive(Debug, Error)]
pub enum DriverError {
    /// Lexical analysis error.
    #[error("Lexical error: {0}")]
    Lexical(#[from] xin_lexer::LexerError),

    /// Parse error.
    #[error("Parse error: {0}")]
    Parse(#[from] xin_parser::ParserError),

    /// Semantic analysis error.
    #[error("Semantic error: {0}")]
    Semantic(#[from] xin_semantic::SemanticError),

    /// Code generation error.
    #[error("Codegen error: {0}")]
    Codegen(#[from] xin_codegen::CodegenError),

    /// Compilation failed without specific error.
    #[error("Compilation failed")]
    Failed,
}

/// Compile Xin source code into output string.
///
/// # Errors
/// Returns `DriverError` if any stage of the pipeline fails.
pub fn compile(source: &str) -> Result<String, DriverError> {
    let tokens = xin_lexer::tokenize(source)?;
    tracing::debug!("Tokenized {} tokens", tokens.len());

    let stmts = xin_parser::parse_statement(source)?;
    tracing::debug!("Parsed {} statements", stmts.len());

    let mut analyzer = xin_semantic::Analyzer::new();
    for stmt in &stmts {
        if let Err(e) = analyzer.analyze(stmt) {
            tracing::error!("Semantic error: {}", e);
            return Err(DriverError::Semantic(e));
        }
    }
    tracing::debug!("Semantic analysis passed");

    let output = xin_codegen::generate(&stmts)?;
    tracing::debug!("Generated output ({} bytes)", output.len());

    Ok(output)
}

/// Tokenize only (no full compilation).
///
/// # Errors
/// Returns `LexerError` on invalid input.
pub fn tokenize(source: &str) -> Result<Vec<xin_lexer::Token>, DriverError> {
    Ok(xin_lexer::tokenize(source)?)
}

/// Parse only (no codegen).
///
/// # Errors
/// Returns `DriverError` on lexer or parser failure.
pub fn parse(source: &str) -> Result<Vec<xin_ast::Statement>, DriverError> {
    let _tokens = xin_lexer::tokenize(source)?;
    Ok(xin_parser::parse_statement(source)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_let() {
        let result = compile("let x = 42;");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_binary_expr() {
        let result = compile("let x = 1 + 2;");
        assert!(result.is_ok(), "compile failed: {:?}", result.err());
    }

    #[test]
    fn test_compile_invalid_syntax() {
        let result = compile("let @@@");
        assert!(result.is_err());
    }
}
