//! Escape sequence validation.
//!
//! Validates escape sequences in string literals according to Xin language rules.
//! The validator ensures that:
//! - Standard escapes (\n, \t, \\, \", \r) are always valid
//! - Unicode escapes (\u{...}) are syntactically well-formed
//! - Unknown escapes are rejected (e.g., \x is invalid)
//!
//! Unicode validation additionally checks code point range (<= 0x10FFFF) and
//! excludes surrogate halves (0xD800-0xDFFF) which are invalid in Rust strings.
//!
//! This module is used during lexing when processing string literals. Invalid
//! escapes produce a `LexerError`.

/// Unicode escape validation.
pub mod unicode;

use super::super::EscapeSequence;

/// Validate an escape sequence.
pub fn validate_escape(escape: &EscapeSequence) -> bool {
    match escape {
        super::EscapeSequence::Newline => true,
        super::EscapeSequence::Tab => true,
        super::EscapeSequence::CarriageReturn => true,
        super::EscapeSequence::Backslash => true,
        super::EscapeSequence::Quote => true,
        super::EscapeSequence::Unicode(_) => true,
        super::EscapeSequence::Unknown(_) => false,
    }
}

/// Validate a Unicode character.
pub fn validate_unicode_char(c: char) -> bool {
    let code = c as u32;
    (0x0000..=0x10FFFF).contains(&code) && !(0xD800..=0xDFFF).contains(&code)
}

#[cfg(test)]
mod tests {
    use super::super::EscapeSequence;
    use super::*;

    #[test]
    fn test_valid_newline() {
        assert!(validate_escape(&EscapeSequence::Newline));
    }
    #[test]
    fn test_valid_tab() {
        assert!(validate_escape(&EscapeSequence::Tab));
    }
    #[test]
    fn test_valid_backslash() {
        assert!(validate_escape(&EscapeSequence::Backslash));
    }
    #[test]
    fn test_valid_quote() {
        assert!(validate_escape(&EscapeSequence::Quote));
    }
    #[test]
    fn test_valid_unicode() {
        assert!(validate_escape(&EscapeSequence::Unicode('A')));
    }
    #[test]
    fn test_invalid_unknown() {
        assert!(!validate_escape(&EscapeSequence::Unknown('x')));
    }
    #[test]
    fn test_valid_unicode_char() {
        assert!(validate_unicode_char('A'));
    }
    #[test]
    fn test_valid_high_unicode() {
        // Emoji (U+1F600) is a valid Unicode scalar value
        assert!(validate_unicode_char('\u{1F600}'));
    }
}
