//! Expression parsing.
//!
//! Parses expressions from token streams.

pub mod parser;
#[cfg(test)]
mod tests;

pub use parser::{ParserError, match_operator, parse_expression, parse_expression_from_tokens};
