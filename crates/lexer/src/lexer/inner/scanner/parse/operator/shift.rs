//! Comparison and shift operators.
//!
//! All operators in this file share a leading `<` or `>`. The two
//! public functions ([`parse_lt`] and [`parse_gt`]) consume the lead,
//! then peek at the next character(s) to decide between a comparison,
//! a shift, or a compound assignment.
//!
//! # State machine
//!
//! ```text
//! '<'  ─► '=' ─► Lte
//!       └► '<' ─► '=' ─► ShlAssign
//!             └►     ─► Shl
//!       └►        ─► Lt
//!
//! '>'  ─► '=' ─► Gte
//!       └► '>' ─► '=' ─► ShrAssign
//!             └►     ─► Shr
//!       └►        ─► Gt
//! ```
//!
//! Equality operators (`==`, `!=`) live in [`super::assign`] along with
//! the other compound-assignment forms; this file deliberately keeps
//! just the `<`/`>` family to keep the precedence of the file structure
//! mirroring the precedence of the language operators.

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