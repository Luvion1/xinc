//! Punctuation tests.

use crate::tokenize;

#[test]
fn test_punctuation_lparen() {
    assert!(!tokenize("(").unwrap().is_empty());
}
#[test]
fn test_punctuation_rparen() {
    assert!(!tokenize(")").unwrap().is_empty());
}
#[test]
fn test_punctuation_lbrace() {
    assert!(!tokenize("{").unwrap().is_empty());
}
#[test]
fn test_punctuation_rbrace() {
    assert!(!tokenize("}").unwrap().is_empty());
}
#[test]
fn test_punctuation_lbracket() {
    assert!(!tokenize("[").unwrap().is_empty());
}
#[test]
fn test_punctuation_rbracket() {
    assert!(!tokenize("]").unwrap().is_empty());
}
#[test]
fn test_semicolon() {
    assert!(!tokenize(";").unwrap().is_empty());
}
#[test]
fn test_colon() {
    assert!(!tokenize(":").unwrap().is_empty());
}
#[test]
fn test_comma() {
    assert!(!tokenize(",").unwrap().is_empty());
}
#[test]
fn test_dot() {
    assert!(!tokenize(".").unwrap().is_empty());
}
