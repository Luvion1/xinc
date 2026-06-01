//! Assignment and equality operators.
//!
//! Handles `=`, `==`, `!=`, plus related forms that share the leading
//! `'='` or `'!'` character.

use crate::token::TokenKind;
use crate::lexer::inner::scanner::Scanner;

/// Parse `=` → `Assign` or `+=` → `Eq`.
pub(super) fn parse_assign(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') {
        s.advance();
        TokenKind::Eq
    } else {
        TokenKind::Assign
    }
}

/// Parse `!` → `Not` or `!=` → `Neq`.
pub(super) fn parse_not(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') {
        s.advance();
        TokenKind::Neq
    } else {
        TokenKind::Not
    }
}