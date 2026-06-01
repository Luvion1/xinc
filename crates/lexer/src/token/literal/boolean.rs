//! Boolean literal tokens.
//! true and false values.

/// Boolean literal value.
///
/// Represents one of two constant values: true or false.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolLiteral {
    /// The true literal
    True,
    /// The false literal
    False,
}

impl BoolLiteral {
    /// Get the boolean value.
    ///
    /// # Returns
    /// Rust bool corresponding to this literal
    pub fn value(self) -> bool {
        matches!(self, Self::True)
    }

    /// Parse a boolean from a string.
    ///
    /// # Arguments
    /// * `input` - String "true" or "false"
    ///
    /// # Returns
    /// Some(BoolLiteral) if valid, None otherwise
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_value() {
        assert!(BoolLiteral::True.value());
    }

    #[test]
    fn test_false_value() {
        assert!(!BoolLiteral::False.value());
    }

    #[test]
    fn test_parse_true() {
        assert_eq!(BoolLiteral::parse("true"), Some(BoolLiteral::True));
    }

    #[test]
    fn test_parse_false() {
        assert_eq!(BoolLiteral::parse("false"), Some(BoolLiteral::False));
    }

    #[test]
    fn test_parse_invalid() {
        assert_eq!(BoolLiteral::parse("yes"), None);
    }
}
