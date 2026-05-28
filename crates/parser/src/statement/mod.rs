//! Statement parsing.
//!
//! Parses statements from tokens into AST nodes.

pub mod let_stmt;
pub mod if_stmt;
pub mod while_stmt;
pub mod fn_stmt;
pub mod assign;
#[cfg(test)]
mod tests;

use xin_ast::Statement;
use xin_lexer::TokenKind;
use super::expression::{parse_expression_from_tokens, ParserError};

/// Parse a statement from source.
pub fn parse_statement(source: &str) -> Result<Vec<Statement>, ParserError> {
    let tokens = xin_lexer::tokenize(source)?;
    let (stmts, _) = parse_statements_from_tokens(&tokens, 0)?;
    Ok(stmts)
}

/// Parse statements from tokens, returning new idx.
pub fn parse_statements_from_tokens(tokens: &[xin_lexer::Token], mut idx: usize) -> Result<(Vec<Statement>, usize), ParserError> {
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
            idx = let_stmt::parse_let_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Fn) {
            idx = fn_stmt::parse_fn_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::If) {
            idx = if_stmt::parse_if_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::While) {
            idx = while_stmt::parse_while_statement(tokens, idx, &mut statements)?;
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