//! Octal number parser.
//! Converts octal string (0-7) to u64 value.

/// Parse octal string.
///
/// # Arguments
/// * `input` - Octal digit string (e.g., "755")
///
/// # Returns
/// Parsed value or None if invalid digit
pub fn parse(input: &str) -> Option<u64> {
    // Start accumulator
    let mut result = 0u64;

    // Process each character as octal digit
    for ch in input.chars() {
        // Convert char to digit in base 8
        let digit = u64::from(ch.to_digit(8)?);

        // Multiply existing by 8 and add digit
        result = result * 8 + digit;
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
    fn test_seven() {
        assert_eq!(parse("7"), Some(7));
    }

    #[test]
    fn test_77() {
        assert_eq!(parse("77"), Some(63));
    }

    #[test]
    fn test_755() {
        assert_eq!(parse("755"), Some(493));
    }
}
