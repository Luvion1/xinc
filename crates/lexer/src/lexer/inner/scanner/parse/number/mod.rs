//! Numeric literal parsing.
//!
//! Splits number parsing by radix: hexadecimal, binary, octal, and decimal
//! (the latter also covers floating-point forms with fractional and
//! exponent parts). Each sub-module owns one radix-specific grammar.

mod binary;
mod decimal;
mod hex;
mod octal;

#[cfg(test)]
mod tests;

use super::super::Scanner;
use crate::error::LexerError;

/// Parse a numeric literal starting at the current scanner position.
///
/// Recognizes decimal, hexadecimal (`0x`), binary (`0b`), octal (`0o`)
/// integers, and floating-point numbers with optional fractional and
/// exponent parts. Underscores act as digit separators.
///
/// Returns the raw lexical form (including prefix and underscores) as a
/// `String`, or an `InvalidNumber` error if the literal is malformed.
pub fn parse_number(scanner: &mut Scanner) -> Result<String, LexerError> {
    if scanner.current_char() != Some('0') {
        return decimal::parse_decimal(scanner);
    }

    match scanner.peek() {
        Some('x') | Some('X') => hex::parse_hexadecimal(scanner),
        Some('b') | Some('B') => binary::parse_binary(scanner),
        Some('o') | Some('O') => octal::parse_octal(scanner),
        _ => {
            scanner.advance();
            Ok("0".to_string())
        }
    }
}
