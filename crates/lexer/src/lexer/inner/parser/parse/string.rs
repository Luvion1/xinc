//! String literal parsing.
//!
//! Parses string literals with escape sequence handling.

use crate::lexer::inner::parser::scanner::Scanner;

/// Parse a normal string literal.
pub fn parse_string_normal(scanner: &mut Scanner) -> Result<String, crate::error::LexerError> {
    let mut result = String::new();
    loop {
        match scanner.current_char() {
            Some('"') => {
                scanner.advance();
                return Ok(result);
            }
            // Handle escape sequences
            Some('\\') => {
                scanner.advance();
                let ch = scanner.current_char().unwrap_or('\0');
                match ch {
                    'n' => {
                        result.push('\n');
                        scanner.advance();
                    }
                    't' => {
                        result.push('\t');
                        scanner.advance();
                    }
                    'r' => {
                        result.push('\r');
                        scanner.advance();
                    }
                    '\\' => {
                        result.push('\\');
                        scanner.advance();
                    }
                    '"' => {
                        result.push('"');
                        scanner.advance();
                    }
                    _ => return Err(crate::error::LexerError::InvalidEscape { char: ch, position: scanner.byte_offset() }),
                }
            }
            Some(c) => {
                result.push(c);
                scanner.advance();
            }
            None => return Err(crate::error::LexerError::UnterminatedString),
        }
    }
}

/// Parse a raw string literal (r"...").
// Raw strings don't process escape sequences
pub fn parse_raw_string(scanner: &mut Scanner) -> Result<String, crate::error::LexerError> {
    let mut result = String::new();
    loop {
        match scanner.current_char() {
            Some('"') => {
                scanner.advance();
                return Ok(result);
            }
            Some(c) => {
                result.push(c);
                scanner.advance();
            }
            None => return Err(crate::error::LexerError::UnterminatedString),
        }
    }
}

/// Parse a character literal.
pub fn parse_char_normal(scanner: &mut Scanner) -> Result<char, crate::error::LexerError> {
    // Get the character
    match scanner.current_char() {
        Some(c) => {
            scanner.advance();
            Ok(c)
        }
        None => Err(crate::error::LexerError::UnterminatedChar),
    }
}

/// Parse a raw character literal (r'...').
pub fn parse_char_raw(scanner: &mut Scanner) -> Result<char, crate::error::LexerError> {
    match scanner.current_char() {
        Some(c) => {
            scanner.advance();
            // Expect closing quote
            if scanner.current_char() == Some('\'') {
                scanner.advance();
                Ok(c)
            } else {
                Err(crate::error::LexerError::UnterminatedChar)
            }
        }
        None => Err(crate::error::LexerError::UnterminatedChar),
    }
}