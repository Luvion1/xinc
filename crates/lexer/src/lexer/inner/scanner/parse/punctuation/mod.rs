//! Punctuation token parsing.

use super::super::Scanner;
use crate::token::TokenKind;

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
        ':' => Some(parse_colon(scanner)),
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

fn parse_colon(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some(':') {
        s.advance();
        TokenKind::DoubleColon
    } else {
        TokenKind::Colon
    }
}
