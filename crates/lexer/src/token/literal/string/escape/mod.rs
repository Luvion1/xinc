//! Escape sequence handling in string literals.
//! Processes backslash escapes to their actual characters.

/// Escape sequence validation.
pub mod validator;

use validator::validate_escape;

/// An escape sequence within a string literal.
///
/// Represents `\` followed by a character that has
/// special meaning in strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeSequence {
    /// Newline: \n
    Newline,
    /// Tab: \t
    Tab,
    /// Carriage return: \r
    CarriageReturn,
    /// Backslash: \\
    Backslash,
    /// Double quote: \"
    Quote,
    /// Unicode escape: \u{XXXX}
    Unicode(char),
    /// Unknown/unsupported escape
    Unknown(char),
}

/// Parse an escape sequence from a character.
///
/// Takes the character after backslash and returns
/// the corresponding escape variant.
///
/// # Arguments
/// * `c` - Character following the backslash
///
/// # Returns
/// Matching `EscapeSequence` enum variant
pub fn parse_escape(c: char) -> EscapeSequence {
    match c {
        'n' => EscapeSequence::Newline,
        't' => EscapeSequence::Tab,
        'r' => EscapeSequence::CarriageReturn,
        '\\' => EscapeSequence::Backslash,
        '"' => EscapeSequence::Quote,
        'u' => EscapeSequence::Unicode('\0'),
        _ => EscapeSequence::Unknown(c),
    }
}

/// Convert escape sequence to its actual character.
///
/// Maps escape to the character it represents.
///
/// # Arguments
/// * `escape` - The escape sequence
///
/// # Returns
/// The character value
pub fn escape_to_char(escape: &EscapeSequence) -> char {
    match escape {
        EscapeSequence::Newline => '\n',
        EscapeSequence::Tab => '\t',
        EscapeSequence::CarriageReturn => '\r',
        EscapeSequence::Backslash => '\\',
        EscapeSequence::Quote => '"',
        EscapeSequence::Unicode(c) | EscapeSequence::Unknown(c) => *c,
    }
}

/// Check if escape sequence is valid.
///
/// Validates escape per Xin language rules.
///
/// # Arguments
/// * `escape` - Escape sequence to validate
///
/// # Returns
/// True if valid, false otherwise
pub fn is_valid_escape(escape: &EscapeSequence) -> bool {
    validate_escape(escape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_newline() {
        assert!(matches!(parse_escape('n'), EscapeSequence::Newline));
    }

    #[test]
    fn test_parse_tab() {
        assert!(matches!(parse_escape('t'), EscapeSequence::Tab));
    }

    #[test]
    fn test_parse_backslash() {
        assert!(matches!(parse_escape('\\'), EscapeSequence::Backslash));
    }

    #[test]
    fn test_parse_unknown() {
        match parse_escape('x') {
            EscapeSequence::Unknown(c) => assert_eq!(c, 'x'),
            _ => panic!("Expected Unknown"),
        }
    }

    #[test]
    fn test_escape_to_char_newline() {
        let c = escape_to_char(&EscapeSequence::Newline);
        assert_eq!(c, '\n');
    }

    #[test]
    fn test_escape_to_char_tab() {
        let c = escape_to_char(&EscapeSequence::Tab);
        assert_eq!(c, '\t');
    }
}
