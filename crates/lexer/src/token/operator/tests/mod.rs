//! Operator tests.

use super::{Operator, is_binary, is_operator_char, precedence};

#[test]
fn test_is_operator_char() {
    assert!(is_operator_char('+'));
    assert!(is_operator_char('='));
    assert!(!is_operator_char('a'));
}

#[test]
fn test_precedence() {
    assert!(precedence(Operator::Add) > precedence(Operator::Assign));
    assert!(precedence(Operator::Mul) > precedence(Operator::Add));
    assert!(precedence(Operator::And) > precedence(Operator::Or));
}

#[test]
fn test_is_binary() {
    assert!(is_binary(Operator::Add));
    assert!(!is_binary(Operator::Not));
}
