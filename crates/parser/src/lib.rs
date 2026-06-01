#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::unnested_or_patterns,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::equatable_if_let
)]

//! Xin parser crate.
//!
//! Parses tokens into AST.

pub mod expression;
pub mod statement;

pub use expression::parse_expression;
pub use statement::parse_statement;

/// Parser error types.
pub use expression::ParserError;
