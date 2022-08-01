use super::{UnmarshalError, UnmarshalIter, Unmarshalable};

// Use Option<> for nullable values
impl<T: Unmarshalable> Unmarshalable for Option<T> {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        if u.peek_non_whitespace() == Some(&'n') {
            u.next();
            match u.next() {
                Some('u') => (),
                unexpected => return Err(u.unexpected(unexpected)),
            }
            match u.next() {
                Some('l') => (),
                unexpected => return Err(u.unexpected(unexpected)),
            }
            match u.next() {
                Some('l') => (),
                unexpected => return Err(u.unexpected(unexpected)),
            }
            Ok(None)
        } else {
            Ok(Some(T::unmarshal_json_with_state(u)?))
        }
    }
}

// Use Option<()> for required nulls
impl Unmarshalable for Option<()> {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        match u.next_non_whitespace() {
            Some('n') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }
        match u.next() {
            Some('u') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }
        match u.next() {
            Some('l') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }
        match u.next() {
            Some('l') => (),
            unexpected => return Err(u.unexpected(unexpected)),
        }
        Ok(None)
    }
}
