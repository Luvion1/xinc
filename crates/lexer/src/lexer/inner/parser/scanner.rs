//! Character-level scanner for lexical analysis.
//!
//! The `Scanner` provides low-level iteration over source code with
//! accurate position tracking (line, column, byte offset). It serves as
//! the foundational input source for the lexer's state machine.
//!
//! # Features
//! - UTF-8 character decoding
//! - CRLF newline handling (treats `\r\n` as single newline)
//! - Peek capability without consumption
//! - Position queries (line, column, byte offset)
//!
//! # Example
//! ```ignore
//! use xin_lexer::lexer::inner::parser::Scanner;
//! let mut scanner = Scanner::new("fn main() {}\n");
//! assert_eq!(scanner.current_char(), Some('f'));
//! scanner.advance();
//! assert_eq!(scanner.position().line, 1);
//! ```
//!
//! The scanner is deliberately simple; all lexical knowledge (operators,
//! keywords, literals) is handled by higher-level parsers.

use crate::diagnostics::Position;
use crate::error::LexerError;

/// Scanner reads source code character by character, tracking position.
///
/// Maintains byte offset, line number (1-indexed), and column number (1-indexed).
/// Handles multi-byte UTF-8 characters and normalizes CRLF line endings.
#[derive(Debug, Clone)]
pub struct Scanner {
    /// The complete source text
    source: String,
    /// Current byte offset into source
    byte_offset: usize,
    /// Current line number (starting at 1)
    line: u32,
    /// Current column number (starting at 1)
    column: u32,
}

impl Scanner {
    /// Create a new scanner from source string.
    ///
    /// # Arguments
    /// * `source` - The source code to scan
    ///
    /// # Returns
    /// A new Scanner positioned at the start of the source
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), byte_offset: 0, line: 1, column: 1 }
    }

    /// Check if scanner has reached end of source.
    ///
    /// # Returns
    /// `true` if no more characters remain
    pub fn is_at_end(&self) -> bool {
        self.byte_offset >= self.source.len()
    }

    /// Get current line number (1-indexed).
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Get current column number (1-indexed).
    pub fn column(&self) -> u32 {
        self.column
    }

    /// Get current position (line and column).
    pub fn position(&self) -> Position {
        Position::new(self.line, self.column)
    }

    /// Get current byte offset into source.
    ///
    /// Useful for error reporting where byte offset is required.
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// Get the current character without consuming it.
    ///
    /// # Returns
    /// `Some(char)` if not at end, `None` if at end
    pub fn current_char(&self) -> Option<char> {
        if self.is_at_end() { None } else { self.source[self.byte_offset..].chars().next() }
    }

    /// Peek at the next character without consuming it.
    ///
    /// This looks ahead one character from the current position.
    ///
    /// # Returns
    /// `Some(char)` if there is a next character, `None` if at end or
    /// if the next character cannot be determined
    pub fn peek(&self) -> Option<char> {
        let c = self.current_char()?;
        let after = self.byte_offset + c.len_utf8();
        if after >= self.source.len() { None } else { self.source[after..].chars().next() }
    }

    /// Consume and return the current character, advancing the scanner.
    ///
    /// Updates line/column counters. Treats `\n` and `\r\n` as single newline.
    ///
    /// # Returns
    /// The consumed character, or `None` if at end
    pub fn advance(&mut self) -> Option<char> {
        let c = self.current_char()?;
        let c_len = c.len_utf8();
        self.byte_offset += c_len;

        match c {
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            '\r' => {
                // If followed by LF, consume it as part of same newline
                if self.current_char() == Some('\n') {
                    // The current char after advancing is the LF, consume it
                    self.byte_offset += 1; // consume LF
                }
                self.line += 1;
                self.column = 1;
            }
            _ => {
                self.column += 1;
            }
        }

        Some(c)
    }

    /// Skip the current character if it matches `expected`.
    ///
    /// If the current character equals `expected`, it is consumed and `Ok(())` is returned.
    /// Otherwise, returns `Err(LexerError::InvalidChar)` with byte position.
    ///
    /// # Arguments
    /// * `expected` - Character to skip
    ///
    /// # Returns
    /// `Ok(())` if matched and consumed, `Err` otherwise
    pub fn expect(&mut self, expected: char) -> Result<(), crate::error::LexerError> {
        if let Some(c) = self.current_char() && c == expected {
            self.advance();
            return Ok(());
        }
        Err(crate::error::LexerError::InvalidChar {
            character: expected,
            position: self.byte_offset,
        })
    }

    /// Skip whitespace and comments.
    ///
    /// Consumes spaces, tabs, newlines, line comments (`//`),
    /// and block comments (`/* ... */`). Returns `Ok(())` on success,
    /// or `Err(LexerError::UnterminatedComment)` if a block comment is unterminated.
    pub fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        loop {
            // Skip whitespace characters
            while let Some(c) = self.current_char() {
                if c.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.is_at_end() {
                return Ok(());
            }
            // Line comment: "//"
            if self.current_char() == Some('/') && self.peek() == Some('/') {
                self.advance(); // '/'
                self.advance(); // '/'
                while let Some(c) = self.current_char() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            // Block comment: "/* ... */"
            if self.current_char() == Some('/') && self.peek() == Some('*') {
                // consume "/*"
                self.advance(); // '/'
                self.advance(); // '*'
                loop {
                    match self.current_char() {
                        Some('*') => {
                            if self.peek() == Some('/') {
                                self.advance(); // '*'
                                self.advance(); // '/'
                                break;
                            } else {
                                self.advance();
                            }
                        }
                        Some(_) => {
                            self.advance();
                        }
                        None => return Err(LexerError::UnterminatedComment),
                    }
                }
                continue;
            }
            break;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scanner() {
        let s = Scanner::new("abc");
        assert!(!s.is_at_end());
        assert_eq!(s.current_char(), Some('a'));
        assert_eq!(s.position().line, 1);
        assert_eq!(s.position().column, 1);
    }

    #[test]
    fn test_advance() {
        let mut s = Scanner::new("ab");
        s.advance();
        assert_eq!(s.current_char(), Some('b'));
        assert_eq!(s.column, 2);
    }

    #[test]
    fn test_newline_lf() {
        let mut s = Scanner::new("a\nb");
        s.advance(); // 'a'
        s.advance(); // '\n'
        assert_eq!(s.line(), 2);
        assert_eq!(s.column(), 1);
        assert_eq!(s.current_char(), Some('b'));
    }

    #[test]
    fn test_newline_crlf() {
        let mut s = Scanner::new("a\r\nb");
        s.advance(); // 'a'
        s.advance(); // '\r' (consumes also '\n')
        assert_eq!(s.line(), 2);
        assert_eq!(s.column(), 1);
        assert_eq!(s.current_char(), Some('b'));
    }

    #[test]
    fn test_peek() {
        let mut s = Scanner::new("ab");
        assert_eq!(s.peek(), Some('b'));
        s.advance();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_expect_success() {
        let mut s = Scanner::new("a");
        assert!(s.expect('a').is_ok());
        assert!(s.is_at_end());
    }

    #[test]
    fn test_expect_failure() {
        let mut s = Scanner::new("b");
        let err = s.expect('a').unwrap_err();
        match err {
            crate::error::LexerError::InvalidChar { character, position } => {
                assert_eq!(character, 'a');
                assert_eq!(position, 0);
            }
            _ => panic!("Wrong error variant"),
        }
    }
}
