//! While statement parser.

use xin_ast::Statement;
use xin_lexer::TokenKind;
use crate::statement::parse_statements_from_tokens;
use super::super::expression::parse_expression_from_tokens;
use super::super::expression::ParserError;

/// Parse while statement.
pub fn parse_while_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>) -> Result<usize, ParserError> {
    idx += 1;

    let (cond, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LBrace {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1;

    let (body, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    statements.push(Statement::While { cond, body });
    Ok(idx)
}