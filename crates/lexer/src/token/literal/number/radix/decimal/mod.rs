//! Decimal number parser.
//! Standard base-10 integer parsing.

/// Parse decimal string to u64.
///
/// # Arguments
/// * `input` - Decimal digits (e.g., "42")
///
/// # Returns
/// Parsed value or None if invalid digit
pub fn parse(input: &str) -> Option<u64> {
    // Accumulator for result
    let mut result = 0u64;

    // Process each character
    for ch in input.chars() {
        // Get decimal digit value
        let digit = u64::from(ch.to_digit(10)?);

        // Multiply accumulator by 10 and add digit
        result = result * 10 + digit;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(parse("0"), Some(0));
    }

    #[test]
    fn test_42() {
        assert_eq!(parse("42"), Some(42));
    }

    #[test]
    fn test_1000() {
        assert_eq!(parse("1000"), Some(1000));
    }
}
