//! Expression parser tests.

use super::parser::*;
use xin_ast::{Expression, Literal, BinaryOp, UnaryOp};

#[test]
fn test_parse_identifier() {
    let expr = parse_expression("x").unwrap();
    assert!(matches!(expr, Expression::Identifier(_)));
}

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
fn test_parse_paren() {
    let expr = parse_expression("(x)").unwrap();
    assert!(matches!(expr, Expression::Identifier(_)));
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
fn test_parse_binary_xor() {
    let expr = parse_expression("a ^ b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::BitXor, .. }));
}

#[test]
fn test_parse_comparison_lt() {
    let expr = parse_expression("a < b").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_comparison_gt() {
    let expr = parse_expression("a > b").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_equality() {
    let expr = parse_expression("a == b").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_inequality() {
    let expr = parse_expression("a != b").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_parse_string_literal_expr() {
    let expr = parse_expression("\"hello world\"").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::String(_))));
}

#[test]
fn test_parse_bool_false() {
    let expr = parse_expression("false").unwrap();
    assert!(matches!(expr, Expression::Literal(Literal::Boolean(false))));
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
fn test_parse_modulo() {
    let expr = parse_expression("10 % 3").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Mod, .. }));
}

#[test]
fn test_parse_chain() {
    let expr = parse_expression("1 + 2 + 3").unwrap();
    assert!(matches!(expr, Expression::Binary { .. }));
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
fn test_parse_not_identifier() {
    let expr = parse_expression("!x").unwrap();
    assert!(matches!(expr, Expression::Unary { op: UnaryOp::Not, .. }));
}

#[test]
fn test_parse_bool_operator() {
    let expr = parse_expression("a == b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Eq, .. }));
}

#[test]
fn test_parse_ne_operator() {
    let expr = parse_expression("a != b").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Neq, .. }));
}

#[test]
fn test_parse_gt_operator() {
    let expr = parse_expression("x > 10").unwrap();
    assert!(matches!(expr, Expression::Binary { op: BinaryOp::Gt, .. }));
}