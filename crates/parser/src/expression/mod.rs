//! Expression parsing.
//!
//! Parses expressions from token streams.

pub mod parser;
#[cfg(test)]
mod tests;

use xin_ast::{Expression, Literal, BinaryOp};
use xin_lexer::{tokenize, TokenKind, LexerError};

pub use parser::{parse_expression, parse_expression_from_tokens, ParserError, match_operator};