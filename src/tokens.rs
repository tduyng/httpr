/// Determines if byte is a token char
/// !, #, $, %, &, ', * +, -, ., ^, _, `, |, ~, digits, alphanumeric
pub fn is_token(b: u8) -> bool {
    b > 0x1f && b < 0x7f
}

// ASCII codes to accept as part of URI strings
// A-Z a-z 0-9 !#$%&'*+-._();:@=,/?[]~^
pub fn is_uri_token(ch: u8) -> bool {
    match ch {
        0..=b' ' => false,
        b'<' | b'>' => false,
        b'!'..=b'~' => true,
        0x7f.. => false,
    }
}

// ASCII codes to accept as part of header names
pub fn is_header_name_token(ch: u8) -> bool {
    matches!(ch, b'!' | b'#'..=b'/' | b'|' | b'~' | b'^' | b'_' | b'`' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z')
}

// ASCII codes to accept as part of header values
pub fn is_header_value_token(ch: u8) -> bool {
    match ch {
        0x9 => true,
        0x7f => false,
        b' '..=0xff => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b'A')]
    #[case(b'Z')]
    #[case(b'a')]
    #[case(b'z')]
    #[case(b'0')]
    #[case(b'9')]
    #[case(b'!')]
    #[case(b'#')]
    #[case(b'$')]
    #[case(b'%')]
    #[case(b'&')]
    #[case(b'\'')]
    #[case(b'*')]
    #[case(b'+')]
    #[case(b'-')]
    #[case(b'.')]
    #[case(b'^')]
    #[case(b'_')]
    #[case(b'`')]
    #[case(b'|')]
    #[case(b'~')]
    #[case(b'/')]
    #[case(b'@')]
    fn test_is_token(#[case] input: u8) {
        assert!(is_token(input));
    }

    #[rstest]
    #[case(b'A')]
    #[case(b'Z')]
    #[case(b'a')]
    #[case(b'z')]
    #[case(b'0')]
    #[case(b'9')]
    #[case(b'!')]
    #[case(b'#')]
    #[case(b'$')]
    #[case(b'%')]
    #[case(b'&')]
    #[case(b'\'')]
    #[case(b'*')]
    #[case(b'+')]
    #[case(b'-')]
    #[case(b'.')]
    #[case(b'^')]
    #[case(b'_')]
    #[case(b'`')]
    #[case(b'|')]
    #[case(b'~')]
    fn test_is_uri_token(#[case] input: u8) {
        assert!(is_uri_token(input));
    }

    #[rstest]
    #[case(b'A')]
    #[case(b'Z')]
    #[case(b'a')]
    #[case(b'z')]
    #[case(b'0')]
    #[case(b'9')]
    #[case(b'!')]
    #[case(b'#')]
    #[case(b'$')]
    #[case(b'%')]
    #[case(b'&')]
    #[case(b'_')]
    #[case(b'^')]
    #[case(b'`')]
    #[case(b'|')]
    #[case(b'~')]
    #[case(b'/')]
    fn test_is_header_name_token(#[case] input: u8) {
        assert!(is_header_name_token(input));
    }

    #[rstest]
    #[case(b'\t')]
    #[case(b' ')]
    #[case(b'!')]
    #[case(b'#')]
    #[case(b'$')]
    #[case(b'%')]
    #[case(b'&')]
    #[case(b'\'')]
    #[case(b'*')]
    #[case(b'+')]
    #[case(b'-')]
    #[case(b'.')]
    #[case(b'^')]
    #[case(b'_')]
    #[case(b'`')]
    #[case(b'|')]
    #[case(b'~')]
    #[case(b'/')]
    #[case(b':')]
    #[case(b';')]
    #[case(b'@')]
    #[case(b'[')]
    #[case(b']')]
    #[case(b'(')]
    #[case(b')')]
    #[case(b',')]
    #[case(b'?')]
    #[case(b'=')]
    fn test_is_header_value_token(#[case] input: u8) {
        assert!(is_header_value_token(input));
    }

    #[rstest]
    #[case(0x00)]
    #[case(0x1F)]
    #[case(0x7F)]
    fn test_is_not_header_value_token(#[case] input: u8) {
        assert!(!is_header_value_token(input));
    }
}
