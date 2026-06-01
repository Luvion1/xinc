//! Literal token values.
//! String, number, boolean, char, and null literals.

/// String literal tokens and escape sequences.
///
/// Handles Unicode, escape sequences, and validation.
pub mod string;

/// Numeric literal tokens.
///
/// Integers and floats with radix support.
pub mod number;

/// Character literal tokens.
///
/// Single Unicode characters.
pub mod character;

/// Boolean literal tokens.
///
/// true and false values.
pub mod boolean;

/// Null literal token.
///
/// The null value.
pub mod null_;

// Re-exports
pub use boolean::BoolLiteral;
pub use character::CharLiteral;
pub use null_::NullLiteral;
pub use number::NumberLiteral;
pub use string::StringLiteral;

/// All literal value types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// A string literal
    String(StringLiteral),
    /// A numeric literal (stored as String for simplicity)
    Number(String),
    /// A character literal
    Character(char),
    /// A boolean literal
    Boolean(bool),
    /// A null literal
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let lit = Literal::Number("42".to_string());
        assert!(matches!(lit, Literal::Number(_)));
    }

    #[test]
    fn test_boolean_true() {
        let lit = Literal::Boolean(true);
        assert!(matches!(lit, Literal::Boolean(true)));
    }

    #[test]
    fn test_boolean_false() {
        let lit = Literal::Boolean(false);
        assert!(matches!(lit, Literal::Boolean(false)));
    }

    #[test]
    fn test_null() {
        let lit = Literal::Null;
        assert!(matches!(lit, Literal::Null));
    }
}
