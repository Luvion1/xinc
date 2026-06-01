//! Decimal integer and floating-point literal parsing.
//!
//! Decimal numbers may be plain integers or floats with an optional
//! fractional part (`.digits`) and an optional exponent (`e[+-]?digits`).
//! Underscores are accepted as digit separators.

use super::super::super::Scanner;
use crate::error::LexerError;

/// Parse a decimal integer or floating-point literal.
///
/// Assumes the scanner is positioned at the first digit (no leading
/// `0`-prefix radix dispatch has happened).
pub fn parse_decimal(scanner: &mut Scanner) -> Result<String, LexerError> {
    let mut value = String::new();
    let mut has_dot = false;
    let mut digit_count = 0;

    while let Some(c) = scanner.current_char() {
        if c.is_ascii_digit() || c == '_' {
            value.push(scanner.advance().unwrap_or('\0'));
            digit_count += 1;
        } else if c == '.' && !has_dot {
            has_dot = true;
            value.push(scanner.advance().unwrap_or('\0'));
            while let Some(c2) = scanner.current_char() {
                if c2.is_ascii_digit() || c2 == '_' {
                    value.push(scanner.advance().unwrap_or('\0'));
                } else {
                    break;
                }
            }
            break; // after fraction, maybe exponent next
        } else {
            break;
        }
    }

    if scanner.current_char() == Some('e') || scanner.current_char() == Some('E') {
        append_exponent(scanner, &mut value)?;
    }

    if digit_count == 0 && !has_dot {
        return Err(LexerError::InvalidNumber {
            reason: "numeric literal has no digits".into(),
        });
    }

    Ok(value)
}

/// Consume an optional `e[+-]?digits` exponent suffix.
///
/// Returns `InvalidNumber` if the exponent marker is present but has no
/// digits following it.
fn append_exponent(scanner: &mut Scanner, value: &mut String) -> Result<(), LexerError> {
    value.push(scanner.advance().unwrap_or('\0'));
    if let Some('+') | Some('-') = scanner.current_char() {
        value.push(scanner.advance().unwrap_or('\0'));
    }
    let mut exp_digits = 0;
    while let Some(c) = scanner.current_char() {
        if c.is_ascii_digit() || c == '_' {
            value.push(scanner.advance().unwrap_or('\0'));
            exp_digits += 1;
        } else {
            break;
        }
    }
    if exp_digits == 0 {
        return Err(LexerError::InvalidNumber {
            reason: "exponent has no digits".into(),
        });
    }
    Ok(())
}
