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

//! Xin lexer crate.
//!
//! Tokenizes Xin source code.
//!
//! # Example
//! ```ignore
//! use xin_lexer::tokenize;
//! let tokens = tokenize("let x = 10;").unwrap();
//! ```

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
