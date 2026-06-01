#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::unnested_or_patterns,
    clippy::redundant_closure_for_method_calls
)]

//! Xin lexer.
//!
//! Tokenizes Xin source code into a stream of [`Token`]s. The lexer is the
//! first stage of the compilation pipeline; its output feeds
//! [`xin_parser`](https://docs.rs/xin-parser).
//!
//! # Capabilities
//!
//! - Identifiers, keywords, literals (numbers, strings, characters, booleans, null).
//! - All [`crate::Operator`] variants (arithmetic, comparison, bitwise, logical).
//! - Punctuation: `( ) { } , ; :`.
//! - f-strings: `f"hello {name}!"` with arbitrary expressions inside braces.
//! - Raw strings and raw characters (`r"..."`, `r'x'`).
//! - Unicode escape sequences in strings and characters.
//! - Line and block comments.
//!
//! # Architecture
//!
//! The scanner is a cursor over the source bytes. It uses a
//! single-character `peek`/`advance` API and dispatches per leading
//! character to a category-specific parser under
//! [`crate::lexer::inner::scanner::parse`].
//!
//! # Example
//!
//! ```ignore
//! use xin_lexer::tokenize;
//! let tokens = tokenize("let x = 10;").unwrap();
//! assert!(tokens.len() > 1);
//! ```
//!
//! # Errors
//!
//! Errors are reported as [`LexerError`] variants. Common cases:
//! - [`LexerError::InvalidChar`] — character not part of Xin's grammar.
//! - [`LexerError::UnterminatedString`] / [`UnterminatedChar`] — missing closing quote.
//! - [`LexerError::UnterminatedComment`] — block comment without `*/`.
//! - [`LexerError::InvalidEscape`] / [`InvalidNumber`] — malformed literal.

mod diagnostics;
mod error;
mod lexer;
mod token;

pub use diagnostics::*;
pub use error::*;
pub use lexer::*;
pub use token::*;

#[cfg(test)]
mod tests;
