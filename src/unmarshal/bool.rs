use crate::Unmarshalable;

use super::{unmarshal_iter::UnmarshalIter, unmarshalable::UnmarshalError};

impl Unmarshalable for bool {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        match u.next_non_whitespace() {
            Some('t') => {
                match u.next() {
                    Some('r') => (),
                    unexpected => return Err(u.unexpected(unexpected)),
                }
                match u.next() {
                    Some('u') => (),
                    unexpected => return Err(u.unexpected(unexpected)),
                }
                match u.next() {
                    Some('e') => Ok(true),
                    unexpected => Err(u.unexpected(unexpected)),
                }
            }
            Some('f') => {
                match u.next() {
                    Some('a') => (),
                    unexpected => return Err(u.unexpected(unexpected)),
                }
                match u.next() {
                    Some('l') => (),
                    unexpected => return Err(u.unexpected(unexpected)),
                }
                match u.next() {
                    Some('s') => (),
                    unexpected => return Err(u.unexpected(unexpected)),
                }
                match u.next() {
                    Some('e') => Ok(false),
                    unexpected => Err(u.unexpected(unexpected)),
                }
            }
            unexpected => Err(u.unexpected(unexpected)),
        }
    }
}
