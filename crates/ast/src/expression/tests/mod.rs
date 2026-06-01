//! Expression tests.

use super::{BinaryOp, Expression, Literal, UnaryOp};

#[test]
fn test_literal_default_is_null() {
    let lit = Literal::default();
    assert_eq!(lit, Literal::Null);
}

#[test]
fn test_expression_clone() {
    let expr = Expression::Literal(Literal::Number("42".to_string()));
    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_binary_op_eq() {
    let left = Expression::Literal(Literal::Number("1".to_string()));
    let right = Expression::Literal(Literal::Number("2".to_string()));
    let expr = Expression::Binary {
        left: Box::new(left.clone()),
        op: BinaryOp::Add,
        right: Box::new(right.clone()),
    };
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Add, .. }));
}

#[test]
fn test_unary_op_neg() {
    let operand = Expression::Literal(Literal::Number("5".to_string()));
    let expr = Expression::Unary { op: UnaryOp::Neg, operand: Box::new(operand) };
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Neg, .. }));
}

#[test]
fn test_call_expression() {
    let callee = Expression::Identifier("print".to_string());
    let expr = Expression::Call {
        callee: Box::new(callee),
        args: vec![Expression::Literal(Literal::String("hi".to_string()))],
    };
    assert!(matches!(expr, Expression::Call { .. }));
}

#[test]
fn test_string_tests() {
    let lit = Literal::String("".to_string());
    assert_eq!(lit, Literal::String("".to_string()));
    let lit = Literal::String("hello\\nworld".to_string());
    assert_eq!(lit, Literal::String("hello\\nworld".to_string()));
    let lit = Literal::String("日本語".to_string());
    assert_eq!(lit, Literal::String("日本語".to_string()));
}

#[test]
fn test_string_escapes() {
    let lit = Literal::String("a\\\\b".to_string());
    assert_eq!(lit, Literal::String("a\\\\b".to_string()));
    let lit = Literal::String("a\\tb".to_string());
    assert_eq!(lit, Literal::String("a\\tb".to_string()));
    let lit = Literal::String("a\\rb".to_string());
    assert_eq!(lit, Literal::String("a\\rb".to_string()));
    let lit = Literal::String("a\\0b".to_string());
    assert_eq!(lit, Literal::String("a\\0b".to_string()));
    let lit = Literal::String("a\\'b".to_string());
    assert_eq!(lit, Literal::String("a\\'b".to_string()));
    let lit = Literal::String("a\\\"b".to_string());
    assert_eq!(lit, Literal::String("a\\\"b".to_string()));
    let lit = Literal::String("a\\xab".to_string());
    assert_eq!(lit, Literal::String("a\\xab".to_string()));
}
