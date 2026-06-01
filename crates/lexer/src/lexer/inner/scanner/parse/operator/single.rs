//! Single-char and simple compound operators.
//!
//! Handles `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `~`, `.`, `?`
//! and their common two- and three-character compound forms.

use crate::token::TokenKind;
use crate::lexer::inner::scanner::Scanner;

/// Parse `+` → `Plus` or `+=` → `AddAssign`.
pub(super) fn parse_plus(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::AddAssign } else { TokenKind::Plus }
}

/// Parse `-` → `Minus`, `-=` → `SubAssign`, or `->` → `Arrow`.
pub(super) fn parse_minus(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::SubAssign }
    else if s.current_char() == Some('>') { s.advance(); TokenKind::Arrow }
    else { TokenKind::Minus }
}

/// Parse `*` → `Star` or `*=` → `MulAssign`.
pub(super) fn parse_star(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::MulAssign } else { TokenKind::Star }
}

/// Parse `%` → `Percent` or `%=` → `ModAssign`.
pub(super) fn parse_percent(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::ModAssign } else { TokenKind::Percent }
}

/// Parse `&` → `BitAnd`, `&&` → `And`, or `&=` → `AndAssign`.
pub(super) fn parse_and(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('&') { s.advance(); TokenKind::And }
    else if s.current_char() == Some('=') { s.advance(); TokenKind::AndAssign }
    else { TokenKind::BitAnd }
}

/// Parse `|` → `BitOr`, `||` → `Or`, or `|=` → `OrAssign`.
pub(super) fn parse_or(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('|') { s.advance(); TokenKind::Or }
    else if s.current_char() == Some('=') { s.advance(); TokenKind::OrAssign }
    else { TokenKind::BitOr }
}

/// Parse `^` → `BitXor` or `^=` → `XorAssign`.
pub(super) fn parse_xor(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('=') { s.advance(); TokenKind::XorAssign } else { TokenKind::BitXor }
}

/// Parse `.` → `Dot`, `..` → `Range`, or `..=` → `RangeInclusive`.
pub(super) fn parse_dot(s: &mut Scanner) -> TokenKind {
    s.advance();
    if s.current_char() == Some('.') {
        s.advance();
        if s.current_char() == Some('=') { s.advance(); TokenKind::RangeInclusive }
        else { TokenKind::Range }
    } else { TokenKind::Dot }
}

/// Parse `?` → continue, `??` → `Coalesce`, or `?.` → `OptionChain`.
pub(super) fn parse_question(s: &mut Scanner) -> Option<TokenKind> {
    s.advance();
    if s.current_char() == Some('?') { s.advance(); Some(TokenKind::Coalesce) }
    else if s.current_char() == Some('.') { s.advance(); Some(TokenKind::OptionChain) }
    else { None }
}