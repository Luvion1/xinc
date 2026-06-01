//! Binary integer literal parsing.
//!
//! Consumes the `0b` prefix and all subsequent `0`/`1` digits (and `_`
//! separators). Requires at least one digit after the prefix.

use super::super::super::Scanner;
use crate::error::LexerError;

/// Parse a `0b` binary integer literal.
pub fn parse_binary(scanner: &mut Scanner) -> Result<String, LexerError> {
    scanner.advance(); // '0'
    scanner.advance(); // 'b'
    let mut value = String::from("0b");
    let mut count = 0;
    while let Some(c) = scanner.current_char() {
        if c == '0' || c == '1' || c == '_' {
            value.push(scanner.advance().unwrap_or('\0'));
            count += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return Err(LexerError::InvalidNumber {
            reason: "binary literal missing digits".into(),
        });
    }
    Ok(value)
}
