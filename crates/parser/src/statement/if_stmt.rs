//! If statement parser.

use super::super::expression::ParserError;
use super::super::expression::parse_expression_from_tokens;
use crate::statement::parse_statements_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse if-else statement.
pub fn parse_if_statement(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
    statements: &mut Vec<Statement>,
) -> Result<usize, ParserError> {
    idx += 1;

    let (cond, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LBrace {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1;

    let (then_stmts, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    let r_else =
        if idx < tokens.len() && tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Else) {
            idx += 1;
            if idx < tokens.len() && tokens[idx].kind == TokenKind::LBrace {
                idx += 1;
                let (else_stmts, else_idx) = parse_statements_from_tokens(tokens, idx)?;
                idx = else_idx;
                if idx < tokens.len() && tokens[idx].kind == TokenKind::RBrace {
                    idx += 1;
                }
                Some(Box::new(Statement::Block(else_stmts)))
            } else {
                return Err(ParserError::ExpectedLBrace);
            }
        } else {
            None
        };

    statements.push(Statement::If { cond, then: then_stmts, r#else: r_else });
    Ok(idx)
}
