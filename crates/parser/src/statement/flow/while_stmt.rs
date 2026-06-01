//! `while` statement parser.
//!
//! Parses the form:
//!
//! ```text
//! while <expression> { <statements> }
//! ```
//!
//! The condition is parsed as a full expression. The body is a recursive
//! statement list, so any statement kind (including nested `while`,
//! blocks, `if`) is valid inside.
//!
//! Note: this parser does not parse `for` loops or `do { } while (...)` —
//! those would belong in a future module.

use super::super::super::expression::ParserError;
use super::super::super::expression::parse_expression_from_tokens;
use crate::statement::parse_statements_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse a `while` statement and push the resulting [`Statement::While`]
/// onto `statements`.
///
/// The caller must have verified that the token at `idx` is the `while`
/// keyword before calling.
///
/// # Errors
///
/// - [`ParserError::ExpectedLBrace`] if the body isn't opened with `{`.
/// - Any error propagated from [`parse_expression_from_tokens`] or
///   [`parse_statements_from_tokens`].
pub fn parse_while_statement(
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

    let (body, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    statements.push(Statement::While { cond, body });
    Ok(idx)
}
