//! Diagnostic types for the lexer.
//!
//! This module provides types for reporting errors and positions in the source code.

/// Represents a position in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Line number (1-indexed).
    pub line: u32,
    /// Column number (1-indexed).
    pub column: u32,
}

impl Position {
    /// Creates a new position at the given line and column.
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(10, 5);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.column, 5);
    }

    #[test]
    fn test_position_eq() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(1, 1);
        assert_eq!(pos1, pos2);
    }
}
