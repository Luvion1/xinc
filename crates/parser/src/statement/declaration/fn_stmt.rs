//! `fn` declaration parser.
//!
//! Parses the form:
//!
//! ```text
//! fn <identifier> ( [param [, param]*]? ) [-> <type>]? { <statements> }
//! ```
//!
//! The body is parsed by recursively calling
//! [`parse_statements_from_tokens`], so any statement kind (including
//! nested blocks, `if`, `while`) is valid inside a function body. The
//! caller is expected to have verified that the token at `idx` is the
//! `fn` keyword before invoking this function.
//!
//! # Error recovery
//!
//! - If the parameter list is unterminated, the parser exits the loop at
//!   the end of the token stream and proceeds to look for `->` and `{`.
//!   This is forgiving: trailing junk is silently ignored.
//! - If the body has no closing `}`, [`parse_statements_from_tokens`]
//!   will stop at `Eof` and this function will not increment past it.

use super::super::super::expression::ParserError;
use super::let_stmt::{parse_identifier, parse_type};
use crate::statement::parse_statements_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse a `fn` declaration and push the resulting [`Statement::Fn`] onto
/// `statements`.
///
/// # Errors
///
/// - [`ParserError::ExpectedIdentifier`] if the name token is not an
///   identifier.
/// - [`ParserError::ExpectedLBrace`] if the parameter list is not opened
///   with `(` or if the body is not opened with `{`. (The variant name
///   is historical; both `(` and `{` surface through it.)
/// - [`ParserError::InvalidType`] from the type parser.
/// - Any error propagated from [`parse_statements_from_tokens`].
pub fn parse_fn_statement(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
    statements: &mut Vec<Statement>,
) -> Result<usize, ParserError> {
    idx += 1;
    let name = parse_identifier(&tokens[idx])?;
    idx += 1;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LParen {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1;

    let mut params = Vec::new();
    while idx < tokens.len() && tokens[idx].kind != TokenKind::RParen {
        let param_name = parse_identifier(&tokens[idx])?;
        idx += 1;

        let param_ty = if idx < tokens.len() && tokens[idx].kind == TokenKind::Colon {
            idx += 1;
            Some(parse_type(&tokens[idx])?)
        } else {
            None
        };
        idx += 1;

        params.push(xin_ast::Param { name: param_name, ty: param_ty });

        if idx < tokens.len() && tokens[idx].kind == TokenKind::Comma {
            idx += 1;
        }
    }

    if idx < tokens.len() && tokens[idx].kind == TokenKind::RParen {
        idx += 1;
    }

    let ret_ty = if idx < tokens.len() && tokens[idx].kind == TokenKind::Arrow {
        idx += 1;
        let ty = parse_type(&tokens[idx])?;
        idx += 1;
        Some(ty)
    } else {
        None
    };

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LBrace {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1;

    let (body, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    statements.push(Statement::Fn { name, params, body, ret_ty });
    Ok(idx)
}
