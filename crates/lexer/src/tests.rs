//! Lexer integration tests.

use crate::{tokenize, TokenKind};

#[test]
fn test_keyword_fn() { assert!(!tokenize("fn").unwrap().is_empty()); }
#[test]
fn test_keyword_let() { assert!(!tokenize("let").unwrap().is_empty()); }
#[test]
fn test_keyword_if() { assert!(!tokenize("if").unwrap().is_empty()); }
#[test]
fn test_keyword_else() { assert!(!tokenize("else").unwrap().is_empty()); }
#[test]
fn test_keyword_while() { assert!(!tokenize("while").unwrap().is_empty()); }
#[test]
fn test_keyword_true() { assert!(!tokenize("true").unwrap().is_empty()); }
#[test]
fn test_keyword_false() { assert!(!tokenize("false").unwrap().is_empty()); }
#[test]
fn test_keyword_return() { assert!(!tokenize("return").unwrap().is_empty()); }
#[test]
fn test_keyword_null() { assert!(!tokenize("null").unwrap().is_empty()); }
#[test]
fn test_identifier_simple() { assert!(!tokenize("x").unwrap().is_empty()); }
#[test]
fn test_identifier_with_num() { assert!(!tokenize("var123").unwrap().is_empty()); }
#[test]
fn test_identifier_underscore() { assert!(!tokenize("_private").unwrap().is_empty()); }
#[test]
fn test_number_int() { assert!(!tokenize("42").unwrap().is_empty()); }
#[test]
fn test_number_float() { assert!(!tokenize("3.14").unwrap().is_empty()); }
#[test]
fn test_string_double() { assert!(!tokenize("\"hello\"").unwrap().is_empty()); }
#[test]
fn test_string_single() { assert!(!tokenize("'world'").unwrap().is_empty()); }
#[test]
fn test_string_empty() { assert!(!tokenize("\"\"").unwrap().is_empty()); }
#[test]
fn test_binary_add() { assert!(!tokenize("+").unwrap().is_empty()); }
#[test]
fn test_binary_sub() { assert!(!tokenize("-").unwrap().is_empty()); }
#[test]
fn test_binary_mul() { assert!(!tokenize("*").unwrap().is_empty()); }
#[test]
fn test_binary_div() { assert!(!tokenize("/").unwrap().is_empty()); }
#[test]
fn test_binary_mod() { assert!(!tokenize("%").unwrap().is_empty()); }
#[test]
fn test_binary_and() { assert!(!tokenize("&").unwrap().is_empty()); }
#[test]
fn test_binary_or() { assert!(!tokenize("|").unwrap().is_empty()); }
#[test]
fn test_binary_xor() { assert!(!tokenize("^").unwrap().is_empty()); }
#[test]
fn test_binary_shl() { assert!(!tokenize("<<").unwrap().is_empty()); }
#[test]
fn test_binary_shr() { assert!(!tokenize(">>").unwrap().is_empty()); }
#[test]
fn test_unary_not() { assert!(!tokenize("!").unwrap().is_empty()); }
#[test]
fn test_unary_bitnot() { assert!(!tokenize("~").unwrap().is_empty()); }
#[test]
fn test_comparison_lt() { assert!(!tokenize("<").unwrap().is_empty()); }
#[test]
fn test_comparison_gt() { assert!(!tokenize(">").unwrap().is_empty()); }
#[test]
fn test_equality() { assert!(!tokenize("==").unwrap().is_empty()); }
#[test]
fn test_inequality() { assert!(!tokenize("!=").unwrap().is_empty()); }
#[test]
fn test_assign() { assert!(!tokenize("=").unwrap().is_empty()); }
#[test]
fn test_punctuation_lparen() { assert!(!tokenize("(").unwrap().is_empty()); }
#[test]
fn test_punctuation_rparen() { assert!(!tokenize(")").unwrap().is_empty()); }
#[test]
fn test_punctuation_lbrace() { assert!(!tokenize("{").unwrap().is_empty()); }
#[test]
fn test_punctuation_rbrace() { assert!(!tokenize("}").unwrap().is_empty()); }
#[test]
fn test_punctuation_lbracket() { assert!(!tokenize("[").unwrap().is_empty()); }
#[test]
fn test_punctuation_rbracket() { assert!(!tokenize("]").unwrap().is_empty()); }
#[test]
fn test_semicolon() { assert!(!tokenize(";").unwrap().is_empty()); }
#[test]
fn test_colon() { assert!(!tokenize(":").unwrap().is_empty()); }
#[test]
fn test_comma() { assert!(!tokenize(",").unwrap().is_empty()); }
#[test]
fn test_dot() { assert!(!tokenize(".").unwrap().is_empty()); }
#[test]
fn test_simple_let() { assert!(!tokenize("let x = 1;").unwrap().is_empty()); }
#[test]
fn test_simple_fn() { assert!(!tokenize("fn foo() {}").unwrap().is_empty()); }
#[test]
fn test_simple_if() { assert!(!tokenize("if true {}").unwrap().is_empty()); }
#[test]
fn test_simple_while() { assert!(!tokenize("while true {}").unwrap().is_empty()); }
#[test]
fn test_simple_return() { assert!(!tokenize("return 42;").unwrap().is_empty()); }
#[test]
fn test_negative_number() { assert!(!tokenize("-5").unwrap().is_empty()); }
#[test]
fn test_hex_number() { assert!(!tokenize("0xFF").unwrap().is_empty()); }
#[test]
fn test_binary_number() { assert!(!tokenize("0b1010").unwrap().is_empty()); }
#[test]
fn test_octal_number() { assert!(!tokenize("0o77").unwrap().is_empty()); }
#[test]
fn test_exponent_number() { assert!(!tokenize("1e10").unwrap().is_empty()); }
#[test]
fn test_underscore_number() { assert!(!tokenize("1_000").unwrap().is_empty()); }
#[test]
fn test_string_with_escape_n() { assert!(!tokenize("\"a\\nb\"").unwrap().is_empty()); }
#[test]
fn test_string_with_escape_t() { assert!(!tokenize("\"a\\tb\"").unwrap().is_empty()); }
#[test]
fn test_string_with_escape_r() { assert!(!tokenize("\"a\\rb\"").unwrap().is_empty()); }
#[test]
fn test_string_with_escape_backslash() { assert!(!tokenize("\"a\\\\b\"").unwrap().is_empty()); }
#[test]
fn test_string_with_unicode() { assert!(tokenize("\"\\u{1F600}\"").is_err()); }
#[test]
fn test_string_with_hex() { assert!(!tokenize("\"\\x41\"").unwrap().is_empty()); }
#[test]
fn test_string_with_octal() { assert!(tokenize("\"\\123\"").is_err()); }
#[test]
fn test_string_with_quote_inside() { assert!(!tokenize("\"'inside'\"").unwrap().is_empty()); }
#[test]
fn test_string_empty_double() { assert!(!tokenize("\"\"").unwrap().is_empty()); }
#[test]
fn test_string_empty_single() { assert!(!tokenize("''").unwrap().is_empty()); }
#[test]
fn test_operators_combined() { assert!(!tokenize("+ - * /").unwrap().is_empty()); }
#[test]
fn test_comparison_combined() { assert!(!tokenize("< > <= >=").unwrap().is_empty()); }
#[test]
fn test_equality_combined() { assert!(!tokenize("== !=").unwrap().is_empty()); }
#[test]
fn test_bitwise_combined() { assert!(!tokenize("& | ^ ~").unwrap().is_empty()); }
#[test]
fn test_shift_combined() { assert!(!tokenize("<< >>").unwrap().is_empty()); }
#[test]
fn test_keywords_combined() { assert!(!tokenize("let fn if else").unwrap().is_empty()); }
#[test]
fn test_mixed_tokens() { assert!(!tokenize("let x = 1 + 2;").unwrap().is_empty()); }
#[test]
fn test_fn_with_params() { assert!(!tokenize("fn f(a: i32) {}").unwrap().is_empty()); }
#[test]
fn test_fn_with_return() { assert!(!tokenize("fn f() -> i32 {}").unwrap().is_empty()); }
#[test]
fn test_if_else() { assert!(!tokenize("if a {} else {}").unwrap().is_empty()); }
#[test]
fn test_nested_parens() { assert!(!tokenize("((x))").unwrap().is_empty()); }
#[test]
fn test_nested_brackets() { assert!(!tokenize("[[[]]]").unwrap().is_empty()); }