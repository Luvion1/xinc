//! Function statement parser.

use super::super::expression::ParserError;
use super::let_stmt::{parse_identifier, parse_type};
use crate::statement::parse_statements_from_tokens;
use xin_ast::Statement;
use xin_lexer::TokenKind;

/// Parse function declaration.
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
