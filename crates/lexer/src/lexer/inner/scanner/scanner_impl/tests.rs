//! Scanner tests.

use super::Scanner;

#[test]
fn test_new_scanner() {
    let s = Scanner::new("abc");
    assert!(!s.is_at_end());
    assert_eq!(s.current_char(), Some('a'));
    assert_eq!(s.position().line, 1);
    assert_eq!(s.position().column, 1);
}

#[test]
fn test_advance() {
    let mut s = Scanner::new("ab");
    s.advance();
    assert_eq!(s.current_char(), Some('b'));
    assert_eq!(s.column, 2);
}

#[test]
fn test_newline_lf() {
    let mut s = Scanner::new("a\nb");
    s.advance();
    s.advance();
    assert_eq!(s.line(), 2);
    assert_eq!(s.column(), 1);
    assert_eq!(s.current_char(), Some('b'));
}

#[test]
fn test_newline_crlf() {
    let mut s = Scanner::new("a\r\nb");
    s.advance();
    s.advance();
    assert_eq!(s.line(), 2);
    assert_eq!(s.column(), 1);
    assert_eq!(s.current_char(), Some('b'));
}

#[test]
fn test_peek() {
    let mut s = Scanner::new("ab");
    assert_eq!(s.peek(), Some('b'));
    s.advance();
    assert_eq!(s.peek(), None);
}

#[test]
fn test_expect_success() {
    let mut s = Scanner::new("a");
    assert!(s.expect('a').is_ok());
    assert!(s.is_at_end());
}

#[test]
fn test_expect_failure() {
    let mut s = Scanner::new("b");
    let err = s.expect('a').unwrap_err();
    match err {
        crate::error::LexerError::InvalidChar { character, position } => {
            assert_eq!(character, 'a');
            assert_eq!(position, 0);
        }
        _ => panic!("Wrong error variant"),
    }
}
