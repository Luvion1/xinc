//! `let` statement parser.
//!
//! Parses the form:
//!
//! ```text
//! let <identifier> [: <type>]? = <expression> ;
//! ```
//!
//! The optional `: <type>` is a type annotation; the value is parsed as a
//! full expression (so it can be any arithmetic / call / literal / etc.).
//! The statement pushes a [`Statement::Let`] onto the caller's
//! statement list and returns the new index, just past the trailing `;`.

use super::super::super::expression::{ParserError, parse_expression_from_tokens};
use xin_ast::Statement;
use xin_ast::{BuiltinType, Type};
use xin_lexer::TokenKind;

/// Parse a `let` statement and push the resulting [`Statement::Let`] onto
/// `statements`.
///
/// The caller is expected to have already verified that the token at
/// `idx` is the `let` keyword. The function consumes the keyword,
/// identifier, optional type, `=`, value expression, and trailing `;`.
///
/// # Errors
///
/// - [`ParserError::ExpectedIdentifier`] if the token after `let` is not
///   an identifier.
/// - [`ParserError::InvalidType`] if a `:` is present but the next token
///   is neither a recognized builtin-type keyword nor an identifier.
/// - [`ParserError::ExpectedAssignment`] if the next token isn't `=`.
/// - [`ParserError::ExpectedSemicolon`] if the statement isn't terminated.
/// - Any error propagated from [`parse_expression_from_tokens`].
pub fn parse_let_statement(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
    statements: &mut Vec<Statement>,
) -> Result<usize, ParserError> {
    idx += 1;
    let name = parse_identifier(&tokens[idx])?;
    idx += 1;

    let ty = if idx < tokens.len() && tokens[idx].kind == TokenKind::Colon {
        idx += 1;
        let ty = parse_type(&tokens[idx])?;
        idx += 1;
        Some(ty)
    } else {
        None
    };

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::Assign {
        return Err(ParserError::ExpectedAssignment);
    }
    idx += 1;

    let (value, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::Semicolon {
        return Err(ParserError::ExpectedSemicolon);
    }
    idx += 1;

    statements.push(Statement::Let { name, ty, value });
    Ok(idx)
}

/// Extract the identifier name from a token, or fail.
///
/// Returns [`ParserError::ExpectedIdentifier`] if the token kind is not
/// [`TokenKind::Identifier`]. Public so the `fn` parser can reuse the
/// validation.
pub fn parse_identifier(token: &xin_lexer::Token) -> Result<String, ParserError> {
    match &token.kind {
        TokenKind::Identifier(name) => Ok(name.clone()),
        _ => Err(ParserError::ExpectedIdentifier),
    }
}

/// Parse a type token into a [`Type`].
///
/// Recognized forms:
/// - The builtin type keywords `i32`, `i64`, `f32`, `f64`, `bool` map to
///   [`Type::Builtin`] variants.
/// - Any other identifier maps to [`Type::Named`].
/// - Anything else is a parse error.
///
/// `Str` and other keywords that look like types but aren't currently
/// accepted here will return [`ParserError::InvalidType`]; the keyword
/// set is intentionally narrow.
pub fn parse_type(token: &xin_lexer::Token) -> Result<Type, ParserError> {
    match &token.kind {
        TokenKind::Keyword(kw) => match kw {
            xin_lexer::Keyword::I32 => Ok(Type::Builtin(BuiltinType::I32)),
            xin_lexer::Keyword::I64 => Ok(Type::Builtin(BuiltinType::I64)),
            xin_lexer::Keyword::F32 => Ok(Type::Builtin(BuiltinType::F32)),
            xin_lexer::Keyword::F64 => Ok(Type::Builtin(BuiltinType::F64)),
            xin_lexer::Keyword::Bool => Ok(Type::Builtin(BuiltinType::Bool)),
            _ => Err(ParserError::InvalidType),
        },
        TokenKind::Identifier(name) => Ok(Type::Named(name.clone())),
        _ => Err(ParserError::InvalidType),
    }
}
