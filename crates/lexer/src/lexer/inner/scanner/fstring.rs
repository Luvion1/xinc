//! f-string continuation handling for the [`Lexer`].
//!
//! The f-string parser is split into three pieces:
//!
//! - [`super::parse::fstring::parse_fstring_fragment`] — the stateless
//!   fragment reader.
//! - [`super::Lexer::handle_fstring_start`] — the entry point that
//!   consumes the `f` and `"` opener and pushes the lexer into
//!   "active f-string" mode.
//! - The methods in this file — the **active f-string state machine**:
//!   each call to [`Lexer::handle_fstring`] emits one of three token
//!   kinds (literal fragment, identifier placeholder, or close) and
//!   transitions the state.
//!
//! # State
//!
//! The state lives on the [`Lexer`] struct:
//! - `fstring_active: bool` — whether we are inside an f-string.
//! - `fstring_brace_nesting: usize` — depth of `{` braces within an
//!   f-string placeholder; `0` means we are between placeholders.
//!
//! The methods in this file mutate both. Outside of an f-string, the
//! `handle_fstring_*` methods are not called.

use super::super::Lexer;
use super::parse::fstring::parse_fstring_fragment;
use crate::diagnostics::Position;
use crate::error::LexerError;
use crate::token::{Token, TokenKind};

impl Lexer {
    /// Dispatch one step of the active f-string state machine.
    ///
    /// Called from [`super::Lexer::next_token`] when `fstring_active`
    /// is true. The behavior depends on `fstring_brace_nesting`:
    ///
    /// - `> 0` — we are inside a `{ ... }` placeholder; the next
    ///   token is the inner expression, lexed as ordinary source.
    /// - `0` — we are between placeholders; the next token is either
    ///   a literal fragment (until the next `{` or closing `"`) or a
    ///   terminator.
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
