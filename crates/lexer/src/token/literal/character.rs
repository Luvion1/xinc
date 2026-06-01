//! Character literal token handling.
//! Single Unicode characters in single quotes.

/// A character literal value.
///
/// Contains the character and whether it's raw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharLiteral {
    /// The Unicode character value
    pub value: char,
    /// Whether it's a raw (unescaped) literal
    pub is_raw: bool,
}

impl CharLiteral {
    /// Create a new character literal.
    ///
    /// # Arguments
    /// * `value` - The character value
    ///
    /// # Returns
    /// A new `CharLiteral` instance (not raw)
    pub fn new(value: char) -> Self {
        Self { value, is_raw: false }
    }

    /// Create a raw character literal.
    ///
    /// Raw literals don't process escape sequences.
    pub fn new_raw(value: char) -> Self {
        Self { value, is_raw: true }
    }

    /// Get the character value.
    ///
    /// # Returns
    /// The stored character
    pub fn value(&self) -> char {
        self.value
    }

    /// Check if this is a raw literal.
    ///
    /// # Returns
    /// True if raw, false if escaped
    pub fn is_raw(&self) -> bool {
        self.is_raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = CharLiteral::new('a');
        assert_eq!(c.value, 'a');
        assert!(!c.is_raw);
    }

    #[test]
    fn test_raw() {
        let c = CharLiteral::new_raw('b');
        assert_eq!(c.value, 'b');
        assert!(c.is_raw);
    }
}
