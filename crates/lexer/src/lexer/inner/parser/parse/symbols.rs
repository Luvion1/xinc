//! Operator and punctuation token recognition.

use super::super::scanner::Scanner;
use crate::token::TokenKind;

/// Attempt to parse an operator token.
///
/// Recognizes multi-character operators using maximal munch.
/// Returns `Some(TokenKind)` if an operator is matched, `None` otherwise.
pub fn parse_operator_token(scanner: &mut Scanner) -> Option<TokenKind> {
    let c = scanner.current_char()?;
    match c {
        '+' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::AddAssign)
            } else {
                Some(TokenKind::Plus)
            }
        }
        '-' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::SubAssign)
            } else if scanner.current_char() == Some('>') {
                scanner.advance();
                Some(TokenKind::Arrow)
            } else {
                Some(TokenKind::Minus)
            }
        }
        '*' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::MulAssign)
            } else {
                Some(TokenKind::Star)
            }
        }
        '/' => {
            scanner.advance();
            Some(TokenKind::Slash)
        }
        '%' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::ModAssign)
            } else {
                Some(TokenKind::Percent)
            }
        }
        '=' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::Eq)
            } else {
                Some(TokenKind::Assign)
            }
        }
        '!' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::Neq)
            } else {
                Some(TokenKind::Not)
            }
        }
        '<' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::Lte)
            } else if scanner.current_char() == Some('<') {
                scanner.advance();
                if scanner.current_char() == Some('=') {
                    scanner.advance();
                    Some(TokenKind::ShlAssign)
                } else {
                    Some(TokenKind::Shl)
                }
            } else {
                Some(TokenKind::Lt)
            }
        }
        '>' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::Gte)
            } else if scanner.current_char() == Some('>') {
                scanner.advance();
                if scanner.current_char() == Some('=') {
                    scanner.advance();
                    Some(TokenKind::ShrAssign)
                } else {
                    Some(TokenKind::Shr)
                }
            } else {
                Some(TokenKind::Gt)
            }
        }
        '&' => {
            scanner.advance();
            if scanner.current_char() == Some('&') {
                scanner.advance();
                Some(TokenKind::And)
            } else if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::AndAssign)
            } else {
                Some(TokenKind::BitAnd)
            }
        }
        '|' => {
            scanner.advance();
            if scanner.current_char() == Some('|') {
                scanner.advance();
                Some(TokenKind::Or)
            } else if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::OrAssign)
            } else {
                Some(TokenKind::BitOr)
            }
        }
        '^' => {
            scanner.advance();
            if scanner.current_char() == Some('=') {
                scanner.advance();
                Some(TokenKind::XorAssign)
            } else {
                Some(TokenKind::BitXor)
            }
        }
        '~' => {
            scanner.advance();
            Some(TokenKind::BitNot)
        }
        '.' => {
            scanner.advance();
            if scanner.current_char() == Some('.') {
                scanner.advance();
                if scanner.current_char() == Some('=') {
                    scanner.advance();
                    Some(TokenKind::RangeInclusive)
                } else {
                    Some(TokenKind::Range)
                }
            } else {
                Some(TokenKind::Dot)
            }
        }
        '?' => {
            scanner.advance();
            if scanner.current_char() == Some('?') {
                scanner.advance();
                Some(TokenKind::Coalesce)
            } else if scanner.current_char() == Some('.') {
                scanner.advance();
                Some(TokenKind::OptionChain)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Parse a punctuation token.
///
/// Returns `Some(TokenKind)` if a punctuation symbol is matched, `None` otherwise.
pub fn parse_punctuation_token(scanner: &mut Scanner) -> Option<TokenKind> {
    let c = scanner.current_char()?;
    match c {
        '(' => {
            scanner.advance();
            Some(TokenKind::LParen)
        }
        ')' => {
            scanner.advance();
            Some(TokenKind::RParen)
        }
        '{' => {
            scanner.advance();
            Some(TokenKind::LBrace)
        }
        '}' => {
            scanner.advance();
            Some(TokenKind::RBrace)
        }
        '[' => {
            scanner.advance();
            Some(TokenKind::LBracket)
        }
        ']' => {
            scanner.advance();
            Some(TokenKind::RBracket)
        }
        ',' => {
            scanner.advance();
            Some(TokenKind::Comma)
        }
        ';' => {
            scanner.advance();
            Some(TokenKind::Semicolon)
        }
        ':' => {
            scanner.advance();
            if scanner.current_char() == Some(':') {
                scanner.advance();
                Some(TokenKind::DoubleColon)
            } else {
                Some(TokenKind::Colon)
            }
        }
        '@' => {
            scanner.advance();
            Some(TokenKind::At)
        }
        '#' => {
            scanner.advance();
            Some(TokenKind::Hash)
        }
        '_' => {
            scanner.advance();
            Some(TokenKind::Underscore)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_plus() {
        let mut s = Scanner::new("+");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::Plus));
    }

    #[test]
    fn test_operator_add_assign() {
        let mut s = Scanner::new("+=");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::AddAssign));
    }

    #[test]
    fn test_operator_range() {
        let mut s = Scanner::new("..");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::Range));
    }

    #[test]
    fn test_operator_range_inclusive() {
        let mut s = Scanner::new("..=");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::RangeInclusive));
    }

    #[test]
    fn test_operator_coalesce() {
        let mut s = Scanner::new("??");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::Coalesce));
    }

    #[test]
    fn test_operator_option_chain() {
        let mut s = Scanner::new("?.");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::OptionChain));
    }

    #[test]
    fn test_punctuation_paren() {
        let mut s = Scanner::new("(");
        assert_eq!(parse_punctuation_token(&mut s), Some(TokenKind::LParen));
    }

    #[test]
    fn test_punctuation_double_colon() {
        let mut s = Scanner::new("::");
        assert_eq!(parse_punctuation_token(&mut s), Some(TokenKind::DoubleColon));
    }

    #[test]
    fn test_operator_dot_alone() {
        let mut s = Scanner::new(".");
        assert_eq!(parse_operator_token(&mut s), Some(TokenKind::Dot));
    }

    #[test]
    fn test_punctuation_at_hash() {
        let mut s = Scanner::new("@");
        assert_eq!(parse_punctuation_token(&mut s), Some(TokenKind::At));
        let mut s2 = Scanner::new("#");
        assert_eq!(parse_punctuation_token(&mut s2), Some(TokenKind::Hash));
    }
}
