//! Statement parsing.
//!
//! Parses statement sequences from a token stream into the AST
//! [`Statement`] variants. The entry point [`parse_statement`] takes a
//! source string; [`parse_statements_from_tokens`] takes a pre-tokenized
//! slice and a starting index.
//!
//! # Submodules
//!
//! - [`assign`] — `x = expr;` assignments to existing bindings.
//! - [`declaration`] — `let` bindings and `fn` declarations.
//! - [`flow`] — `if` / `while` control flow.
//!
//! # Block handling
//!
//! A `{` token starts a [`Statement::Block`]. The block parser is
//! recursive: nested `{ ... }` push additional `Block` nodes onto the
//! statement list. A matching `}` ends the current block. A bare `}` at
//! the top level (e.g. after a finished block) stops the loop without
//! erroring — useful for the recursive call from
//! [`declaration::fn_stmt::parse_fn_statement`] and the flow parsers.
//!
//! # Unknown statements
//!
//! Anything that is not a known keyword, identifier-assignment, or
//! `LBrace`/`RBrace` causes the parser to skip the token and advance
//! the index. This is intentionally lenient: a stray semicolon or
//! unrecognized keyword in a body is silently passed over. The semantic
//! analyzer is the source of truth for rejecting truly malformed input.

pub mod assign;
pub mod declaration;
pub mod flow;
#[cfg(test)]
mod tests;

use super::expression::ParserError;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse a list of statements from a source string.
///
/// Convenience wrapper that tokenizes the input and calls
/// [`parse_statements_from_tokens`]. The returned index is discarded.
///
/// # Errors
///
/// Returns [`ParserError::Lexer`] if tokenization fails, or any variant
/// surfaced by the per-statement parsers.
pub fn parse_statement(source: &str) -> Result<Vec<Statement>, ParserError> {
    let tokens = xin_lexer::tokenize(source)?;
    let (stmts, _) = parse_statements_from_tokens(&tokens, 0)?;
    Ok(stmts)
}

/// Parse statements from a pre-tokenized slice, returning the new index.
///
/// The parser loops until it sees [`TokenKind::Eof`] or a closing
/// `RBrace` that wasn't consumed by the current statement. Each
/// statement kind is dispatched to its submodule:
///
/// - `let` → [`declaration::let_stmt`]
/// - `fn`  → [`declaration::fn_stmt`]
/// - `if`  → [`flow::if_stmt`]
/// - `while` → [`flow::while_stmt`]
/// - `IDENT =` → [`assign`]
/// - `{` → push a [`Statement::Block`] and recurse
///
/// The returned index is the position after the last consumed token
/// (or the position of the first unconsumed token if the parser stopped
/// at `}`/`Eof`).
pub fn parse_statements_from_tokens(
    tokens: &[xin_lexer::Token],
    mut idx: usize,
) -> Result<(Vec<Statement>, usize), ParserError> {
    let mut statements = Vec::new();

    while idx < tokens.len() {
        if tokens[idx].kind == TokenKind::Eof {
            break;
        }

        if tokens[idx].kind == TokenKind::LBrace {
            idx += 1;
            let (block_stmts, new_idx) = parse_statements_from_tokens(tokens, idx)?;
            statements.push(Statement::Block(block_stmts));
            idx = new_idx;
            if idx < tokens.len() && tokens[idx].kind == TokenKind::RBrace {
                idx += 1;
            }
            continue;
        }

        if tokens[idx].kind == TokenKind::RBrace {
            break;
        }

        if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Let) {
            idx = declaration::let_stmt::parse_let_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Fn) {
            idx = declaration::fn_stmt::parse_fn_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::If) {
            idx = flow::if_stmt::parse_if_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::While) {
            idx = flow::while_stmt::parse_while_statement(tokens, idx, &mut statements)?;
        } else if let TokenKind::Identifier(name) = &tokens[idx].kind {
            let name = name.clone();
            let next_idx = idx + 1;
            if next_idx < tokens.len() && tokens[next_idx].kind == TokenKind::Assign {
                idx = assign::parse_assign_statement(tokens, idx, &mut statements, name)?;
                continue;
            }
        }
        idx += 1;
    }

    Ok((statements, idx))
}
