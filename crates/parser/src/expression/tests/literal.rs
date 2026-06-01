//! Literal expression tests.

use super::super::parser::*;
use xin_ast::{Expression, Literal};

#[test]
fn test_parse_number() {
    let expr = parse_expression("42").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::Number(_))));
}

#[test]
fn test_parse_string() {
    let expr = parse_expression("\"hello\"").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::String(_))));
}

#[test]
fn test_parse_bool() {
    let expr = parse_expression("true").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::Boolean(true))));
}

#[test]
fn test_parse_bool_false() {
    let expr = parse_expression("false").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::Boolean(false))));
}

#[test]
fn test_parse_string_literal_expr() {
    let expr = parse_expression("\"hello world\"").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::String(_))));
}
