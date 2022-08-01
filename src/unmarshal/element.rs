use std::collections::HashMap;

use super::Unmarshalable;
use crate::{f64::unmarshal_float_from_int, Element, UnmarshalError, UnmarshalIter};

impl Unmarshalable for Element {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        // The peeked character must be cloned so u doesn't remain mutably
        // borrowed when attempting to report errors
        match u.peek_non_whitespace().cloned() {
            Some('{') => Ok(Element::JsonObject(
                HashMap::<String, Element>::unmarshal_json_with_state(u)?,
            )),
            Some('[') => Vec::<Element>::unmarshal_json_with_state(u).map(Element::JsonList),
            Some('"') => String::unmarshal_json_with_state(u).map(Element::JsonString),
            Some('n') => Option::<()>::unmarshal_json_with_state(u).and(Ok(Element::JsonNull)),
            Some('t' | 'f') => bool::unmarshal_json_with_state(u).map(Element::JsonBool),
            Some(num_start @ ('-' | '0'..='9')) => {
                // Attempts to read the number as an int, but returns a float
                // if a decimal or exponent is found.
                let i = i64::unmarshal_json_with_state(u)?;
                match u.peek().cloned() {
                    Some('.' | 'e' | 'E') => {
                        let sign = if num_start == '-' { -1.0 } else { 1.0 };
                        unmarshal_float_from_int(i, sign, u).map(Element::JsonFloat)
                    }
                    _ => Ok(Element::JsonInt(i)),
                }
            }
            unexpected => Err(u.unexpected(unexpected)),
        }
    }
}
