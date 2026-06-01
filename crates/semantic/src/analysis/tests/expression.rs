//! Expression tests.

#![cfg_attr(not(test), allow(unused_imports))]

use crate::Analyzer;
use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

#[test]
fn test_analyzer_unary() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Expr(Expression::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expression::Literal(Literal::Boolean(true))),
    });
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_function_call() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Expr(Expression::Call {
        callee: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
        args: vec![],
    });
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_binop_chain() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Expr(Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
        op: BinaryOp::Add,
        right: Box::new(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
            op: BinaryOp::Mul,
            right: Box::new(Expression::Literal(Literal::Number("3".to_string()))),
        }),
    });
    analyzer.analyze(&stmt).unwrap();
}
