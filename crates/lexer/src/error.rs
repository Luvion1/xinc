//! Lexer error types.
//! Errors that occur during tokenization.

use thiserror::Error;
// For Error derive

/// Lexer error enumeration.
///
/// All possible errors that the lexer can produce
/// while tokenizing source code.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LexerError {
    /// Invalid character not allowed in Xin.
    ///
    /// Contains the character and its byte position.
    #[error("Invalid character '{character}' at position {position}")]
    InvalidChar {
        /// The unexpected character
        character: char,
        /// Byte offset in source string
        position: usize,
    },

    /// Unterminated string literal.
    ///
    /// A string literal was started but not closed
    /// before end of source.
    #[error("Unterminated string literal")]
    UnterminatedString,

    /// Unterminated character literal.
    ///
    /// A character literal (in single quotes) was not closed.
    #[error("Unterminated character literal")]
    UnterminatedChar,

    /// Unterminated block comment.
    ///
    /// A comment starting with /* never got its */ terminator.
    #[error("Unterminated comment (missing '*/')")]
    UnterminatedComment,

    /// Invalid escape sequence in string/char.
    ///
    /// Contains the escape character and position.
    #[error("Invalid escape sequence '\\{char}' at position {position}")]
    InvalidEscape {
        /// The escape character after backslash
        char: char,
        /// Byte position of backslash
        position: usize,
    },

    /// Invalid number literal format.
    ///
    /// Examples: leading zeros, overflow, etc.
    #[error("Invalid number literal: {reason}")]
    InvalidNumber {
        /// Explanation of what's wrong
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_char_error() {
        let err = LexerError::InvalidChar { character: '@', position: 5 };
        assert!(err.to_string().contains("@"));
    }

    #[test]
    fn test_unterminated_string() {
        let err = LexerError::UnterminatedString;
        assert!(err.to_string().contains("Unterminated"));
    }

    #[test]
    fn test_invalid_escape() {
        let err = LexerError::InvalidEscape { char: 'x', position: 10 };
        assert!(err.to_string().contains("\\x"));
    }

    #[test]
    fn test_invalid_number() {
        let err = LexerError::InvalidNumber { reason: "overflow".to_string() };
        assert!(err.to_string().contains("overflow"));
    }

    #[test]
    fn test_unterminated_char() {
        let err = LexerError::UnterminatedChar;
        assert!(err.to_string().contains("Unterminated"));
    }

    #[test]
    fn test_unterminated_comment() {
        let err = LexerError::UnterminatedComment;
        assert!(err.to_string().contains("Unterminated"));
    }
}
