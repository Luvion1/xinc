//! Parsing submodules for lexical constructs.
//!
//! Each module handles a specific category of tokens:
//! - `ident`: identifiers and keyword detection
//! - `number`: numeric literals (integer/float, all radices)
//! - `string`: string and character literals (normal, raw, f-string fragments)
//! - `symbols`: operators and punctuation
//! - `fstring`: f-string interpolation state and fragment parsing

pub mod fstring;
pub mod ident;
pub mod number;
pub mod string;
pub mod symbols;
