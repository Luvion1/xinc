//! Token tests.

use super::super::{Keyword, Token, TokenKind};

#[test]
fn test_token_eof() {
    let tok = Token::eof();
    assert_eq!(tok.kind, TokenKind::Eof);
}

#[test]
fn test_token_number() {
    let tok = Token::new(TokenKind::Number("42".to_string()), 1, 1);
    assert!(matches!(tok.kind, TokenKind::Number(_)));
}

#[test]
fn test_token_string() {
    let tok = Token::new(TokenKind::String("hello".to_string()), 1, 1);
    assert!(matches!(tok.kind, TokenKind::String(_)));
}

#[test]
fn test_token_identifier() {
    let tok = Token::new(TokenKind::Identifier("x".to_string()), 1, 1);
    assert!(matches!(tok.kind, TokenKind::Identifier(_)));
}

#[test]
fn test_token_keyword() {
    let tok = Token::new(TokenKind::Keyword(Keyword::Let), 1, 1);
    assert!(matches!(tok.kind, TokenKind::Keyword(_)));
}

#[test]
fn test_token_operators() {
    let ops = [
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Percent,
        TokenKind::BitAnd,
        TokenKind::BitOr,
        TokenKind::BitXor,
        TokenKind::BitNot,
        TokenKind::Shl,
        TokenKind::Shr,
        TokenKind::Not,
    ];
    for op in ops {
        let _ = op;
    }
}

#[test]
fn test_token_punctuation() {
    let punct = [
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBrace,
        TokenKind::RBrace,
        TokenKind::Comma,
        TokenKind::Semicolon,
        TokenKind::Colon,
        TokenKind::Assign,
    ];
    for p in punct {
        let _ = p;
    }
}

#[test]
fn test_keyword_variants() {
    let kws = [
        Keyword::Let,
        Keyword::Fn,
        Keyword::If,
        Keyword::Else,
        Keyword::While,
        Keyword::Return,
        Keyword::True,
        Keyword::False,
        Keyword::Null,
    ];
    for kw in kws {
        let _ = kw;
    }
}
