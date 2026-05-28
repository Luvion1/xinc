//! Let statement parser.

use xin_ast::Statement;
use xin_lexer::TokenKind;
use super::super::expression::{parse_expression_from_tokens, ParserError};
use xin_ast::{Type, BuiltinType};

/// Parse let statement.
pub fn parse_let_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>) -> Result<usize, ParserError> {
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

/// Parse identifier from token.
pub fn parse_identifier(token: &xin_lexer::Token) -> Result<String, ParserError> {
    match &token.kind {
        TokenKind::Identifier(name) => Ok(name.clone()),
        _ => Err(ParserError::ExpectedIdentifier),
    }
}

/// Parse type from token.
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