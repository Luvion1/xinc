//! Expression parsing.
//!
//! Parses expressions from token streams.

use xin_ast::{Expression, Literal, BinaryOp, UnaryOp};
use xin_lexer::{tokenize, TokenKind, LexerError};

/// Parse an expression from source.
pub fn parse_expression(source: &str) -> Result<Expression, ParserError> {
    let tokens = tokenize(source)?;
    if tokens.is_empty() {
        return Err(ParserError::EmptyInput);
    }

    parse_expr(&tokens, &mut 0)
}

/// Parse expression from pre-tokenized tokens.
pub fn parse_expression_from_tokens(tokens: &[xin_lexer::Token], mut idx: usize) -> Result<(Expression, usize), ParserError> {
    let result = parse_expr(tokens, &mut idx)?;
    Ok((result, idx))
}

/// Recursive expression parsing with operator precedence.
fn parse_expr(tokens: &[xin_lexer::Token], idx: &mut usize) -> Result<Expression, ParserError> {
    let left = parse_atom(tokens, idx)?;
    
    // Handle binary operators
    #[allow(clippy::collapsible_if)]
    if *idx < tokens.len() {
        if let Some(op) = match_operator(&tokens[*idx].kind) {
            *idx += 1;
            let right = parse_atom(tokens, idx)?;
            return Ok(Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
    }
    
    Ok(left)
}

/// Parse atomic expressions (literals, identifiers, parenthesized).
fn parse_atom(tokens: &[xin_lexer::Token], idx: &mut usize) -> Result<Expression, ParserError> {
    if *idx >= tokens.len() {
        return Err(ParserError::UnexpectedEnd);
    }

    match &tokens[*idx].kind {
        TokenKind::Not => {
            *idx += 1;
            let operand = parse_atom(tokens, idx)?;
            Ok(Expression::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            })
        }
        TokenKind::Minus => {
            *idx += 1;
            let operand = parse_atom(tokens, idx)?;
            Ok(Expression::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            })
        }
        TokenKind::Identifier(name) => {
            let name = name.clone();
            *idx += 1;
            // Check for function call
            if *idx < tokens.len() && tokens[*idx].kind == TokenKind::LParen {
                *idx += 1; // skip '('
                let mut args = Vec::new();
                while *idx < tokens.len() && tokens[*idx].kind != TokenKind::RParen {
                    let (arg, new_idx) = parse_expression_from_tokens(tokens, *idx)?;
                    args.push(arg);
                    *idx = new_idx;
                    // Skip comma if present
                    if *idx < tokens.len() && tokens[*idx].kind == TokenKind::Comma {
                        *idx += 1;
                    }
                }
                if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen {
                    *idx += 1; // skip ')'
                }
                Ok(Expression::Call {
                    callee: Box::new(Expression::Identifier(name)),
                    args,
                })
            } else {
                Ok(Expression::Identifier(name))
            }
        }
        TokenKind::Number(n) => {
            *idx += 1;
            Ok(Expression::Literal(Literal::Number(n.clone())))
        }
        TokenKind::String(s) => {
            *idx += 1;
            Ok(Expression::Literal(Literal::String(s.clone())))
        }
        TokenKind::Keyword(kw) => match kw {
            xin_lexer::Keyword::True => {
                *idx += 1;
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            xin_lexer::Keyword::False => {
                *idx += 1;
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            _ => Err(ParserError::ExpectedExpression),
        },
        TokenKind::LParen => {
            *idx += 1; // skip '('
            let expr = parse_expr(tokens, idx)?;
            if *idx < tokens.len() && tokens[*idx].kind == TokenKind::RParen {
                *idx += 1; // skip ')'
            }
            Ok(expr)
        }
        _ => Err(ParserError::ExpectedExpression),
    }
}

/// Match token to binary operator.
pub fn match_operator(kind: &TokenKind) -> Option<BinaryOp> {
    match kind {
        TokenKind::Plus => Some(BinaryOp::Add),
        TokenKind::Minus => Some(BinaryOp::Sub),
        TokenKind::Star => Some(BinaryOp::Mul),
        TokenKind::Slash => Some(BinaryOp::Div),
        TokenKind::Eq => Some(BinaryOp::Eq),
        TokenKind::Neq => Some(BinaryOp::Neq),
        TokenKind::Lt => Some(BinaryOp::Lt),
        TokenKind::Gt => Some(BinaryOp::Gt),
        _ => None,
    }
}

/// Parser error types.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    /// Lexer error.
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),

    /// Empty input.
    #[error("Empty input")]
    EmptyInput,

    /// Expected expression.
    #[error("Expected expression")]
    ExpectedExpression,

    /// Unexpected end of input.
    #[error("Unexpected end of input")]
    UnexpectedEnd,

    /// Expected identifier.
    #[error("Expected identifier")]
    ExpectedIdentifier,

    /// Expected assignment operator.
    #[error("Expected '='")]
    ExpectedAssignment,

    /// Expected semicolon.
    #[error("Expected ';'")]
    ExpectedSemicolon,

    /// Invalid type.
    #[error("Invalid type")]
    InvalidType,

    /// Expected left brace.
    #[error("Expected '{{'")]
    ExpectedLBrace,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}