//! Expression parser tests.

mod binary;
mod literal;
mod unary;

use super::parser::*;
#[allow(unused_imports)]
use xin_ast::{BinaryOp, Expression, Literal, UnaryOp};

#[test]
fn test_parse_identifier() {
    let expr = parse_expression("x").unwrap();
    assert!(matches!(expr, Expression::Identifier(_)));
}

#[test]
fn test_parse_paren() {
    let expr = parse_expression("(x)").unwrap();
    assert!(matches!(expr, Expression::Identifier(_)));
}

#[test]
fn test_parse_nested_parens() {
    let expr = parse_expression("((x))").unwrap();
    assert!(matches!(expr, Expression::Identifier(_)));
}

#[test]
fn test_parse_negative_paren() {
    let expr = parse_expression("-(x)").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Neg, .. }));
}

#[test]
fn test_parse_empty_input() {
    let result = parse_expression("");
    assert!(result.is_err());
}

#[test]
fn test_parse_unknown_token() {
    let result = parse_expression("@");
    assert!(result.is_err());
}

#[test]
fn test_parse_chain() {
    let expr = parse_expression("1 + 2 + 3").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}
