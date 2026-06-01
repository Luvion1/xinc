//! Expression parsing.
//!
//! Parses expressions from token streams using precedence-climbing
//! for binary operators and recursive descent for atomic forms.

pub mod atom;
pub mod error;
pub mod parser;

#[cfg(test)]
mod tests;

pub use atom::parse_atom;
pub use error::ParserError;
pub use parser::{match_operator, parse_expression, parse_expression_from_tokens};