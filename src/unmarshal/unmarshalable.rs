use std::str::Chars;

use super::UnmarshalIter;

pub trait Unmarshalable: Sized {
    /// Unmarshals the type using characters provided from the unmarshaler.
    /// Should exhaust all characters necessary to unmarshal the type, and
    /// leave any remaining characters to allow for nested unmarshaling.
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError>;

    fn unmarshal_json(chars: Chars) -> Result<Self, UnmarshalError> {
        let mut u = UnmarshalIter::new(chars);
        let unmarshaled = Self::unmarshal_json_with_state(&mut u)?;
        u.check_finished()?;
        Ok(unmarshaled)
    }
}

#[derive(Debug, PartialEq)]
pub enum UnmarshalError {
    UnexpectedChar { c: char, row: u32, col: u32 },
    EndOfChars,
}
