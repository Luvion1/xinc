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

//! Xin parser.
//!
//! Converts a token stream from [`xin_lexer`] into the
//! [`xin_ast`](https://docs.rs/xin-ast) representation.
//!
//! # Entry points
//!
//! - [`parse_expression`] — parse a single expression from a source string.
//! - [`parse_statement`] — parse a list of statements from a source string.
//!
//! Both entry points call into the same underlying machinery:
//!
//! - [`expression::parser::parse_expr`] implements **precedence climbing**
//!   for binary operators. Operator precedence tiers (in tightest-to-loosest
//!   order): multiplicative (`* / %`), additive (`+ -`), shift (`<< >>`),
//!   comparison (`< >`), equality (`== !=`), bitwise AND, XOR, OR, logical
//!   AND, logical OR. All binary operators are left-associative.
//! - [`expression::atom::parse_atom`] handles atomic (non-recursive) forms:
//!   literals, identifiers, function calls, parenthesized sub-expressions,
//!   and unary `!`/`~`/`-`.
//! - [`statement`] handles top-level constructs: `let`, `fn`, `if`, `while`,
//!   `return`, blocks, assignments.
//!
//! # Errors
//!
//! All failures surface as [`ParserError`]. The most common variants:
//!
//! - [`ParserError::EmptyInput`] — source string is empty.
//! - [`ParserError::ExpectedExpression`] — token cannot start an expression.
//! - [`ParserError::ExpectedIdentifier`] / `ExpectedAssignment` /
//!   `ExpectedSemicolon` / `ExpectedLBrace` — statement-level syntax errors.
//! - [`ParserError::Lexer`] — wraps a [`crate::xin_lexer::LexerError`]
//!   surfaced from tokenization.
//!
//! # Example
//!
//! ```ignore
//! use xin_parser::parse_expression;
//! let expr = parse_expression("1 + 2 * 3").unwrap();
//! ```

pub mod expression;
pub mod statement;

pub use expression::parse_expression;
pub use statement::parse_statement;

/// Parser error types.
pub use expression::ParserError;
