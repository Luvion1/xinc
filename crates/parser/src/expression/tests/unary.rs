//! Unary expression tests.

use super::super::parser::*;
use xin_ast::{Expression, UnaryOp};

#[test]
fn test_parse_unary_not() {
    let expr = parse_expression("!true").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Not, .. }));
}

#[test]
fn test_parse_unary_minus() {
    let expr = parse_expression("-5").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Neg, .. }));
}

#[test]
fn test_parse_unary_bitnot() {
    let expr = parse_expression("~x").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::BitNot, .. }));
}

#[test]
fn test_parse_not_identifier() {
    let expr = parse_expression("!x").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Not, .. }));
}

#[test]
fn test_parse_function_call() {
    let expr = parse_expression("foo()").unwrap();
    if let Expression::Call { callee: box_expr, args } = expr {
        if let Expression::Identifier(name) = box_expr.as_ref() {
            assert_eq!(name, "foo");
        } else {
            panic!("Expected identifier callee");
        }
        assert!(args.is_empty());
    } else {
        panic!("Expected function call expression");
    }
}

#[test]
fn test_parse_function_call_with_args() {
    let expr = parse_expression("foo(1, 2)").unwrap();
    if let Expression::Call { callee: box_expr, args } = expr {
        assert!(matches!(box_expr.as_ref(), Expression::Identifier(_)));
        assert_eq!(args.len(), 2);
    } else {
        panic!("Expected function call expression");
    }
}
