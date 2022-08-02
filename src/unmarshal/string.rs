use crate::Unmarshalable;

use super::{unmarshal_iter::UnmarshalIter, unmarshalable::UnmarshalError};

impl Unmarshalable for String {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        let first = u.next_non_whitespace();
        if first != Some('"') {
            return Err(u.unexpected(first));
        }

        let mut out = String::new();
        while let Some(c) = u.next() {
            match c {
                '"' => return Ok(out),
                '\\' => match u.next() {
                    Some(escaped @ ('\\' | '"' | '/')) => out.push(escaped),
                    Some('b') => out.push('\x08'), // Literal backspace
                    Some('f') => out.push('\x0c'), // Formfeed
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('u') => out.push(try_read_unicode_escape(u)?),
                    unexpected => return Err(u.unexpected(unexpected)),
                },
                _ => out.push(c),
            }
        }
        Err(UnmarshalError::EndOfChars)
    }
}

// Attempts to read unicode escape as "XXXX" or "XXXX\uXXXX" where X is a hex digit
fn try_read_unicode_escape(u: &mut UnmarshalIter) -> Result<char, UnmarshalError> {
    let h0 = u.try_next_digit(16)?;
    let h1 = u.try_next_digit(16)?;
    let h2 = u.try_next_digit(16)?;
    let h3 = u.try_next_digit(16)?;
    let lead = (h0 << 12) + (h1 << 8) + (h2 << 4) + h3;

    if !(0xd800..=0xdbff).contains(&lead) {
        char::from_u32(lead).ok_or(u.unexpected_char('u'))
    } else {
        // If lead is a surrogate, try to get the trailing value
        match u.next() {
            Some('\\') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }
        match u.next() {
            Some('u') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }

        let h0 = u.try_next_digit(16)?;
        let h1 = u.try_next_digit(16)?;
        let h2 = u.try_next_digit(16)?;
        let h3 = u.try_next_digit(16)?;
        let trail = (h0 << 12) + (h1 << 8) + (h2 << 4) + h3;

        // This line is taken from the Python json lib because I couldn't find any
        // resources explaining exactly how decoding JSON unicode escapes works
        let n = 0x10000 + (((lead - 0xd800) << 10) | (trail - 0xdc00));
        char::from_u32(n).ok_or(u.unexpected_char('u'))
    }
}
