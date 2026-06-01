//! f-string parsing.
//!
//! Two helpers cooperate to turn an f-string literal into a sequence of
//! [`TokenKind::String`] (the literal fragments) and [`TokenKind::Identifier`]
//! (the expression placeholders):
//!
//! - [`parse_fstring_fragment`] — reads the literal text between `{` braces
//!   or the closing `"`, processing escapes. Stops at the first unescaped
//!   `{` or `"`.
//! - [`parse_fstring_placeholder`] — reads a single `{ ... }` block
//!   including brace nesting, returning the inner source text. The lexer
//!   hands the inner text back to the parser later (one re-tokenization
//!   per placeholder) so that real expressions — not just identifiers —
//!   are valid inside `{ ... }`.
//!
//! # State ownership
//!
//! The active-f-string state (brace depth, the accumulator) is held
//! by the outer [`crate::lexer::inner::scanner::Lexer`], **not** by the
//! scanner cursor. The functions in this module are pure parsers: they
//! take a `&mut Scanner` and read from the source bytes. The lexer
//! stitches the fragments together at a higher layer.
//!
//! # Errors
//!
//! Returns [`crate::error::LexerError::UnterminatedString`] if the
//! f-string is never closed; the lexer converts this into a regular
//! error before the parser ever sees the placeholder.

use super::super::Scanner;
use crate::error::LexerError;

/// Parse the literal text fragment between `{` placeholders in an
/// f-string.
///
/// Reads from the current scanner position, processing escape
/// sequences identically to a regular string. Stops when it sees
/// either an unescaped `{` (start of a placeholder) or `"` (end of the
/// f-string). The scanner is left positioned **on** the delimiter; the
/// caller decides whether the delimiter is a `{` placeholder or a `"`
/// terminator.
pub fn parse_fstring_fragment(scanner: &mut Scanner) -> Result<String, LexerError> {
    let mut result = String::new();
    while let Some(c) = scanner.current_char() {
        match c {
            '\\' => {
                scanner.advance();
                        let Some(esc) = scanner.advance() else {
                            return Err(LexerError::UnterminatedString);
                        };
                match esc {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    'u' => {
                        if scanner.current_char() != Some('{') {
                            return Err(LexerError::InvalidEscape {
                                char: 'u',
                                position: scanner.byte_offset(),
                            });
                        }
                        scanner.advance(); // {
                        let mut hex = String::new();
                        while let Some(d) = scanner.current_char() {
                            if d == '}' {
                                break;
                            }
                            if d.is_ascii_hexdigit() {
                                hex.push(scanner.advance().unwrap());
                            } else {
                                return Err(LexerError::InvalidEscape {
                                    char: d,
                                    position: scanner.byte_offset(),
                                });
                            }
                        }
                        scanner.expect('}')?;
                        let code = u32::from_str_radix(&hex, 16).map_err(|_| {
                            LexerError::InvalidNumber { reason: "Invalid hex in \\u{}".into() }
                        })?;
                        if code > 0x0010_FFFF || (0xD800..=0xDFFF).contains(&code) {
                            return Err(LexerError::InvalidEscape {
                                char: 'u',
                                position: scanner.byte_offset() - hex.len() - 2,
                            });
                        }
                        result.push(char::from_u32(code).ok_or_else(|| {
                            LexerError::InvalidEscape { char: 'u', position: scanner.byte_offset() }
                        })?);
                    }
                    _ => {
                        return Err(LexerError::InvalidEscape {
                            char: esc,
                            position: scanner.byte_offset(),
                        });
                    }
                }
            }
            '{' | '"' => {
                break;
            }
            _ => result.push(scanner.advance().unwrap()),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_simple() {
        let mut s = Scanner::new("hello");
        let frag = parse_fstring_fragment(&mut s).unwrap();
        assert_eq!(frag, "hello");
        assert!(s.is_at_end());
    }

    #[test]
    fn test_fragment_stops_at_brace() {
        let mut s = Scanner::new("hello{world}");
        let frag = parse_fstring_fragment(&mut s).unwrap();
        assert_eq!(frag, "hello");
        assert_eq!(s.current_char(), Some('{'));
    }

    #[test]
    fn test_fragment_stops_at_quote() {
        let mut s = Scanner::new("hello\"world");
        let frag = parse_fstring_fragment(&mut s).unwrap();
        assert_eq!(frag, "hello");
        assert_eq!(s.current_char(), Some('"'));
    }

    #[test]
    fn test_fragment_escape() {
        let mut s = Scanner::new("line\\nbreak");
        let frag = parse_fstring_fragment(&mut s).unwrap();
        assert_eq!(frag, "line\nbreak");
    }
}
