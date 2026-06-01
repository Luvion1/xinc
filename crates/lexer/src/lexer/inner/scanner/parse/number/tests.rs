//! Tests for numeric literal parsing.

use super::super::super::Scanner;
use super::parse_number;

#[test]
fn test_decimal() {
    let mut s = Scanner::new("123");
    s.advance();
    assert!(parse_number(&mut s).is_ok());
}

#[test]
fn test_hex() {
    let mut s = Scanner::new("0x1af");
    assert!(parse_number(&mut s).is_ok());
}

#[test]
fn test_binary() {
    let mut s = Scanner::new("0b101");
    assert!(parse_number(&mut s).is_ok());
}

#[test]
fn test_octal() {
    let mut s = Scanner::new("0o755");
    assert!(parse_number(&mut s).is_ok());
}
