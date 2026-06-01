//! Tests for keyword token definitions.

use super::*;

#[test]
fn test_fn_keyword() {
    assert_eq!(from_str("fn"), Some(Keyword::Fn));
}

#[test]
fn test_let_keyword() {
    assert_eq!(from_str("let"), Some(Keyword::Let));
}

#[test]
fn test_type_keywords() {
    assert_eq!(from_str("i32"), Some(Keyword::I32));
    assert_eq!(from_str("bool"), Some(Keyword::Bool));
}

#[test]
fn test_not_keyword() {
    assert_eq!(from_str("foo"), None);
}

#[test]
fn test_self_keyword() {
    assert_eq!(from_str("self"), Some(Keyword::Self_));
}

#[test]
fn test_self_type_keyword() {
    assert_eq!(from_str("Self"), Some(Keyword::SelfType));
}
