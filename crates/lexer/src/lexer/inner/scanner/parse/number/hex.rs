//! Hexadecimal integer literal parsing.
//!
//! Consumes the `0x` prefix and all subsequent hex digits (and `_`
//! separators). Requires at least one digit after the prefix.

use super::super::super::Scanner;
use crate::error::LexerError;

/// Parse a `0x` hexadecimal integer literal.
///
/// Consumes the `0x` prefix and any hex digits or `_` separators that
/// follow. Returns `InvalidNumber` if no digits appear after the prefix.
pub fn parse_hexadecimal(scanner: &mut Scanner) -> Result<String, LexerError> {
    scanner.advance(); // '0'
    scanner.advance(); // 'x'
    let mut value = String::from("0x");
    let mut count = 0;
    while let Some(c) = scanner.current_char() {
        if c.is_ascii_hexdigit() || c == '_' {
            value.push(scanner.advance().unwrap_or('\0'));
            count += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return Err(LexerError::InvalidNumber {
            reason: "hexadecimal literal missing digits".into(),
        });
    }
    Ok(value)
}
