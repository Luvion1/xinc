//! Shift and comparison operators.
//!
//! Handles `<`, `<=`, `<<`, `<<=`, `>`, `>=`, `>>`, `>>=` shared
//! logic for both comparison and bit-shift operations.

use crate::token::TokenKind;
use crate::lexer::inner::scanner::Scanner;

/// Parse `<` → `Lt`, `<=` → `Lte`, or `<<`, `<<=`.
pub(super) fn parse_lt(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') {
        s.advance();
        TokenKind::Lte
    } else if s.current_char() == Some('<') {
        parse_shl(s)
    } else {
        TokenKind::Lt
    }
}

fn parse_shl(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::ShlAssign } else { TokenKind::Shl }
}

/// Parse `>` → `Gt`, `>=` → `Gte`, or `>>`, `>>=`.
pub(super) fn parse_gt(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') {
        s.advance();
        TokenKind::Gte
    } else if s.current_char() == Some('>') {
        parse_shr(s)
    } else {
        TokenKind::Gt
    }
}

fn parse_shr(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::ShrAssign } else { TokenKind::Shr }
}