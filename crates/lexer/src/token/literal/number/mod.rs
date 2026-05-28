//! Numeric literal tokens.
//! Handles integer and floating-point literals.

/// Radix support.
///
/// Binary, octal, decimal, hexadecimal number parsing.
pub mod radix;

/// A numeric literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum NumberLiteral {
    /// An integer
    Integer(IntegerLiteral),
    /// A float
    Float(FloatLiteral),
}

/// Integer literal with radix.
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: String,
    pub radix: u8,
    pub has_underscore: bool,
}

/// Float literal.
#[derive(Debug, Clone, PartialEq)]
pub struct FloatLiteral {
    pub value: String,
    pub has_decimal: bool,
    pub has_exponent: bool,
}

/// Parse number from string.
pub fn parse_number(input: &str) -> Option<NumberLiteral> {
    if input.contains('.') {
        Some(NumberLiteral::Float(FloatLiteral {
            value: input.to_string(),
            has_decimal: true,
            has_exponent: input.contains('e') || input.contains('E'),
        }))
    } else {
        Some(NumberLiteral::Integer(IntegerLiteral {
            value: input.to_string(),
            radix: 10,
            has_underscore: input.contains('_'),
        }))
    }
}

/// Detect radix from prefix.
pub fn detect_radix(input: &str) -> Option<u8> {
    if input.starts_with("0x") || input.starts_with("0X") {
        Some(16)
    } else if input.starts_with("0b") || input.starts_with("0B") {
        Some(2)
    } else if input.starts_with("0o") || input.starts_with("0O") {
        Some(8)
    } else {
        Some(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let result = parse_number("42");
        assert!(matches!(result, Some(NumberLiteral::Integer(_))));
    }

    #[test]
    fn test_parse_float() {
        let result = parse_number("3.14");
        assert!(matches!(result, Some(NumberLiteral::Float(_))));
    }

    #[test]
    fn test_radix_hex() {
        assert_eq!(detect_radix("0xFF"), Some(16));
    }
    #[test]
    fn test_radix_binary() {
        assert_eq!(detect_radix("0b101"), Some(2));
    }
    #[test]
    fn test_radix_octal() {
        assert_eq!(detect_radix("0o77"), Some(8));
    }
    #[test]
    fn test_radix_decimal() {
        assert_eq!(detect_radix("42"), Some(10));
    }
}
