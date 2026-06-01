//! Unicode validation and normalization.
//! Handles Unicode scalar values and text normalization.

/// Normalization forms submodule.
pub mod normalization;

use normalization::NormalizationForm;

/// Check if a Unicode code point is a valid scalar.
///
/// Valid range: U+0000..=U+10FFFF excluding surrogates.
pub fn is_valid_scalar(code: u32) -> bool {
    (0x0000..=0x0010_FFFF).contains(&code) && !(0xD800..=0xDFFF).contains(&code)
}

/// Check if a char is a valid Unicode scalar.
pub fn is_valid_unicode_char(c: char) -> bool {
    is_valid_scalar(c as u32)
}

/// Normalize a Unicode string (stub).
pub fn normalize(input: &str, _form: NormalizationForm) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii() {
        assert!(is_valid_scalar('A' as u32));
    }

    #[test]
    fn test_valid_unicode() {
        assert!(is_valid_scalar(0x1F600));
    }

    #[test]
    fn test_invalid_surrogate() {
        assert!(!is_valid_scalar(0xD800));
    }

    #[test]
    fn test_valid_char() {
        assert!(is_valid_unicode_char('A'));
    }

    #[test]
    fn test_normalize_nfc() {
        let s = "café";
        let n = normalize(s, NormalizationForm::NFC);
        assert_eq!(n, s);
    }
}
