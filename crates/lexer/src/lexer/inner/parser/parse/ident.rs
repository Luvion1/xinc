//! Identifier parsing for lexical analysis.
//!
//! Parses identifiers (user-defined names) and distinguishes them from keywords.
//! An identifier starts with a Unicode letter or underscore, followed by letters,
//! digits, or underscores. All keywords are reserved and cannot be used as identifiers.

use super::super::scanner::Scanner;
use crate::error::LexerError;
use crate::token::identifier::is_identifier_start;

/// Parse an identifier from the current scanner position.
///
/// The identifier must start with a letter or underscore, followed by
/// zero or more letters, digits, or underscores. Consumes all valid identifier
/// characters and returns the identifier's name as a `String`.
///
/// # Arguments
/// * `scanner` - The scanner providing character input
///
/// # Returns
/// `Ok(String)` containing the identifier name, or `Err(LexerError::InvalidChar)`
/// if the first character is not a valid identifier start.
pub fn parse_identifier(scanner: &mut Scanner) -> Result<String, LexerError> {
    let start = scanner.byte_offset();

    // Validate first character before consuming
    let first = scanner
        .current_char()
        .ok_or(LexerError::InvalidChar { character: '\0', position: start })?;
    if !is_identifier_start(first) {
        return Err(LexerError::InvalidChar { character: first, position: start });
    }

    let mut result = String::new();

    // Consume the first character (which we already validated)
    result.push(scanner.advance().unwrap());

    // Consume subsequent identifier characters
    while let Some(c) = scanner.current_char() {
        if is_identifier_continue(c) {
            result.push(scanner.advance().unwrap());
        } else {
            break;
        }
    }

    Ok(result)
}

/// Check if a character is a valid continuation for identifiers.
///
/// After the first character, identifiers may also contain digits.
///
/// # Arguments
/// * `c` - Character to test
pub fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier_simple() {
        let mut s = Scanner::new("foo123");
        let id = parse_identifier(&mut s).unwrap();
        assert_eq!(id, "foo123");
        assert!(s.is_at_end());
    }

    #[test]
    fn test_parse_identifier_underscore() {
        let mut s = Scanner::new("_bar_baz");
        let id = parse_identifier(&mut s).unwrap();
        assert_eq!(id, "_bar_baz");
    }

    #[test]
    fn test_parse_identifier_stops_at_space() {
        let mut s = Scanner::new("foo bar");
        let id = parse_identifier(&mut s).unwrap();
        assert_eq!(id, "foo");
        assert_eq!(s.current_char(), Some(' '));
    }

    #[test]
    fn test_parse_identifier_unicode() {
        let mut s = Scanner::new("naïve");
        let id = parse_identifier(&mut s).unwrap();
        assert_eq!(id, "naïve");
    }

    #[test]
    fn test_parse_identifier_mixed_unicode() {
        let mut s = Scanner::new("tokö_λ");
        let id = parse_identifier(&mut s).unwrap();
        assert_eq!(id, "tokö_λ");
    }

    #[test]
    fn test_identifier_continue_check() {
        assert!(is_identifier_continue('a'));
        assert!(is_identifier_continue('1'));
        assert!(!is_identifier_continue('!'));
    }
}
