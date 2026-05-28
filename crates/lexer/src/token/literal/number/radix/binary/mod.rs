//! Binary number parser.
//! Converts binary string (0/1) to u64 value.

/// Parser for binary numbers.
pub struct BinaryParser;

/// Parse binary string to integer.
///
/// Processes each digit, validating it's 0 or 1.
/// Builds result using bit shifting.
///
/// # Arguments
/// * `input` - Binary digit string (e.g., "1010")
///
/// # Returns
/// * `Some(u64)` - Parsed value
/// * `None` - Invalid digit found
pub fn parse(input: &str) -> Option<u64> {
    // Start with zero
    let mut result = 0u64;

    // Process each character
    for ch in input.chars() {
        match ch {
            // Binary digit 0: shift left
            '0' => {
                result <<= 1;
            }
            // Binary digit 1: shift left and set LSB
            '1' => {
                result = (result << 1) | 1;
            }
            // Invalid digit
            _ => return None,
        }
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
    fn test_one() {
        assert_eq!(parse("1"), Some(1));
    }

    #[test]
    fn test_1010() {
        assert_eq!(parse("1010"), Some(10));
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(parse("11111111"), Some(255));
    }

    #[test]
    fn test_invalid_digit() {
        assert_eq!(parse("102"), None);
    }
}
