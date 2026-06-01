//! Binary expression tests.

use super::super::parser::*;
use xin_ast::{BinaryOp, Expression, Literal};

fn as_binary<'a>(expr: &'a Expression) -> (&'a BinaryOp, &'a Expression, &'a Expression) {
    match expr {
        Expression::Binary { op, left, right } => (op, left.as_ref(), right.as_ref()),
        _ => panic!("expected binary, got {:?}", expr),
    }
}

fn assert_binary<'a>(
    expr: &'a Expression,
    expected: BinaryOp,
) -> (&'a Expression, &'a Expression) {
    let (op, left, right) = as_binary(expr);
    assert_eq!(*op, expected, "expected {:?}, got {:?}", expected, op);
    (left, right)
}

#[test]
fn test_precedence_mul_over_add() {
    let expr = parse_expression("1 + 2 * 3").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Add);
    assert!(matches!(left, Expression::Literal(Literal::Number(n)) if n == "1"));
    let (_op, rl, rr) = as_binary(right);
    assert!(matches!(rl, Expression::Literal(Literal::Number(n)) if n == "2"));
    assert!(matches!(rr, Expression::Literal(Literal::Number(n)) if n == "3"));
}

#[test]
fn test_precedence_add_over_eq() {
    let expr = parse_expression("a == b + c").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Eq);
    assert!(matches!(left, Expression::Identifier(name) if name == "a"));
    let _ = assert_binary(right, BinaryOp::Add);
}

#[test]
fn test_left_associative_add() {
    let expr = parse_expression("1 + 2 + 3").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Add);
    let (l1, r1) = assert_binary(left, BinaryOp::Add);
    assert!(matches!(l1, Expression::Literal(Literal::Number(n)) if n == "1"));
    assert!(matches!(r1, Expression::Literal(Literal::Number(n)) if n == "2"));
    assert!(matches!(right, Expression::Literal(Literal::Number(n)) if n == "3"));
}

#[test]
fn test_left_associative_sub() {
    let expr = parse_expression("10 - 5 - 2").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Sub);
    let (l1, r1) = assert_binary(left, BinaryOp::Sub);
    assert!(matches!(l1, Expression::Literal(Literal::Number(n)) if n == "10"));
    assert!(matches!(r1, Expression::Literal(Literal::Number(n)) if n == "5"));
    assert!(matches!(right, Expression::Literal(Literal::Number(n)) if n == "2"));
}

#[test]
fn test_precedence_mul_over_sub() {
    let expr = parse_expression("10 - 4 / 2").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Sub);
    assert!(matches!(left, Expression::Literal(Literal::Number(n)) if n == "10"));
    let _ = assert_binary(right, BinaryOp::Div);
}

#[test]
fn test_precedence_paren_overrides() {
    let expr = parse_expression("(1 + 2) * 3").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Mul);
    assert!(matches!(right, Expression::Literal(Literal::Number(n)) if n == "3"));
    let (ll, _rr) = assert_binary(left, BinaryOp::Add);
    assert!(matches!(ll, Expression::Literal(Literal::Number(n)) if n == "1"));
}

#[test]
fn test_precedence_mul_chain() {
    let expr = parse_expression("2 * 3 * 4").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Mul);
    assert!(matches!(left, Expression::Literal(Literal::Number(n)) if n == "2"));
    assert_binary(left, BinaryOp::Mul);
    assert!(matches!(right, Expression::Literal(Literal::Number(n)) if n == "4"));
}

#[test]
fn test_precedence_mixed_depth() {
    let expr = parse_expression("a + b * c - d / e").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Sub);
    assert!(matches!(left, Expression::Identifier(name) if name == "a"));
    assert!(matches!(right, Expression::Identifier(name) if name == "d"));
}

#[test]
fn test_parse_binary_add() {
    let expr = parse_expression("1 + 2").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_binary_sub() {
    let expr = parse_expression("a - b").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_binary_mul() {
    let expr = parse_expression("3 * 4").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_binary_div() {
    let expr = parse_expression("10 / 2").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_binary_mod() {
    let expr = parse_expression("10 % 3").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Mod, .. }));
}

#[test]
fn test_parse_binary_and() {
    let expr = parse_expression("a & b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::BitAnd, .. }));
}

#[test]
fn test_parse_binary_or() {
    let expr = parse_expression("a | b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::BitOr, .. }));
}

#[test]
fn test_parse_logical_and() {
    let expr = parse_expression("a && b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::And, .. }));
}

#[test]
fn test_parse_logical_or() {
    let expr = parse_expression("a || b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Or, .. }));
}

#[test]
fn test_parse_binary_xor() {
    let expr = parse_expression("a ^ b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::BitXor, .. }));
}

#[test]
fn test_parse_binary_shl() {
    let expr = parse_expression("1 << 4").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Shl, .. }));
}

#[test]
fn test_parse_binary_shr() {
    let expr = parse_expression("8 >> 2").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Shr, .. }));
}

#[test]
fn test_parse_comparison_lt() {
    let expr = parse_expression("a < b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Lt, .. }));
}

#[test]
fn test_parse_comparison_gt() {
    let expr = parse_expression("a > b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Gt, .. }));
}

#[test]
fn test_parse_comparison_le() {
    let expr = parse_expression("a <= b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Le, .. }));
}

#[test]
fn test_parse_comparison_ge() {
    let expr = parse_expression("a >= b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Ge, .. }));
}

#[test]
fn test_precedence_le_lt() {
    let expr = parse_expression("a + b <= c").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Le);
    let (_rl, _rr) = assert_binary(left, BinaryOp::Add);
    assert!(matches!(right, Expression::Identifier(n) if n == "c"));
}

#[test]
fn test_precedence_ge_gt() {
    let expr = parse_expression("a * b >= c").unwrap();
    let (left, right) = assert_binary(&expr, BinaryOp::Ge);
    let (rl, rr) = assert_binary(left, BinaryOp::Mul);
    assert!(matches!(right, Expression::Identifier(n) if n == "c"));
    assert!(matches!(rl, Expression::Identifier(n) if n == "a"));
    assert!(matches!(rr, Expression::Identifier(n) if n == "b"));
}

#[test]
fn test_parse_equality() {
    let expr = parse_expression("a == b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Eq, .. }));
}

#[test]
fn test_parse_inequality() {
    let expr = parse_expression("a != b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Neq, .. }));
}
