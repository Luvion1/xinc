//! Identifier token handling.
//! Variable names, function names, and other identifiers.

/// Identifier token.
///
/// Represents a user-defined name in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// The identifier name as string
    pub name: String,
    /// Whether it contains any Unicode escapes
    pub has_escapes: bool,
}

impl Identifier {
    /// Create a new identifier.
    ///
    /// # Arguments
    /// * `name` - The identifier name
    ///
    /// # Returns
    /// New Identifier instance (no escapes by default)
    pub fn new(name: String) -> Self {
        Self { name, has_escapes: false }
    }

    /// Get the identifier name.
    ///
    /// # Returns
    /// String slice of the identifier
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if identifier starts with a digit.
    ///
    /// Invalid in most languages including Xin.
    ///
    /// # Returns
    /// True if name starts with digit
    pub fn starts_with_digit(&self) -> bool {
        self.name.chars().next().is_some_and(|c| c.is_ascii_digit())
    }

    /// Check if identifier is a keyword.
    ///
    /// Even though keywords are not identifiers, this can
    /// check if the name matches a reserved word.
    ///
    /// # Returns
    /// Some(keyword) if it's a keyword, None otherwise
    pub fn as_keyword(&self) -> Option<super::keyword::Keyword> {
        super::keyword::from_str(&self.name)
    }
}

/// Check if a character is a valid identifier start.
///
/// Xin identifiers must start with a letter or underscore.
/// Unicode letters are allowed.
///
/// # Arguments
/// * `c` - Character to test
///
/// # Returns
/// True if valid start char
pub fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || (c as u32) >= 0x80
}

/// Check if a character is a valid identifier continue.
///
/// After first char, digits are also allowed.
///
/// # Arguments
/// * `c` - Character to test
///
/// # Returns
/// True if valid continuation char
pub fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_identifier() {
        let id = Identifier::new("foo".to_string());
        assert_eq!(id.name(), "foo");
        assert!(!id.has_escapes);
    }

    #[test]
    fn test_starts_with_digit() {
        let id = Identifier::new("123".to_string());
        assert!(id.starts_with_digit());
    }

    #[test]
    fn test_as_keyword() {
        let id = Identifier::new("fn".to_string());
        assert!(id.as_keyword().is_some());
    }

    #[test]
    fn test_identifier_start() {
        assert!(is_identifier_start('a'));
        assert!(is_identifier_start('_'));
        assert!(!is_identifier_start('1'));
    }

    #[test]
    fn test_identifier_continue() {
        assert!(is_identifier_continue('a'));
        assert!(is_identifier_continue('1'));
        assert!(!is_identifier_continue('!'));
    }
}
