//! Numeric literal tokens.
//!
//! Defines the type-level representation of a Xin number literal. The
//! parser side ([`crate::lexer::inner::scanner::parse::number`]) takes
//! raw characters and produces a [`NumberLiteral`].
//!
//! # Forms supported
//!
//! | Form | Example | Stored as |
//! |------|---------|-----------|
//! | Decimal integer | `42`, `1_000` | [`IntegerLiteral`] with `radix = 10` |
//! | Hex integer | `0x2A`, `0xFF_u32` | [`IntegerLiteral`] with `radix = 16` |
//! | Octal integer | `0o52` | [`IntegerLiteral`] with `radix = 8` |
//! | Binary integer | `0b1010` | [`IntegerLiteral`] with `radix = 2` |
//! | Float | `3.14`, `1.0e10` | [`FloatLiteral`] with `has_decimal = true` |
//!
//! Underscores (`_`) are allowed between digits as visual separators and
//! recorded via the `has_underscore` flag for diagnostics.
//!
//! # Why stringly typed
//!
//! The value is kept as a `String` rather than `u64`/`f64` so the lexer
//! does not silently lose precision on large literals or perform
//! premature type inference. Semantic analysis is the right place to
//! pick a concrete numeric type and check the range.

/// Radix support for integer literals.
///
/// See the per-radix modules for parse rules. The dispatcher at
/// [`crate::lexer::inner::scanner::parse::number::parse_number`] picks
/// the right module based on the leading prefix.
pub mod radix;

/// A numeric literal value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberLiteral {
    /// An integer
    Integer(IntegerLiteral),
    /// A float
    Float(FloatLiteral),
}

/// Integer literal with explicit radix.
///
/// `radix` is one of `2`, `8`, `10`, or `16`. `value` is the digit
/// sequence **without** the `0b`/`0o`/`0x` prefix and **without**
/// underscore separators (those are stripped by the parser).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: String,
    pub radix: u8,
    pub has_underscore: bool,
}

/// Float literal.
///
/// `value` includes the digits, decimal point, and exponent suffix
/// (e.g. `"1.5e10"`). `has_decimal` is `true` for any literal that
/// contains a `.` or an `e`/`E` exponent; integers that fit in 64 bits
/// are kept as [`IntegerLiteral`] instead.
#[derive(Debug, Clone, PartialEq, Eq)]
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
