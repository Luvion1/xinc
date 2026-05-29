//! Statement parser tests.

use super::*;

#[test]
fn test_parse_let() {
    let stmts = parse_statement("let x = 1;").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_let_with_type() {
    let stmts = parse_statement("let x: i32 = 42;").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_let_string() {
    let stmts = parse_statement("let s = \"hello\";").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_let_bool() {
    let stmts = parse_statement("let flag = true;").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_if() {
    let stmts = parse_statement("if true { let x = 1; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_if_else() {
    let stmts = parse_statement("if x { y = 1; } else { y = 2; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_while() {
    let stmts = parse_statement("while true { x = 1; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_while_with_expr() {
    let stmts = parse_statement("while x > 0 { x = x - 1; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_fn_simple() {
    let stmts = parse_statement("fn main() {}").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_fn_with_params() {
    let stmts = parse_statement("fn add(a: i32, b: i32) { a + b; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_fn_with_return() {
    let stmts = parse_statement("fn get() -> i32 { return 42; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_assign() {
    let stmts = parse_statement("x = 1;").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_block() {
    let stmts = parse_statement("{ let x = 1; }").unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_multiple_statements() {
    let stmts = parse_statement("let x = 1; let y = 2;").unwrap();
    assert_eq!(stmts.len(), 2);
}
