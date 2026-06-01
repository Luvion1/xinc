//! Parser error types for the expression parser.
//!
//! Covers lexer errors propagated through `From`, plus syntax errors
//! specific to expression parsing such as unexpected end-of-input or
//! missing operands.

#[allow(unused_imports)]
use thiserror::Error;
use xin_lexer::LexerError;

/// Errors that can occur during expression parsing.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    /// Wraps a lexer error that occurred during tokenization.
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),

    /// Input source string was empty.
    #[error("Empty input")]
    EmptyInput,

    /// Encountered a token where an expression was expected.
    #[error("Expected expression")]
    ExpectedExpression,

    /// Reached end of token stream unexpectedly.
    #[error("Unexpected end of input")]
    UnexpectedEnd,

    /// Expected an identifier token.
    #[error("Expected identifier")]
    ExpectedIdentifier,

    /// Expected assignment operator `=`.
    #[error("Expected '='")]
    ExpectedAssignment,

    /// Expected statement terminator `;`.
    #[error("Expected ';'")]
    ExpectedSemicolon,

    /// Invalid or unrecognized type annotation.
    #[error("Invalid type")]
    InvalidType,

    /// Expected opening brace `{`.
    #[error("Expected '{{'")]
    ExpectedLBrace,
}