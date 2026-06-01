//! Operator token parsing.
//!
//! Dispatches single-char operator heads to per-operator parsers.
//! Each helper handles the primary token plus any compound forms
//! (e.g. `+=`, `==`, `<<=`) by peeking the next character.

mod assign;
mod shift;
mod single;

use assign::{parse_assign, parse_not};
use crate::token::TokenKind;
use crate::lexer::inner::scanner::Scanner;
use shift::{parse_lt, parse_gt};
use single::{parse_and, parse_dot, parse_minus, parse_plus, parse_question, parse_star, parse_xor, parse_or, parse_percent};

/// Attempt to parse an operator token.
pub fn parse_operator_token(scanner: &mut Scanner) -> Option<TokenKind> {
    let c = scanner.current_char()?;
    match c {
        '+' => Some(parse_plus(scanner)),
        '-' => Some(parse_minus(scanner)),
        '*' => Some(parse_star(scanner)),
        '/' => Some(TokenKind::Slash),
        '%' => Some(parse_percent(scanner)),
        '=' => Some(parse_assign(scanner)),
        '!' => Some(parse_not(scanner)),
        '<' => Some(parse_lt(scanner)),
        '>' => Some(parse_gt(scanner)),
        '&' => Some(parse_and(scanner)),
        '|' => Some(parse_or(scanner)),
        '^' => Some(parse_xor(scanner)),
        '~' => Some(TokenKind::BitNot),
        '.' => Some(parse_dot(scanner)),
        '?' => parse_question(scanner),
        _ => None,
    }
}
