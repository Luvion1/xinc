//! Parsing submodules for lexical constructs.
//!
//! Each parser focuses on one lexical category: identifiers, numbers,
//! operators, punctuation, string/char literals, and f-string fragments.

pub mod fstring;
pub mod ident;
pub mod number;
pub mod operator;
pub mod punctuation;
pub mod string;

#[cfg(test)]
mod tests;

pub use operator::parse_operator_token;
pub use punctuation::parse_punctuation_token;
