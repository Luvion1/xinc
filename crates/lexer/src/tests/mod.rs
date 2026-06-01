//! Lexer integration tests.

mod keyword;
mod operator;
mod punctuation;

use crate::tokenize;

#[test]
fn test_identifier_simple() {
    assert!(!tokenize("x").unwrap().is_empty());
}
#[test]
fn test_identifier_with_num() {
    assert!(!tokenize("var123").unwrap().is_empty());
}
#[test]
fn test_identifier_underscore() {
    assert!(!tokenize("_private").unwrap().is_empty());
}
#[test]
fn test_number_int() {
    assert!(!tokenize("42").unwrap().is_empty());
}
#[test]
fn test_number_float() {
    assert!(!tokenize("3.14").unwrap().is_empty());
}
#[test]
fn test_string_double() {
    assert!(!tokenize("\"hello\"").unwrap().is_empty());
}
#[test]
fn test_string_single() {
    assert!(!tokenize("'a'").unwrap().is_empty());
}
#[test]
fn test_string_empty() {
    assert!(!tokenize("\"\"").unwrap().is_empty());
}

#[test]
fn test_simple_let() {
    assert!(!tokenize("let x = 1;").unwrap().is_empty());
}
#[test]
fn test_simple_fn() {
    assert!(!tokenize("fn foo() {}").unwrap().is_empty());
}
#[test]
fn test_simple_if() {
    assert!(!tokenize("if true {}").unwrap().is_empty());
}
#[test]
fn test_simple_while() {
    assert!(!tokenize("while true {}").unwrap().is_empty());
}
#[test]
fn test_simple_return() {
    assert!(!tokenize("return 42;").unwrap().is_empty());
}
#[test]
fn test_negative_number() {
    assert!(!tokenize("-5").unwrap().is_empty());
}
#[test]
fn test_hex_number() {
    assert!(!tokenize("0xFF").unwrap().is_empty());
}
#[test]
fn test_binary_number() {
    assert!(!tokenize("0b1010").unwrap().is_empty());
}
#[test]
fn test_octal_number() {
    assert!(!tokenize("0o77").unwrap().is_empty());
}
#[test]
fn test_exponent_number() {
    assert!(!tokenize("1e10").unwrap().is_empty());
}
#[test]
fn test_underscore_number() {
    assert!(!tokenize("1_000").unwrap().is_empty());
}
#[test]
fn test_string_with_escape_n() {
    assert!(!tokenize("\"a\\nb\"").unwrap().is_empty());
}
#[test]
fn test_string_with_escape_t() {
    assert!(!tokenize("\"a\\tb\"").unwrap().is_empty());
}
#[test]
fn test_string_with_escape_r() {
    assert!(!tokenize("\"a\\rb\"").unwrap().is_empty());
}
#[test]
fn test_string_with_unicode() {
    assert!(tokenize("\"\\u{1F600}\"").is_err());
}
#[test]
fn test_string_with_quote_inside() {
    assert!(!tokenize("\"'inside'\"").unwrap().is_empty());
}
#[test]
fn test_comparison_combined() {
    assert!(!tokenize("< > <= >=").unwrap().is_empty());
}
#[test]
fn test_equality_combined() {
    assert!(!tokenize("== !=").unwrap().is_empty());
}
#[test]
fn test_bitwise_combined() {
    assert!(!tokenize("& | ^ ~").unwrap().is_empty());
}
#[test]
fn test_shift_combined() {
    assert!(!tokenize("<< >>").unwrap().is_empty());
}
#[test]
fn test_keywords_combined() {
    assert!(!tokenize("let fn if else").unwrap().is_empty());
}
#[test]
fn test_mixed_tokens() {
    assert!(!tokenize("let x = 1 + 2;").unwrap().is_empty());
}
#[test]
fn test_fn_with_params() {
    assert!(!tokenize("fn f(a: i32) {}").unwrap().is_empty());
}
#[test]
fn test_fn_with_return() {
    assert!(!tokenize("fn f() -> i32 {}").unwrap().is_empty());
}
#[test]
fn test_if_else() {
    assert!(!tokenize("if a {} else {}").unwrap().is_empty());
}
#[test]
fn test_nested_parens() {
    assert!(!tokenize("((x))").unwrap().is_empty());
}
#[test]
fn test_nested_brackets() {
    assert!(!tokenize("[[[]]]").unwrap().is_empty());
}
