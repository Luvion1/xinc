//! Lexical analyzer implementation.

pub mod fstring;
pub mod parse;
mod scanner_impl;

pub use scanner_impl::Scanner;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Keyword;

    #[test]
    fn test_let_stmt() {
        let src = "let x = 10;";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() >= 6);
        match tokens[0].kind {
            TokenKind::Keyword(k) => assert_eq!(k, Keyword::Let),
            _ => panic!("Let expected"),
        }
    }

    #[test]
    fn test_string_escape() {
        let src = "\"a\\nb\"";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        if let TokenKind::String(s) = &tokens[0].kind {
            assert_eq!(s, "a\nb");
        } else {
            panic!("String expected");
        }
    }

    #[test]
    fn test_fstring_basic() {
        let src = "f\"hi {x}\"";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 2);
        if let TokenKind::String(s) = &tokens[0].kind {
            assert_eq!(s, "hi ");
        }
        if let TokenKind::Identifier(name) = &tokens[1].kind {
            assert_eq!(name, "x");
        }
    }

    #[test]
    fn test_raw_string() {
        let src = "r\"no\\nescapes\"";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        if let TokenKind::String(s) = &tokens[0].kind {
            assert_eq!(s, "no\\nescapes");
        }
    }

    #[test]
    fn test_raw_char() {
        let src = "r'x'";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        if let TokenKind::Char(c) = &tokens[0].kind {
            assert_eq!(*c, 'x');
        }
    }
}

use crate::diagnostics::Position;
use crate::error::LexerError;
use crate::token::identifier::is_identifier_start;
use crate::token::keyword::from_str;
use crate::token::{Token, TokenKind};
use parse::{
    ident::parse_identifier,
    number::parse_number,
    operator::parse_operator_token,
    punctuation::parse_punctuation_token,
    string::{parse_char_normal, parse_char_raw, parse_raw_string, parse_string_normal},
};
/// The lexer struct.
#[derive(Debug)]
pub struct Lexer {
    scanner: Scanner,
    fstring_active: bool,
    fstring_brace_nesting: usize,
}

impl Lexer {
    /// Create a new lexer.
    pub fn new(source: &str) -> Self {
        Self { scanner: Scanner::new(source), fstring_active: false, fstring_brace_nesting: 0 }
    }

    /// Tokenize the source into a vector of tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            if !(self.fstring_active && self.fstring_brace_nesting == 0) {
                self.scanner.skip_whitespace_and_comments()?;
            }
            if self.scanner.is_at_end() {
                if self.fstring_active {
                    return Err(LexerError::UnterminatedString);
                }
                break;
            }
            if let Some(tok) = self.next_token()? {
                tokens.push(tok);
            }
        }
        tokens.push(Token::eof());
        Ok(tokens)
    }

    /// Get the next token (excluding EOF).
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        let start = self.scanner.position();

        if self.fstring_active {
            return self.handle_fstring(start);
        }

        if self.scanner.current_char() == Some('f') {
            if self.scanner.peek() == Some('"') {
                return self.handle_fstring_start(start);
            }
            self.parse_normal(start)
        } else if self.scanner.current_char() == Some('r') {
            if self.scanner.peek() == Some('"') {
                self.scanner.advance();
                self.scanner.advance();
                let raw = parse_raw_string(&mut self.scanner)?;
                return Ok(Some(Token::new(TokenKind::String(raw), start.line, start.column)));
            }
            if self.scanner.peek() == Some('\'') {
                self.scanner.advance();
                self.scanner.advance();
                let raw_char = parse_char_raw(&mut self.scanner)?;
                return Ok(Some(Token::new(TokenKind::Char(raw_char), start.line, start.column)));
            }
            self.parse_normal(start)
        } else if self.scanner.current_char() == Some('"') {
            self.scanner.advance();
            let s = parse_string_normal(&mut self.scanner)?;
            Ok(Some(Token::new(TokenKind::String(s), start.line, start.column)))
        } else if self.scanner.current_char() == Some('\'') {
            self.scanner.advance();
            let ch = parse_char_normal(&mut self.scanner)?;
            Ok(Some(Token::new(TokenKind::Char(ch), start.line, start.column)))
        } else {
            self.parse_normal(start)
        }
    }

    /// Dispatch normal token parsing.
    fn parse_normal(&mut self, start: Position) -> Result<Option<Token>, LexerError> {
        if let Some(c) = self.scanner.current_char()
            && is_identifier_start(c)
        {
            let name = parse_identifier(&mut self.scanner)?;
            if let Some(kw) = from_str(&name) {
                return Ok(Some(Token::new(TokenKind::Keyword(kw), start.line, start.column)));
            }
            return Ok(Some(Token::new(TokenKind::Identifier(name), start.line, start.column)));
        }

        if let Some(c) = self.scanner.current_char()
            && c.is_ascii_digit()
        {
            let num = parse_number(&mut self.scanner)?;
            return Ok(Some(Token::new(TokenKind::Number(num), start.line, start.column)));
        }

        if let Some(tok) = parse_operator_token(&mut self.scanner) {
            return Ok(Some(Token::new(tok, start.line, start.column)));
        }
        if let Some(tok) = parse_punctuation_token(&mut self.scanner) {
            return Ok(Some(Token::new(tok, start.line, start.column)));
        }

        let ch = self.scanner.current_char().unwrap_or('\0');
        let pos = self.scanner.byte_offset();
        self.scanner.advance();
        Err(LexerError::InvalidChar { character: ch, position: pos })
    }
}
