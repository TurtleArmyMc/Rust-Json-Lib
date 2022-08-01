use std::{
    collections::{BTreeMap, HashMap},
    iter,
};

use super::{UnmarshalError, UnmarshalIter, Unmarshalable};

impl<T: Unmarshalable> Unmarshalable for HashMap<String, T> {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        let first = u.next_non_whitespace();
        if first != Some('{') {
            return Err(u.unexpected(first));
        }

        if u.peek_non_whitespace() == Some(&'}') {
            u.next();
            return Ok(HashMap::from_iter(iter::empty()));
        }

        let mut object: HashMap<String, T> = HashMap::from([unmarshal_pair(u)?]);
        while let Some(c) = u.next_non_whitespace() {
            match c {
                ',' => {
                    let (k, v) = unmarshal_pair(u)?;
                    object.insert(k, v);
                }
                '}' => return Ok(HashMap::from_iter(object.into_iter())),
                _ => return Err(u.unexpected_char(c)),
            }
        }
        Err(UnmarshalError::EndOfChars)
    }
}

impl<T: Unmarshalable> Unmarshalable for BTreeMap<String, T> {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        Ok(HashMap::<String, T>::unmarshal_json_with_state(u)?
            .into_iter()
            .collect())
    }
}

/// Unmarshals a key value pair separated with a ':'
fn unmarshal_pair<T: Unmarshalable>(u: &mut UnmarshalIter) -> Result<(String, T), UnmarshalError> {
    let key = String::unmarshal_json_with_state(u)?;
    match u.next_non_whitespace() {
        Some(':') => Ok((key, T::unmarshal_json_with_state(u)?)),
        unexpected => Err(u.unexpected(unexpected)),
    }
}
