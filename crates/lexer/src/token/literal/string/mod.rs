//! String literal tokens.
//!
//! Handles string literals in Xin source code.
//! Supports Unicode, escape sequences, and validation.

/// Escape sequence handling.
///
/// Processes backslash escape sequences.
pub mod escape;

use escape::EscapeSequence;

/// A string literal value.
///
/// Contains the raw string content and metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    /// The string content.
    pub value: String,
    /// Whether this is a raw string.
    pub is_raw: bool,
}

impl StringLiteral {
    /// Create a new string literal.
    pub fn new(value: String) -> Self {
        StringLiteral { value, is_raw: false }
    }

    /// Create a raw string literal.
    pub fn new_raw(value: String) -> Self {
        StringLiteral { value, is_raw: true }
    }

    /// Get the length of the string.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Check if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

/// Process escape sequences in a string.
///
/// Converts escape sequences to their actual characters.
///
/// # Arguments
/// * `input` - The string with escape sequences.
///
/// # Returns
/// The string with escape sequences resolved.
pub fn process_escapes(input: &str) -> String {
    // Build result character by character
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                match next {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => result.push(next),
                }
                chars.next();
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = StringLiteral::new("hello".to_string());
        assert_eq!(s.value, "hello");
        assert!(!s.is_raw);
    }

    #[test]
    fn test_raw() {
        let s = StringLiteral::new_raw("hello".to_string());
        assert_eq!(s.value, "hello");
        assert!(s.is_raw);
    }

    #[test]
    fn test_len() {
        let s = StringLiteral::new("hello".to_string());
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_empty() {
        let s = StringLiteral::new("".to_string());
        assert!(s.is_empty());
    }

    #[test]
    fn test_escape_newline() {
        let result = process_escapes("hello\\nworld");
        assert_eq!(result, "hello\nworld");
    }

    #[test]
    fn test_escape_tab() {
        let result = process_escapes("hello\\tworld");
        assert_eq!(result, "hello\tworld");
    }

    #[test]
    fn test_escape_backslash() {
        let result = process_escapes("hello\\\\world");
        assert_eq!(result, "hello\\world");
    }
}
