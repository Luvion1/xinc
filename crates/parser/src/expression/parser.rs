//! Expression parser implementation.

use xin_ast::{BinaryOp, Expression, Literal, UnaryOp};
use xin_lexer::{LexerError, TokenKind, tokenize};

/// Parse an expression from source.
pub fn parse_expression(source: &str) -> Result<Expression, ParserError> {
    let tokens = tokenize(source)?;
    if tokens.is_empty() {
        return Err(ParserError::EmptyInput);
    }

    parse_expr(&tokens, &mut 0)
}

/// Parse expression from pre-tokenized tokens.
pub fn parse_expression_from_tokens(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
) -> Result<(Expression, usize), ParserError> {
    let result = parse_expr(tokens, &mut idx)?;
    Ok((result, idx))
}

/// Recursive expression parsing with operator precedence.
pub fn parse_expr(tokens: &[xin_lexer::Token], idx: &mut usize) -> Result<Expression, ParserError> {
    let left = parse_atom(tokens, idx)?;

    #[allow(clippy::collapsible_if)]
    if *idx < tokens.len() {
        if let Some(op) = match_operator(&tokens[*idx].kind) {
            *idx += 1;
            let right = parse_atom(tokens, idx)?;
            return Ok(Expression::Binary { left: Box::new(left), op, right: Box::new(right) });
        }
    }

    Ok(left)
}

/// Parse atomic expressions.
pub fn parse_atom(tokens: &[xin_lexer::Token], idx: &mut usize) -> Result<Expression, ParserError> {
    if *idx >= tokens.len() {
        return Err(ParserError::UnexpectedEnd);
    }

    match &tokens[*idx].kind {
        TokenKind::Not => parse_unary_not(tokens, idx),
        TokenKind::BitNot => parse_unary_bitnot(tokens, idx),
        TokenKind::Minus => parse_unary_neg(tokens, idx),
        TokenKind::Identifier(name) => parse_identifier_expr(tokens, idx, name.clone()),
        TokenKind::Number(n) => parse_number_expr(tokens, idx, n.clone()),
        TokenKind::String(s) => parse_string_expr(tokens, idx, s.clone()),
        TokenKind::Keyword(kw) => parse_keyword_expr(tokens, idx, kw),
        TokenKind::LParen => parse_paren_expr(tokens, idx),
        _ => Err(ParserError::ExpectedExpression),
    }
}

fn parse_unary_not(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
) -> Result<Expression, ParserError> {
    *idx += 1;
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::Not, operand: Box::new(operand) })
}

fn parse_unary_bitnot(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
) -> Result<Expression, ParserError> {
    *idx += 1;
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::BitNot, operand: Box::new(operand) })
}

fn parse_unary_neg(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
) -> Result<Expression, ParserError> {
    *idx += 1;
    let operand = parse_atom(tokens, idx)?;
    Ok(Expression::Unary { op: UnaryOp::Neg, operand: Box::new(operand) })
}

fn parse_identifier_expr(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
    name: String,
) -> Result<Expression, ParserError> {
    *idx += 1;
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::LParen {
        parse_call_expr(tokens, idx, name)
    } else {
        Ok(Expression::Identifier(name))
    }
}

fn parse_call_expr(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
    name: String,
) -> Result<Expression, ParserError> {
    *idx += 1;
    let mut args = Vec::new();
    while *idx < tokens.len() && tokens[*idx].kind != TokenKind::RParen {
        let (arg, new_idx) = parse_expression_from_tokens(tokens, *idx)?;
        args.push(arg);
        *idx = new_idx;
        if *idx < tokens.len() && tokens[*idx].kind == TokenKind::Comma {
            *idx += 1;
        }
    }
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen {
        *idx += 1;
    }
    Ok(Expression::Call { callee: Box::new(Expression::Identifier(name)), args })
}

fn parse_number_expr(
    _tokens: &[xin_lexer::Token],
    idx: &mut usize,
    n: String,
) -> Result<Expression, ParserError> {
    *idx += 1;
    Ok(Expression::Literal(Literal::Number(n)))
}

fn parse_string_expr(
    _tokens: &[xin_lexer::Token],
    idx: &mut usize,
    s: String,
) -> Result<Expression, ParserError> {
    *idx += 1;
    Ok(Expression::Literal(Literal::String(s)))
}

fn parse_keyword_expr(
    _tokens: &[xin_lexer::Token],
    idx: &mut usize,
    kw: &xin_lexer::Keyword,
) -> Result<Expression, ParserError> {
    match kw {
        xin_lexer::Keyword::True => {
            *idx += 1;
            Ok(Expression::Literal(Literal::Boolean(true)))
        }
        xin_lexer::Keyword::False => {
            *idx += 1;
            Ok(Expression::Literal(Literal::Boolean(false)))
        }
        _ => Err(ParserError::ExpectedExpression),
    }
}

fn parse_paren_expr(
    tokens: &[xin_lexer::Token],
    idx: &mut usize,
) -> Result<Expression, ParserError> {
    *idx += 1;
    let expr = parse_expr(tokens, idx)?;
    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen {
        *idx += 1;
    }
    Ok(expr)
}

/// Match token to binary operator.
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
        TokenKind::Gt => Some(BinaryOp::Gt),
        TokenKind::Shl => Some(BinaryOp::Shl),
        TokenKind::Shr => Some(BinaryOp::Shr),
        TokenKind::And => Some(BinaryOp::BitAnd),
        TokenKind::Or => Some(BinaryOp::BitOr),
        TokenKind::BitXor => Some(BinaryOp::BitXor),
        TokenKind::BitAnd => Some(BinaryOp::BitAnd),
        TokenKind::BitOr => Some(BinaryOp::BitOr),
        _ => None,
    }
}

/// Parser error types.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),

    #[error("Empty input")]
    EmptyInput,

    #[error("Expected expression")]
    ExpectedExpression,

    #[error("Unexpected end of input")]
    UnexpectedEnd,

    #[error("Expected identifier")]
    ExpectedIdentifier,

    #[error("Expected '='")]
    ExpectedAssignment,

    #[error("Expected ';'")]
    ExpectedSemicolon,

    #[error("Invalid type")]
    InvalidType,

    #[error("Expected '{{'")]
    ExpectedLBrace,
}
