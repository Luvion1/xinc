//! Xin parser crate.
//!
//! Parses tokens into AST.

pub mod expression;
pub mod statement;

pub use expression::parse_expression;
pub use statement::parse_statement;

/// Parser error types.
pub use expression::ParserError;