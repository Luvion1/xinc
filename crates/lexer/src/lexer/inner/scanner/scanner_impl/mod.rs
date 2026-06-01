//! Character-level scanner for lexical analysis.

#[cfg(test)]
mod tests;

use crate::diagnostics::Position;
use crate::error::LexerError;

/// Scanner reads source code character by character, tracking position.
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
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), byte_offset: 0, line: 1, column: 1 }
    }

    /// Check if scanner has reached end of source.
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
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// Get the current character without consuming it.
    pub fn current_char(&self) -> Option<char> {
        if self.is_at_end() { None } else { self.source[self.byte_offset..].chars().next() }
    }

    /// Peek at the next character without consuming it.
    pub fn peek(&self) -> Option<char> {
        let c = self.current_char()?;
        let after = self.byte_offset + c.len_utf8();
        if after >= self.source.len() { None } else { self.source[after..].chars().next() }
    }

    /// Consume and return the current character, advancing the scanner.
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
                if self.current_char() == Some('\n') {
                    self.byte_offset += 1;
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
    pub fn expect(&mut self, expected: char) -> Result<(), LexerError> {
        if let Some(c) = self.current_char()
            && c == expected
        {
            self.advance();
            return Ok(());
        }
        Err(LexerError::InvalidChar { character: expected, position: self.byte_offset })
    }

    /// Skip whitespace and comments.
    pub fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        loop {
            while let Some(c) = self.current_char()
                && c.is_whitespace()
            {
                self.advance();
            }
            if self.is_at_end() {
                return Ok(());
            }
            if self.current_char() == Some('/') && self.peek() == Some('/') {
                self.advance();
                self.advance();
                while let Some(c) = self.current_char()
                    && c != '\n'
                {
                    self.advance();
                }
                continue;
            }
            if self.current_char() == Some('/') && self.peek() == Some('*') {
                self.advance();
                self.advance();
                loop {
                    match self.current_char() {
                        Some('*') if self.peek() == Some('/') => {
                            self.advance();
                            self.advance();
                            break;
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
