//! Assignment statement parser.

use xin_ast::Statement;
use xin_lexer::TokenKind;
use super::super::expression::parse_expression_from_tokens;
use super::super::expression::ParserError;

/// Parse assignment statement.
pub fn parse_assign_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>, name: String) -> Result<usize, ParserError> {
    idx += 1; // skip identifier (already have name)
    idx += 1; // skip '='
    let (value, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx < tokens.len() && tokens[idx].kind == TokenKind::Semicolon {
        idx += 1;
    }

    statements.push(Statement::Assign { target: name, value });
    Ok(idx)
}