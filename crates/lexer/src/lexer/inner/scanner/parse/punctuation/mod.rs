//! Punctuation token parsing.
//!
//! Every Xin's structural punctuation character is recognized by
//! [`parse_punctuation_token`]. The set covers:
//!
//! - Brackets: `( ) { } [ ]`
//! - Comma: `,`
//! - Semicolon: `;`
//! - Colon and double-colon: `:` / `::`
//! - At-sign: `@`
//! - Hash: `#`
//! - Underscore: `_` (treated as a punctuation token to keep it
//!   distinct from identifiers, since the grammar reserves `_` for
//!   pattern placeholders).
//!
//! `:` is special-cased through [`parse_colon`]: a single `:` produces
//! [`TokenKind::Colon`], while `::` produces [`TokenKind::DoubleColon`]
//! (used for path-style namespacing).
//!
//! Anything that is not one of the above returns `None`, leaving the
//! dispatcher's next branch to try operators or fall back to an
//! invalid-character error.

use super::super::Scanner;
use crate::token::TokenKind;

/// Try to parse a single punctuation token from `scanner`.
///
/// Returns `Some(TokenKind)` if the scanner's current character starts
/// a known punctuation symbol. Returns `None` for anything else
/// (including end-of-input).
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
