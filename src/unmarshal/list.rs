use super::{UnmarshalError, UnmarshalIter, Unmarshalable};

impl<T: Unmarshalable> Unmarshalable for Vec<T> {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        let first = u.next_non_whitespace();
        if first != Some('[') {
            return Err(u.unexpected(first));
        }

        if u.peek_non_whitespace() == Some(&']') {
            u.next();
            return Ok(vec![]);
        }

        let mut elements = vec![T::unmarshal_json_with_state(u)?];

        while let Some(c) = u.next_non_whitespace() {
            match c {
                ',' => elements.push(T::unmarshal_json_with_state(u)?),
                ']' => return Ok(elements.into_iter().collect()),
                _ => return Err(u.unexpected_char(c)),
            }
        }
        Err(UnmarshalError::EndOfChars)
    }
}
