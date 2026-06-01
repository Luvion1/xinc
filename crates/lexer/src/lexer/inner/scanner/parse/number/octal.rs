//! Octal integer literal parsing.
//!
//! Consumes the `0o` prefix and all subsequent digits in the range
//! `0..=7` (and `_` separators). Requires at least one digit after the
//! prefix.

use super::super::super::Scanner;
use crate::error::LexerError;

/// Parse a `0o` octal integer literal.
pub fn parse_octal(scanner: &mut Scanner) -> Result<String, LexerError> {
    scanner.advance(); // '0'
    scanner.advance(); // 'o'
    let mut value = String::from("0o");
    let mut count = 0;
    while let Some(c) = scanner.current_char() {
        if ('0'..='7').contains(&c) || c == '_' {
            value.push(scanner.advance().unwrap_or('\0'));
            count += 1;
        } else {
            break;
        }
    }
    if count == 0 {
        return Err(LexerError::InvalidNumber {
            reason: "octal literal missing digits".into(),
        });
    }
    Ok(value)
}
