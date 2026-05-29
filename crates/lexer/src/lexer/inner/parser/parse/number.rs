//! Numeric literal parsing.
//!
//! Parses integer and floating-point literals with support for multiple radices
//! and underscores as digit separators. Returns the raw lexical form (including
//! underscores and prefixes) as a `String`.

use super::super::scanner::Scanner;
use crate::error::LexerError;

/// Parse a numeric literal.
///
/// Recognizes decimal, hexadecimal (`0x`), binary (`0b`), octal (`0o`) integers,
/// and floating-point numbers with optional fractional part and exponent.
pub fn parse_number(scanner: &mut Scanner) -> Result<String, LexerError> {
    let mut value = String::new();
    let mut has_dot = false;
    let mut has_exp = false;

    // Detect radix prefix for non-decimal bases
    match scanner.current_char() {
        Some('0') => {
            if let Some('x') = scanner.peek() {
                scanner.advance(); // '0'
                scanner.advance(); // 'x'
                value.push_str("0x");
                let mut count = 0;
                while let Some(c) = scanner.current_char() {
                    if c.is_ascii_hexdigit() || c == '_' {
                        value.push(scanner.advance().unwrap());
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
                return Ok(value);
            }
            if let Some('b') = scanner.peek() {
                scanner.advance();
                scanner.advance();
                value.push_str("0b");
                let mut count = 0;
                while let Some(c) = scanner.current_char() {
                    if c == '0' || c == '1' || c == '_' {
                        value.push(scanner.advance().unwrap());
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
                return Ok(value);
            }
            if let Some('o') = scanner.peek() {
                scanner.advance();
                scanner.advance();
                value.push_str("0o");
                let mut count = 0;
                while let Some(c) = scanner.current_char() {
                    if ('0'..='7').contains(&c) || c == '_' {
                        value.push(scanner.advance().unwrap());
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
                return Ok(value);
            }
            // Just '0' alone
            scanner.advance();
            value.push('0');
            Ok(value)
        }
        _ => {
            // Decimal (integer or float)
            let mut digit_count = 0;
            while let Some(c) = scanner.current_char() {
                if c.is_ascii_digit() || c == '_' {
                    value.push(scanner.advance().unwrap());
                    digit_count += 1;
                } else if c == '.' && !has_dot && !has_exp {
                    has_dot = true;
                    value.push(scanner.advance().unwrap());
                    // Fractional digits
                    while let Some(c2) = scanner.current_char() {
                        if c2.is_ascii_digit() || c2 == '_' {
                            value.push(scanner.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    break; // after fraction, maybe exponent next
                } else {
                    break;
                }
            }

            // Exponent for floats (may appear with or without decimal point)
            if scanner.current_char() == Some('e') || scanner.current_char() == Some('E') {
                has_exp = true;
                value.push(scanner.advance().unwrap());
                // optional sign
                if let Some('+') | Some('-') = scanner.current_char() {
                    value.push(scanner.advance().unwrap());
                }
                // Must have at least one digit
                let mut exp_digits = 0;
                while let Some(c) = scanner.current_char() {
                    if c.is_ascii_digit() || c == '_' {
                        value.push(scanner.advance().unwrap());
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
            }

            // Ensure at least one digit before any dot/exponent
            if digit_count == 0 && !has_dot && !has_exp {
                return Err(LexerError::InvalidNumber {
                    reason: "numeric literal has no digits".into(),
                });
            }

            Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_integer() {
        let mut s = Scanner::new("42");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "42");
    }

    #[test]
    fn test_decimal_float() {
        let mut s = Scanner::new("3.14");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "3.14");
    }

    #[test]
    fn test_hex() {
        let mut s = Scanner::new("0xFF");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "0xFF");
    }

    #[test]
    fn test_number_negative() {
        // Negative numbers are tokenized as two tokens: '-' operator and positive number
        // This test verifies the scanner starts correctly
        let s = Scanner::new("-42");
        assert_eq!(s.current_char(), Some('-'));
    }

    #[test]
    fn test_octal() {
        let mut s = Scanner::new("0o77");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "0o77");
    }

    #[test]
    fn test_exponent() {
        let mut s = Scanner::new("1e10");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "1e10");
    }

    #[test]
    fn test_underscores() {
        let mut s = Scanner::new("1_000_000");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "1_000_000");
    }

    #[test]
    fn test_invalid_hex_no_digits() {
        let mut s = Scanner::new("0x");
        let err = parse_number(&mut s).unwrap_err();
        assert!(matches!(err, LexerError::InvalidNumber { .. }));
    }

    #[test]
    fn test_invalid_binary_no_digits() {
        let mut s = Scanner::new("0b");
        let err = parse_number(&mut s).unwrap_err();
        assert!(matches!(err, LexerError::InvalidNumber { .. }));
    }

    #[test]
    fn test_invalid_octal_no_digits() {
        let mut s = Scanner::new("0o");
        let err = parse_number(&mut s).unwrap_err();
        assert!(matches!(err, LexerError::InvalidNumber { .. }));
    }

    #[test]
    fn test_binary_number() {
        let mut s = Scanner::new("0b1010");
        let val = parse_number(&mut s).unwrap();
        assert_eq!(val, "0b1010");
    }

    #[test]
    fn test_invalid_empty() {
        let mut s = Scanner::new("");
        let err = parse_number(&mut s).unwrap_err();
        assert!(matches!(err, LexerError::InvalidNumber { .. }));
    }
}
