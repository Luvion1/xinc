//! Expression parsing with precedence climbing.
//!
//! Entry points: `parse_expression` (str → AST) and
//! `parse_expression_from_tokens` (token slice → AST). Delegates to
//! `atom::parse_atom` for atomic (non-recursive) forms.

use xin_ast::{Expression, BinaryOp};
use xin_lexer::{TokenKind, tokenize};
use super::error::ParserError;

use super::atom::parse_atom;

/// Parse an expression from source string.
pub fn parse_expression(source: &str) -> Result<Expression, ParserError> {
    let tokens = tokenize(source)?;
    if tokens.is_empty() { return Err(ParserError::EmptyInput); }
    let mut idx = 0;
    parse_expr(&tokens, &mut idx, 0)
}

/// Parse expression from pre-tokenized tokens.
pub fn parse_expression_from_tokens(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
) -> Result<(Expression, usize), ParserError> {
    let result = parse_expr(tokens, &mut idx, 0)?;
    Ok((result, idx))
}

/// Precedence-climbing expression parser (Pratt-style).
///
/// Parses left-associative binary operators with correct precedence:
/// higher-precedence operators (e.g. `*`) bind tighter than lower ones (e.g. `+`).
/// The `min_prec` parameter controls the minimum precedence level to accept,
/// enabling recursive descent through precedence tiers.
///
/// # Precedence levels
/// | Level | Operators |
/// |-------|-----------|
/// | 3     | `\|\|`    |
/// | 4     | `&&`      |
/// | 5     | `\|`      |
/// | 6     | `^`       |
/// | 7     | `&`       |
/// | 8     | `==` `!=` |
/// | 9     | `<` `>`   |
/// | 11    | `<<` `>>` |
/// | 12    | `+` `-`   |
/// | 13    | `*` `/` `%` |
pub fn parse_expr(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
    min_prec: u8,
) -> Result<Expression, ParserError> {
    let mut left = parse_atom(tokens, idx)?;

    while *idx < tokens.len() {
        let Some(op) = match_operator(&tokens[*idx].kind) else { break };
        let prec = binary_precedence(op);
        if prec < min_prec { break; }
        *idx += 1;
        let right = parse_expr(tokens, idx, prec + 1)?;
        left = Expression::Binary { left: Box::new(left), op, right: Box::new(right) };
    }

    Ok(left)
}

/// Precedence level for each binary operator (higher = tighter binding).
fn binary_precedence(op: BinaryOp) -> u8 {
    match op {
        BinaryOp::Or => 3,
        BinaryOp::And => 4,
        BinaryOp::BitOr => 5,
        BinaryOp::BitXor => 6,
        BinaryOp::BitAnd => 7,
        BinaryOp::Eq | BinaryOp::Neq => 8,
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 9,
        BinaryOp::Shl | BinaryOp::Shr => 11,
        BinaryOp::Add | BinaryOp::Sub => 12,
        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 13,
    }
}

/// Match token kind to binary operator.
pub fn match_operator(kind: &TokenKind) -> Option<BinaryOp> {
    match kind {
        TokenKind::Plus => Some(BinaryOp::Add),
        TokenKind::Minus => Some(BinaryOp::Sub),
        TokenKind::Star => Some(BinaryOp::Mul),
        TokenKind::Slash => Some(BinaryOp::Div),
        TokenKind::Percent => Some(BinaryOp::Mod),
        TokenKind::Eq => Some(BinaryOp::Eq),
        TokenKind::Neq => Some(BinaryOp::Neq),
        TokenKind::Lt => Some(BinaryOp::Lt),
        TokenKind::Lte => Some(BinaryOp::Le),
        TokenKind::Gt => Some(BinaryOp::Gt),
        TokenKind::Gte => Some(BinaryOp::Ge),
        TokenKind::Shl => Some(BinaryOp::Shl),
        TokenKind::Shr => Some(BinaryOp::Shr),
        TokenKind::BitAnd => Some(BinaryOp::BitAnd),
        TokenKind::BitOr => Some(BinaryOp::BitOr),
        TokenKind::BitXor => Some(BinaryOp::BitXor),
        TokenKind::And => Some(BinaryOp::And),
        TokenKind::Or => Some(BinaryOp::Or),
        _ => None,
    }
}