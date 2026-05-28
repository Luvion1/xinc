//! Punctuation tokens.
//! Brackets, semicolons, commas, and other separators.

/// Punctuation tokens in Xin.
///
/// These tokens mark structure and separate elements in the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    /// Parentheses: (
    LParen,
    /// Parentheses: )
    RParen,
    /// Curly brace: {
    LBrace,
    /// Curly brace: }
    RBrace,
    /// Square bracket: [
    LBracket,
    /// Square bracket: ]
    RBracket,
    /// Semicolon: ;
    Semicolon,
    /// Colon: :
    Colon,
    /// Comma: ,
    Comma,
    /// Dot: .
    Dot,
    /// Arrow: ->
    Arrow,
    /// Fat arrow: =>
    FatArrow,
    /// Range: ..
    Range,
    /// Range exclusive: ..<
    RangeExclusive,
    /// At sign: @
    At,
    /// Hash/pound: #
    Hash,
    /// Double colon: ::
    DoubleColon,
}

/// Get punctuation from a character.
///
/// Maps single-character punctuation to its token.
/// Multi-char punctuations are handled separately.
///
/// # Arguments
/// * `c` - Character to check
///
/// # Returns
/// Some(Punctuation) if recognized, None otherwise
pub fn from_char(c: char) -> Option<Punctuation> {
    match c {
        '(' => Some(Punctuation::LParen),
        ')' => Some(Punctuation::RParen),
        '{' => Some(Punctuation::LBrace),
        '}' => Some(Punctuation::RBrace),
        '[' => Some(Punctuation::LBracket),
        ']' => Some(Punctuation::RBracket),
        ';' => Some(Punctuation::Semicolon),
        ':' => Some(Punctuation::Colon),
        ',' => Some(Punctuation::Comma),
        '.' => Some(Punctuation::Dot),
        '@' => Some(Punctuation::At),
        '#' => Some(Punctuation::Hash),
        _ => None,
    }
}

/// Check if a character is punctuation.
///
/// # Arguments
/// * `c` - Character to test
///
/// # Returns
/// True if the character is punctuation
pub fn is_punctuation(c: char) -> bool {
    from_char(c).is_some()
}

/// Get string representation of punctuation.
///
/// # Arguments
/// * `p` - Punctuation token
///
/// # Returns
/// String slice for that punctuation
pub fn as_str(p: Punctuation) -> &'static str {
    match p {
        Punctuation::LParen => "(",
        Punctuation::RParen => ")",
        Punctuation::LBrace => "{",
        Punctuation::RBrace => "}",
        Punctuation::LBracket => "[",
        Punctuation::RBracket => "]",
        Punctuation::Semicolon => ";",
        Punctuation::Colon => ":",
        Punctuation::Comma => ",",
        Punctuation::Dot => ".",
        Punctuation::Arrow => "->",
        Punctuation::FatArrow => "=>",
        Punctuation::Range => "..",
        Punctuation::RangeExclusive => "..<",
        Punctuation::At => "@",
        Punctuation::Hash => "#",
        Punctuation::DoubleColon => "::",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_char_paren() {
        assert_eq!(from_char('('), Some(Punctuation::LParen));
        assert_eq!(from_char(')'), Some(Punctuation::RParen));
    }

    #[test]
    fn test_from_char_brace() {
        assert_eq!(from_char('{'), Some(Punctuation::LBrace));
        assert_eq!(from_char('}'), Some(Punctuation::RBrace));
    }

    #[test]
    fn test_is_punctuation() {
        assert!(is_punctuation(';'));
        assert!(is_punctuation(','));
        assert!(!is_punctuation('a'));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(as_str(Punctuation::Semicolon), ";");
        assert_eq!(as_str(Punctuation::Arrow), "->");
    }
}
