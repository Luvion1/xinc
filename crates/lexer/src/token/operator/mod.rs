//! Operator tokens and utility functions.
//!
//! Arithmetic, logical, comparison, and assignment operators.

mod enum_;
mod utils;

pub use enum_::Operator;
pub use utils::{is_binary, is_operator_char, precedence};

#[cfg(test)]
pub mod tests;
