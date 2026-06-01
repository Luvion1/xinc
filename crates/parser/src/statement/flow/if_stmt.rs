//! `if`/`else` statement parser.
//!
//! Parses the form:
//!
//! ```text
//! if <expression> { <statements> } [else { <statements> }]?
//! ```
//!
//! The condition is parsed as a full expression. The `then` and `else`
//! branches are recursive statement lists (i.e. any statement kind is
//! allowed inside). The `else` branch is currently restricted to a block;
//! chained `else if` works only because the inner `Block` can contain
//! another `Statement::If` — a future change can lift this if a
//! `Statement::ElseIf` variant is introduced.
//!
//! The caller must have verified that the token at `idx` is the `if`
//! keyword before calling.

use super::super::super::expression::ParserError;
use super::super::super::expression::parse_expression_from_tokens;
use crate::statement::parse_statements_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse an `if` statement and push the resulting [`Statement::If`] onto
/// `statements`.
///
/// # Errors
///
/// - [`ParserError::ExpectedLBrace`] if the `then` body isn't opened with
///   `{`, or if an `else` is present but the `else` body isn't a block.
/// - Any error propagated from [`parse_expression_from_tokens`] (the
///   condition) or [`parse_statements_from_tokens`] (the bodies).
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
