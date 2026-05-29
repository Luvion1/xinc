//! Lexical analyzer implementation.
//!
//! The `Lexer` is the main tokenization engine. It uses a `Scanner` for
//! low-level character input and dispatches to specialized parsers for each
//! token category. Supports f-string interpolation.

// Declare submodules (files in this directory)
pub mod parse;
pub mod scanner;

// Bring submodules into scope for convenience
use crate::diagnostics::Position;
use crate::error::LexerError;
use crate::token::identifier::is_identifier_start;
use crate::token::keyword::from_str;
use crate::token::{Token, TokenKind};
use parse::{
    fstring::parse_fstring_fragment,
    ident::parse_identifier,
    number::parse_number,
    string::{parse_char_normal, parse_char_raw, parse_raw_string, parse_string_normal},
    symbols::{parse_operator_token, parse_punctuation_token},
};
use scanner::Scanner;

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
    ///
    /// # Errors
    /// Returns `LexerError` if lexical analysis fails (e.g., invalid character, unterminated string or comment, invalid number, etc.).
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            // Skip whitespace/comments except during f-string literal part
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

        // f-string mode handling
        if self.fstring_active {
            return if self.fstring_brace_nesting > 0 {
                // Inside expression: handle braces specially
                if let Some('}') = self.scanner.current_char() {
                    self.scanner.advance();
                    if self.fstring_brace_nesting == 1 {
                        self.fstring_brace_nesting = 0;
                        return Ok(None);
                    } else {
                        self.fstring_brace_nesting -= 1;
                        return Ok(Some(Token::new(TokenKind::RBrace, start.line, start.column)));
                    }
                }
                if let Some('{') = self.scanner.current_char() {
                    self.scanner.advance();
                    self.fstring_brace_nesting += 1;
                    return Ok(Some(Token::new(TokenKind::LBrace, start.line, start.column)));
                }
                self.parse_normal(start)
            } else {
                // String literal part
                let fragment = parse_fstring_fragment(&mut self.scanner)?;
                if !fragment.is_empty() {
                    return Ok(Some(Token::new(
                        TokenKind::String(fragment),
                        start.line,
                        start.column,
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
                    _ => {
                        if self.scanner.is_at_end() {
                            Err(LexerError::UnterminatedString)
                        } else {
                            Ok(None)
                        }
                    }
                }
            };
        }

        // Non-fstring prefixes
        if let Some('f') = self.scanner.current_char() {
            if self.scanner.peek() == Some('"') {
                self.scanner.advance(); // f
                self.scanner.advance(); // "
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
            } else {
                self.parse_normal(start)
            }
        } else if let Some('r') = self.scanner.current_char() {
            if self.scanner.peek() == Some('"') {
                self.scanner.advance(); // r
                self.scanner.advance(); // "
                let raw = parse_raw_string(&mut self.scanner)?;
                Ok(Some(Token::new(TokenKind::String(raw), start.line, start.column)))
            } else if self.scanner.peek() == Some('\'') {
                self.scanner.advance(); // r
                self.scanner.advance(); // '
                let raw_char = parse_char_raw(&mut self.scanner)?;
                Ok(Some(Token::new(TokenKind::Char(raw_char), start.line, start.column)))
            } else {
                self.parse_normal(start)
            }
        } else if let Some('"') = self.scanner.current_char() {
            self.scanner.advance();
            let s = parse_string_normal(&mut self.scanner)?;
            Ok(Some(Token::new(TokenKind::String(s), start.line, start.column)))
        } else if let Some('\'') = self.scanner.current_char() {
            self.scanner.advance();
            let ch = parse_char_normal(&mut self.scanner)?;
            Ok(Some(Token::new(TokenKind::Char(ch), start.line, start.column)))
        } else {
            self.parse_normal(start)
        }
    }

    /// Dispatch normal token parsing (identifiers, numbers, operators/punctuation).
    fn parse_normal(&mut self, start: Position) -> Result<Option<Token>, LexerError> {
        if let Some(c) = self.scanner.current_char()
            && is_identifier_start(c)
        {
            let name = parse_identifier(&mut self.scanner)?;
            if let Some(kw) = from_str(&name) {
                return Ok(Some(Token::new(TokenKind::Keyword(kw), start.line, start.column)));
            } else {
                return Ok(Some(Token::new(TokenKind::Identifier(name), start.line, start.column)));
            }
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
        match tokens[1].kind {
            TokenKind::Identifier(ref name) => assert_eq!(name, "x"),
            _ => panic!("Identifier expected"),
        }
        match tokens[2].kind {
            TokenKind::Assign => {}
            _ => panic!("Assign expected"),
        }
        match tokens[3].kind {
            TokenKind::Number(ref n) => assert_eq!(n, "10"),
            _ => panic!("Number expected"),
        }
        match tokens[4].kind {
            TokenKind::Semicolon => {}
            _ => panic!("Semicolon expected"),
        }
        match tokens[5].kind {
            TokenKind::Eof => {}
            _ => panic!("Eof expected"),
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
