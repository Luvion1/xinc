//! Operator tests.

use crate::tokenize;

#[test]
fn test_binary_add() {
    assert!(!tokenize("+").unwrap().is_empty());
}
#[test]
fn test_binary_sub() {
    assert!(!tokenize("-").unwrap().is_empty());
}
#[test]
fn test_binary_mul() {
    assert!(!tokenize("*").unwrap().is_empty());
}
#[test]
fn test_binary_div() {
    assert!(!tokenize("/").unwrap().is_empty());
}
#[test]
fn test_binary_mod() {
    assert!(!tokenize("%").unwrap().is_empty());
}
#[test]
fn test_binary_and() {
    assert!(!tokenize("&").unwrap().is_empty());
}
#[test]
fn test_binary_or() {
    assert!(!tokenize("|").unwrap().is_empty());
}
#[test]
fn test_binary_xor() {
    assert!(!tokenize("^").unwrap().is_empty());
}
#[test]
fn test_binary_shl() {
    assert!(!tokenize("<<").unwrap().is_empty());
}
#[test]
fn test_binary_shr() {
    assert!(!tokenize(">>").unwrap().is_empty());
}
#[test]
fn test_unary_not() {
    assert!(!tokenize("!").unwrap().is_empty());
}
#[test]
fn test_unary_bitnot() {
    assert!(!tokenize("~").unwrap().is_empty());
}
#[test]
fn test_comparison_lt() {
    assert!(!tokenize("<").unwrap().is_empty());
}
#[test]
fn test_comparison_gt() {
    assert!(!tokenize(">").unwrap().is_empty());
}
#[test]
fn test_equality() {
    assert!(!tokenize("==").unwrap().is_empty());
}
#[test]
fn test_inequality() {
    assert!(!tokenize("!=").unwrap().is_empty());
}
#[test]
fn test_assign() {
    assert!(!tokenize("=").unwrap().is_empty());
}
