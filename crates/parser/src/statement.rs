//! Statement parsing.
//!
//! Parses statements from tokens, starting with let statements.

use xin_ast::{Statement, Type, BuiltinType};
use xin_lexer::{tokenize, TokenKind};
use super::expression::{parse_expression_from_tokens, ParserError};

/// Parse a statement from source.
pub fn parse_statement(source: &str) -> Result<Vec<Statement>, ParserError> {
    let tokens = tokenize(source)?;
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

        // Parse let statement
        if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Let) {
            idx = parse_let_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::If) {
            idx = parse_if_statement(tokens, idx, &mut statements)?;
        } else if tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::While) {
            idx = parse_while_statement(tokens, idx, &mut statements)?;
        } else {
            // Check for assignment: identifier followed by =
            if let TokenKind::Identifier(name) = &tokens[idx].kind {
                let name = name.clone();
                let next_idx = idx + 1;
                if next_idx < tokens.len() && tokens[next_idx].kind == TokenKind::Assign {
                    idx = parse_assign_statement(tokens, idx, &mut statements, name)?;
                    continue;
                }
            }
            idx += 1;
        }
    }

    Ok((statements, idx))
}

/// Parse let statement.
fn parse_let_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>) -> Result<usize, ParserError> {
    idx += 1; // skip 'let'
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

/// Parse if-else statement.
fn parse_if_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>) -> Result<usize, ParserError> {
    idx += 1; // skip 'if'

    let (cond, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LBrace {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1; // skip '{'

    let (then_stmts, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    let r_else = if idx < tokens.len() && tokens[idx].kind == TokenKind::Keyword(xin_lexer::Keyword::Else) {
        idx += 1; // skip 'else'
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

/// Parse while statement.
fn parse_while_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>) -> Result<usize, ParserError> {
    idx += 1; // skip 'while'

    let (cond, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx >= tokens.len() || tokens[idx].kind != TokenKind::LBrace {
        return Err(ParserError::ExpectedLBrace);
    }
    idx += 1; // skip '{'

    let (body, mut new_idx) = parse_statements_from_tokens(tokens, idx)?;
    if new_idx < tokens.len() && tokens[new_idx].kind == TokenKind::RBrace {
        new_idx += 1;
    }
    idx = new_idx;

    statements.push(Statement::While { cond, body });
    Ok(idx)
}

/// Parse assignment statement.
fn parse_assign_statement(tokens: &[xin_lexer::Token], mut idx: usize, statements: &mut Vec<Statement>, name: String) -> Result<usize, ParserError> {
    idx += 1; // skip 'x' (already have name)
    idx += 1; // skip '='
    let (value, new_idx) = parse_expression_from_tokens(tokens, idx)?;
    idx = new_idx;

    if idx < tokens.len() && tokens[idx].kind == TokenKind::Semicolon {
        idx += 1;
    }

    statements.push(Statement::Assign { target: name, value });
    Ok(idx)
}

/// Parse identifier from token.
fn parse_identifier(token: &xin_lexer::Token) -> Result<String, ParserError> {
    match &token.kind {
        TokenKind::Identifier(name) => Ok(name.clone()),
        _ => Err(ParserError::ExpectedIdentifier),
    }
}

/// Parse type from token.
fn parse_type(token: &xin_lexer::Token) -> Result<Type, ParserError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let() {
        let stmts = parse_statement("let x = 10;").unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_let_with_type() {
        let stmts = parse_statement("let x: i32 = 10;").unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_block() {
        let stmts = parse_statement("{ let x = 1; }").unwrap();
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Statement::Block(_)));
    }

    #[test]
    fn test_parse_if() {
        let stmts = parse_statement("if x { let y = 1; }").unwrap();
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Statement::If { .. }));
    }

    #[test]
    fn test_parse_if_else() {
        let stmts = parse_statement("if x { let y = 1; } else { let z = 2; }").unwrap();
        assert_eq!(stmts.len(), 1);
        if let Statement::If { r#else: Some(_), .. } = &stmts[0] {
            // test passed
        } else {
            panic!("Expected if-else statement with else branch");
        }
    }

    #[test]
    fn test_parse_while() {
        let stmts = parse_statement("while x { let y = 1; }").unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_while_with_expr() {
        let stmts = parse_statement("while true { x = 1; }").unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_assign() {
        let stmts = parse_statement("x = 10;").unwrap();
        assert_eq!(stmts.len(), 1);
        if let Statement::Assign { target, .. } = &stmts[0] {
            assert_eq!(target, "x");
        } else {
            panic!("Expected Assign statement");
        }
    }
}