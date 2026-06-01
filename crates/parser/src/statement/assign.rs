//! Assignment statement parser.
//!
//! Parses `IDENT = <expression> ;` and pushes a [`Statement::Assign`].
//!
//! The caller (see [`super::parse_statements_from_tokens`]) is expected
//! to have already peeked the `IDENT` and the following `=`; it passes
//! the identifier name in so the parser does not re-scan it.
//!
//! The trailing `;` is optional. If the source ends without one, the
//! assignment still goes through; the next top-level statement parser
//! will pick up where this one left off. This matches the lenient
//! style of the rest of the statement parser.

use super::super::expression::ParserError;
use super::super::expression::parse_expression_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse an assignment statement and push the resulting
/// [`Statement::Assign`] onto `statements`.
///
/// The caller must have already verified that the token at `idx - 1` is
/// the identifier (passed in as `name`) and that the token at `idx` is
/// the `=` operator.
///
/// # Errors
///
/// Propagates any error from [`parse_expression_from_tokens`] (the
/// right-hand side). A missing `;` is **not** an error.
pub fn parse_assign_statement(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
    statements: &mut Vec<Statement>,
    name: String,
) -> Result<usize, ParserError> {
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
