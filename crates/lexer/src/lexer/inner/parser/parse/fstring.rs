//! f-string interpolation parsing.
//!
//! Handles the literal parts of f-strings, which are the text between
//! expression blocks (`{...}`). This parser reads characters until a `{`
//! or the closing `"` of the f-string, processing escape sequences just
//! like a normal string.

use super::super::scanner::Scanner;
use crate::error::LexerError;

/// Parse an f-string fragment (the literal text part).
///
/// Starts after the opening `"` of an f-string. Reads characters until
/// encountering a `{` (which begins an embedded expression) or a `"` (which
/// ends the f-string). Escape sequences are processed using the same rules
/// as normal strings.
///
/// The scanner is left positioned at the delimiter (`{` or `"`) after the
/// fragment is consumed. The caller must handle the delimiter.
pub fn parse_fstring_fragment(scanner: &mut Scanner) -> Result<String, LexerError> {
    let mut result = String::new();
    while let Some(c) = scanner.current_char() {
        match c {
            '\\' => {
                scanner.advance();
                let esc = match scanner.advance() {
                    Some(e) => e,
                    None => return Err(LexerError::UnterminatedString),
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
                        if code > 0x10FFFF || (0xD800..=0xDFFF).contains(&code) {
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
