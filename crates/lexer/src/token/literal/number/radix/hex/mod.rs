//! Hexadecimal number parser.
//! Converts hex string (0-9, a-f, A-F) to u64.

/// Parse hexadecimal string.
///
/// Accepts both with and without "0x"/"0X" prefix.
///
/// # Arguments
/// * `input` - Hex digits (e.g., "FF" or "0xFF")
///
/// # Returns
/// Parsed value or None if invalid digit
pub fn parse(input: &str) -> Option<u64> {
    // Trim whitespace and strip optional 0x prefix
    let input = input.trim();
    let input = if input.len() >= 2 && input[0..2].eq_ignore_ascii_case("0x") {
        &input[2..]
    } else {
        input
    };

    // Return None if empty after stripping prefix
    if input.is_empty() {
        return None;
    }

    // Start with zero
    let mut result = 0u64;

    // Process each character as hex digit
    for ch in input.chars() {
        // Convert char to hex digit (0-15)
        let digit = u64::from(ch.to_digit(16)?);

        // Multiply by 16 and add digit
        result = result * 16 + digit;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(parse("F"), Some(15));
    }

    #[test]
    fn test_ff_uppercase() {
        assert_eq!(parse("FF"), Some(255));
    }

    #[test]
    fn test_ff_lowercase() {
        assert_eq!(parse("ff"), Some(255));
    }

    #[test]
    fn test_0xff() {
        assert_eq!(parse("0xFF"), Some(255));
    }
}
