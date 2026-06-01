//! Codegen tests.

use super::{CodegenError, generate};
use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

#[test]
fn test_codegen_empty() {
    let code = generate(&[]).unwrap();
    assert!(code.is_empty());
}

#[test]
fn test_codegen_error_debug() {
    let err = CodegenError::InvalidStatement;
    assert!(format!("{:?}", err).contains("Invalid"));
}

#[test]
fn test_codegen_error_display() {
    let err = CodegenError::InvalidStatement;
    assert_eq!(format!("{}", err), "Invalid statement for codegen");
}

#[test]
fn test_codegen_literal() {
    let stmt = Statement::Expr(Expression::Literal(Literal::Number("42".to_string())));
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("42"));
}

#[test]
fn test_codegen_binary() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
        op: BinaryOp::Add,
        right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(1 + 2)"));
}

#[test]
fn test_codegen_unary() {
    let stmt = Statement::Expr(Expression::Unary {
        op: UnaryOp::Neg,
        operand: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("-5"));
}

#[test]
fn test_codegen_bitwise() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
        op: BinaryOp::BitAnd,
        right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(1 & 2)"));
}

#[test]
fn test_codegen_shift() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("8".to_string()))),
        op: BinaryOp::Shr,
        right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(8 >> 2)"));
}

#[test]
fn test_codegen_logical_and() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Boolean(true))),
        op: BinaryOp::And,
        right: Box::new(Expression::Literal(Literal::Boolean(false))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(true && false)"));
}

#[test]
fn test_codegen_logical_or() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Boolean(true))),
        op: BinaryOp::Or,
        right: Box::new(Expression::Literal(Literal::Boolean(false))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(true || false)"));
}

#[test]
fn test_codegen_bitxor() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
        op: BinaryOp::BitXor,
        right: Box::new(Expression::Literal(Literal::Number("3".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(5 ^ 3)"));
}

#[test]
fn test_codegen_bitor() {
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
        op: BinaryOp::BitOr,
        right: Box::new(Expression::Literal(Literal::Number("3".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("(5 | 3)"));
}

#[test]
fn test_codegen_bitnot() {
    let stmt = Statement::Expr(Expression::Unary {
        op: UnaryOp::BitNot,
        operand: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
    });
    let code = generate(&[stmt]).unwrap();
    assert!(code.contains("~5"));
}
