//! Xin lexical analyzer.
//!
//! Public API:
//! - `Lexer` struct
//! - `Token` and `TokenKind`
//! - `LexerError`
//! - `tokenize` convenience function

// Re-export core types for external use
pub use crate::error::LexerError;
pub use crate::token::Token;
#[allow(unused_imports)]
pub use crate::token::TokenKind;

// Internal modules
pub mod inner;
pub use inner::parser::Lexer;

/// Tokenize source code.
///
/// # Errors
/// Returns `LexerError` if lexical analysis fails (e.g., invalid character, unterminated string).
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    Lexer::new(source).tokenize()
}
