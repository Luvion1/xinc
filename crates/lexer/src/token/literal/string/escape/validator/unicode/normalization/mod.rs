//! Unicode normalization forms.
//! Defines NFC, NFD, NFKC, NFKD.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NormalizationForm {
    #[default]
    NFC,
    NFD,
    NFKC,
    NFKD,
}

/// Get name of normalization form.
pub fn form_name(form: NormalizationForm) -> &'static str {
    match form {
        NormalizationForm::NFC => "NFC",
        NormalizationForm::NFD => "NFD",
        NormalizationForm::NFKC => "NFKC",
        NormalizationForm::NFKD => "NFKD",
    }
}

/// Check if a character is a combining character.
pub fn is_combining_char(c: char) -> bool {
    let code = c as u32;
    (0x0300..=0x036F).contains(&code)
        || (0x1DC0..=0x1DFF).contains(&code)
        || (0x20D0..=0x20FF).contains(&code)
}

/// Check if two strings are equivalent under normalization (stub).
pub fn are_equivalent(a: &str, b: &str, form: NormalizationForm) -> bool {
    let na = normalize(a, form);
    let nb = normalize(b, form);
    na == nb
}

/// Check if a string is already normalized (stub).
pub fn is_normalized(input: &str, _form: NormalizationForm) -> bool {
    let _ = input;
    true
}

/// Stub normalize (just returns input).
pub fn normalize(input: &str, _form: NormalizationForm) -> String {
    input.to_string()
}

/// Submodule for Unicode scalar value operations.
pub mod scalar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(NormalizationForm::default(), NormalizationForm::NFC);
    }
    #[test]
    fn test_combining_char() {
        assert!(is_combining_char('\u{0301}'));
    }
    #[test]
    fn test_form_name() {
        assert_eq!(form_name(NormalizationForm::NFC), "NFC");
    }
}
