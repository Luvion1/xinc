//! Xin lexer crate.
//!
//! Tokenizes Xin source code.
//!
//! # Example
//! ```ignore
//! use xin_lexer::tokenize;
//! let tokens = tokenize("let x = 10;").unwrap();
//! ```

mod lexer;
mod token;
mod error;
mod diagnostics;

pub use lexer::*;
pub use token::*;
pub use error::*;
pub use diagnostics::*;