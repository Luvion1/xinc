//! Atomic (non-recursive) expression parsing.
//!
//! This module handles all atomic (leaf or non-recursive) expression
//! forms that do not require operator-precedence logic. Each function
//! here parses a single syntactic category and advances the token index
//! past the consumed range.
//!
//! Covered forms:
//! - Unary operators (`not`, `~`, `-` prefix).
//! - Primary literals (numbers, strings, booleans).
//! - Identifiers and calls.
//! - Parenthesized sub-expressions.

use xin_ast::{Expression, Literal, UnaryOp};
use xin_lexer::{Token, TokenKind};
use crate::expression::ParserError;

/// Parse an atomic expression at the current token position.
///
/// Dispatches based on the leading token kind. Returns the parsed AST
/// node and any error if the token is unexpected in expression context.
pub fn parse_atom(tokens: &[Token], idx: &mut usize) -> Result<Expression, ParserError> {
    if *idx >= tokens.len() { return Err(ParserError::UnexpectedEnd); }
    match &tokens[*idx].kind {
        TokenKind::Not => parse_unary_not(tokens, idx),
        TokenKind::BitNot => parse_unary_bitnot(tokens, idx),
        TokenKind::Minus => parse_unary_neg(tokens, idx),
        TokenKind::Identifier(name) => parse_identifier_expr(tokens, idx, name.clone()),
        TokenKind::Number(n) => Ok(parse_number_expr(tokens, idx, n.clone())),
        TokenKind::String(s) => Ok(parse_string_expr(tokens, idx, s.clone())),
        TokenKind::Keyword(kw) => parse_keyword_expr(tokens, idx, *kw),
        TokenKind::LParen => parse_paren_expr(tokens, idx),
        _ => Err(ParserError::ExpectedExpression),
    }
}

// Unary operators

/// Parse logical NOT unary expression (`not expr`).
fn parse_unary_not(tokens: &[Token], idx: &mut usize) -> Result<Expression, ParserError> {
    *idx += 1; // skip `not` keyword
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::Not, operand: Box::new(operand) })
}

/// Parse bitwise NOT unary expression (`~expr`).
fn parse_unary_bitnot(tokens: &[Token], idx: &mut usize) -> Result<Expression, ParserError> {
    *idx += 1; // skip `~` token
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::BitNot, operand: Box::new(operand) })
}

/// Parse arithmetic negation expression (`-expr`).
fn parse_unary_neg(tokens: &[Token], idx: &mut usize) -> Result<Expression, ParserError> {
    *idx += 1; // skip `-` token
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::Neg, operand: Box::new(operand) })
}

// Primary expressions

/// Parse identifier reference or function call expression.
///
/// If the token after the identifier is `(`, parses a full call with
/// comma-separated arguments. Otherwise returns `Identifier(name)`.
fn parse_identifier_expr(
    tokens: &[Token],
    idx: &mut usize,
    name: String,
) -> Result<Expression, ParserError> {
    *idx += 1; // consume identifier token
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::LParen {
        parse_call_expr(tokens, idx, name)
    } else {
        Ok(Expression::Identifier(name))
    }
}

/// Parse function call expression `name(arg1, arg2, ...)`.
///
/// Expects opening `(` already positioned. Recursively parses arguments
/// via `parse_expression_from_tokens` until closing `)` or EOF.
fn parse_call_expr(
    tokens: &[Token],
    idx: &mut usize,
    name: String,
) -> Result<Expression, ParserError> {
    *idx += 1; // skip `(`
    let mut args = Vec::new();
    // Parse comma-separated arguments until `)` or out of tokens
    while *idx < tokens.len() && tokens[*idx].kind != TokenKind::RParen {
        let (arg, new_idx) = crate::expression::parser::parse_expression_from_tokens(tokens, *idx)?;
        args.push(arg);
        *idx = new_idx;
        if *idx < tokens.len() && tokens[*idx].kind == TokenKind::Comma { *idx += 1; }
    }
    // Consume closing `)` if present
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen { *idx += 1; }
    Ok(Expression::Call { callee: Box::new(Expression::Identifier(name)), args })
}

/// Wrap numeric literal into AST node.
fn parse_number_expr(_tokens: &[Token], idx: &mut usize, n: String) -> Expression {
    *idx += 1;
    Expression::Literal(Literal::Number(n))
}

/// Wrap string literal into AST node.
fn parse_string_expr(_tokens: &[Token], idx: &mut usize, s: String) -> Expression {
    *idx += 1;
    Expression::Literal(Literal::String(s))
}

/// Parse boolean literal (`true` / `false` keyword tokens).
///
/// Other keywords return `ExpectedExpression` so the caller can surface
/// a useful syntax error.
fn parse_keyword_expr(_tokens: &[Token], idx: &mut usize, kw: xin_lexer::Keyword) -> Result<Expression, ParserError> {
    match kw {
        xin_lexer::Keyword::True => { *idx += 1; Ok(Expression::Literal(Literal::Boolean(true))) }
        xin_lexer::Keyword::False => { *idx += 1; Ok(Expression::Literal(Literal::Boolean(false))) }
        _ => Err(ParserError::ExpectedExpression),
    }
}

/// Parse parenthesized expression `(expr)`.
///
/// Delegates to the precedence-climbing `parse_expr` so that operators
/// inside parentheses bind correctly. Consumes the closing `)` if found.
fn parse_paren_expr(tokens: &[Token], idx: &mut usize) -> Result<Expression, ParserError> {
    *idx += 1; // skip `(`
    let expr = crate::expression::parser::parse_expr(tokens, idx, 0)?;
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen { *idx += 1; }
    Ok(expr)
}