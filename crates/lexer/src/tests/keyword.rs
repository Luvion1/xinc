//! Keyword tests.

use crate::tokenize;

#[test]
fn test_keyword_fn() {
    assert!(!tokenize("fn").unwrap().is_empty());
}
#[test]
fn test_keyword_let() {
    assert!(!tokenize("let").unwrap().is_empty());
}
#[test]
fn test_keyword_if() {
    assert!(!tokenize("if").unwrap().is_empty());
}
#[test]
fn test_keyword_else() {
    assert!(!tokenize("else").unwrap().is_empty());
}
#[test]
fn test_keyword_while() {
    assert!(!tokenize("while").unwrap().is_empty());
}
#[test]
fn test_keyword_true() {
    assert!(!tokenize("true").unwrap().is_empty());
}
#[test]
fn test_keyword_false() {
    assert!(!tokenize("false").unwrap().is_empty());
}
#[test]
fn test_keyword_return() {
    assert!(!tokenize("return").unwrap().is_empty());
}
#[test]
fn test_keyword_null() {
    assert!(!tokenize("null").unwrap().is_empty());
}
