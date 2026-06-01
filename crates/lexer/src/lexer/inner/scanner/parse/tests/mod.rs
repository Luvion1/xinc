//! Tests for symbol parsing.

use super::{operator::parse_operator_token, punctuation::parse_punctuation_token};
use super::super::Scanner;
use crate::token::TokenKind;

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