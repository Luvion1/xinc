//! Null literal token.
//! The null value represents absence of a value.

/// The null literal value.
///
/// In Xin, `null` is a valid value that indicates
/// the absence of a value. Unlike some languages,
/// null is a proper type-safe value of its own type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NullLiteral;

impl NullLiteral {
    /// Create a new null literal.
    ///
    /// There is only one null value.
    pub fn new() -> Self {
        NullLiteral
    }

    /// Check if a value is null.
    ///
    /// Always returns true for NullLiteral.
    pub fn is_null(&self) -> bool {
        true
    }

    /// Parse the "null" keyword into a null literal.
    ///
    /// # Returns
    /// Some(NullLiteral) if input is "null", None otherwise
    pub fn parse(input: &str) -> Option<NullLiteral> {
        if input == "null" { Some(NullLiteral) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let n = NullLiteral::new();
        assert!(n.is_null());
    }

    #[test]
    fn test_parse_valid() {
        assert!(NullLiteral::parse("null").is_some());
    }

    #[test]
    fn test_parse_invalid() {
        assert!(NullLiteral::parse("NULL").is_none());
        assert!(NullLiteral::parse("nil").is_none());
    }
}
