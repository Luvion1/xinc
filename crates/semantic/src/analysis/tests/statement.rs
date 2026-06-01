//! Statement tests.

#![cfg_attr(not(test), allow(unused_imports))]

use crate::Analyzer;
use xin_ast::{Expression, Literal, Statement, Type};

#[test]
fn test_analyzer_let() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Let {
        name: "x".to_string(),
        ty: None,
        value: Expression::Literal(Literal::Number("42".to_string())),
    };
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_let_with_type() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Let {
        name: "x".to_string(),
        ty: Some(Type::Builtin(xin_ast::BuiltinType::I32)),
        value: Expression::Literal(Literal::Number("42".to_string())),
    };
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_expr_statement() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Expr(Expression::Literal(Literal::Boolean(true)));
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_block() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Block(vec![]);
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_return() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Return(None);
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_if() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::If {
        cond: Expression::Literal(Literal::Boolean(true)),
        then: vec![],
        r#else: None,
    };
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_if_else() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::If {
        cond: Expression::Literal(Literal::Boolean(true)),
        then: vec![],
        r#else: Some(Box::new(Statement::Return(None))),
    };
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_nested_block() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Block(vec![Statement::Let {
        name: "x".to_string(),
        ty: None,
        value: Expression::Literal(Literal::Number("1".to_string())),
    }]);
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_while() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::While { cond: Expression::Literal(Literal::Boolean(true)), body: vec![] };
    analyzer.analyze(&stmt).unwrap();
}

#[test]
fn test_analyzer_while_with_body() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::While {
        cond: Expression::Literal(Literal::Boolean(true)),
        body: vec![Statement::Expr(Expression::Literal(Literal::Number("1".to_string())))],
    };
    analyzer.analyze(&stmt).unwrap();
}
