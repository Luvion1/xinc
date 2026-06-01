//! f-string token handling for Lexer.

use super::super::Lexer;
use super::parse::fstring::parse_fstring_fragment;
use crate::diagnostics::Position;
use crate::error::LexerError;
use crate::token::{Token, TokenKind};

impl Lexer {
    #[allow(dead_code)]
    pub(super) fn handle_fstring(&mut self, start: Position) -> Result<Option<Token>, LexerError> {
        if self.fstring_brace_nesting > 0 {
            return self.handle_fstring_expr(start);
        }
        self.handle_fstring_literal(start)
    }

    fn handle_fstring_expr(&mut self, start: Position) -> Result<Option<Token>, LexerError> {
        if self.scanner.current_char() == Some('}') {
            self.scanner.advance();
            if self.fstring_brace_nesting == 1 {
                self.fstring_brace_nesting = 0;
                return Ok(None);
            }
            self.fstring_brace_nesting -= 1;
            return Ok(Some(Token::new(TokenKind::RBrace, start.line, start.column)));
        }
        if self.scanner.current_char() == Some('{') {
            self.scanner.advance();
            self.fstring_brace_nesting += 1;
            return Ok(Some(Token::new(TokenKind::LBrace, start.line, start.column)));
        }
        self.parse_normal(start)
    }

    fn handle_fstring_literal(&mut self, start: Position) -> Result<Option<Token>, LexerError> {
        let fragment = parse_fstring_fragment(&mut self.scanner)?;
        if !fragment.is_empty() {
            return Ok(Some(Token::new(TokenKind::String(fragment), start.line, start.column)));
        }
        match self.scanner.current_char() {
            Some('{') => {
                self.scanner.advance();
                self.fstring_brace_nesting = 1;
                Ok(None)
            }
            Some('"') => {
                self.scanner.advance();
                self.fstring_active = false;
                Ok(None)
            }
            _ => {
                if self.scanner.is_at_end() {
                    Err(LexerError::UnterminatedString)
                } else {
                    Ok(None)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub(super) fn handle_fstring_start(
        &mut self,
        start: Position,
    ) -> Result<Option<Token>, LexerError> {
        self.scanner.advance();
        self.scanner.advance();
        self.fstring_active = true;
        let start_pos = start;
        let fragment = parse_fstring_fragment(&mut self.scanner)?;
        if !fragment.is_empty() {
            return Ok(Some(Token::new(
                TokenKind::String(fragment),
                start_pos.line,
                start_pos.column,
            )));
        }
        match self.scanner.current_char() {
            Some('{') => {
                self.scanner.advance();
                self.fstring_brace_nesting = 1;
                Ok(None)
            }
            Some('"') => {
                self.scanner.advance();
                self.fstring_active = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
