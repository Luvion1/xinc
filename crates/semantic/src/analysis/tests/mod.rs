//! Semantic analyzer tests.

#![cfg_attr(not(test), allow(unused_imports))]

mod expression;
mod statement;

use crate::{Analyzer, Symbol, SymbolTable};
use xin_ast::{Expression, Literal, Statement};

#[test]
fn test_symbol_table() {
    let mut table = SymbolTable::new();
    table.insert("x".to_string(), Symbol { ty: Some("i32".to_string()), mutable: true });
    assert!(table.lookup("x").is_some());
}

#[test]
fn test_symbol_table_not_found() {
    let table = SymbolTable::new();
    assert!(table.lookup("missing").is_none());
}

#[test]
fn test_symbol_table_mutable() {
    let mut table = SymbolTable::new();
    table.insert("x".to_string(), Symbol { ty: Some("i32".to_string()), mutable: true });
    assert!(table.lookup("x").unwrap().mutable);
}

#[test]
fn test_undefined_variable() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Expr(Expression::Identifier("undefined".to_string()));
    assert!(analyzer.analyze(&stmt).is_err());
}

#[test]
fn test_analyzer_assign() {
    let mut analyzer = Analyzer::new();
    let stmt = Statement::Assign {
        target: "x".to_string(),
        value: Expression::Literal(Literal::Number("10".to_string())),
    };
    assert!(analyzer.analyze(&stmt).is_err());
}

#[test]
fn test_analyzer_default() {
    let analyzer = Analyzer::default();
    let _ = analyzer;
}
