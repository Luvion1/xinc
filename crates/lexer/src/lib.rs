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
