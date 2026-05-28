//! Unicode normalization scalar validation.
//!
//! Validates Unicode scalar values in string literals.

/// Validates a Unicode scalar value.
///
/// Returns true if the character is a valid Unicode scalar.
pub fn is_valid_scalar(ch: char) -> bool {
    let cp = ch as u32;
    (0x0000..=0xD7FF).contains(&cp) || (0xE000..=0x10FFFF).contains(&cp)
}

/// Validates a string as valid Unicode scalars.
pub fn validate_string(s: &str) -> bool {
    s.chars().all(is_valid_scalar)
}