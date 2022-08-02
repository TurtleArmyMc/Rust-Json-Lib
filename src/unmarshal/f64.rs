use crate::Unmarshalable;

use super::{unmarshal_iter::UnmarshalIter, unmarshalable::UnmarshalError};

impl Unmarshalable for f64 {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        // Unmarshal before decimal point and/or exponent
        let sign = if Some(&'-') == u.peek_non_whitespace() {
            u.next();
            -1.0
        } else {
            1.0
        };
        let mut f = u.try_next_digit(10)? as f64;
        while let Some(d) = u.peek().and_then(|c| c.to_digit(10)) {
            f = f * 10.0 + d as f64;
            u.next();
        }

        // Unmarshal decimal
        if Some(&'.') == u.peek() {
            u.next();
            let mut dec_place = 0.1;
            f += u.try_next_digit(10)? as f64 * dec_place;
            while let Some(d) = u.peek().and_then(|c| c.to_digit(10)) {
                dec_place /= 10.0;
                f += d as f64 * dec_place;
                u.next();
            }
        }

        // Unmarshal exponent
        match u.peek() {
            Some(&('e' | 'E')) => {
                u.next();
                Ok(f * 10f64.powi(try_unmarshal_exponent(u)?) * sign)
            }
            _ => Ok(f * sign),
        }
    }
}

fn try_unmarshal_exponent(u: &mut UnmarshalIter) -> Result<i32, UnmarshalError> {
    let (mut exponent, exponent_sign) = match u.next() {
        Some('-') => (u.try_next_digit(10)? as i32, -1),
        Some('+') => (u.try_next_digit(10)? as i32, 1),
        Some(c) => (
            c.to_digit(10).ok_or_else(|| u.unexpected_char(c))? as i32,
            1,
        ),
        None => return Err(UnmarshalError::EndOfChars),
    };
    while let Some(d) = u.peek().and_then(|c| c.to_digit(10)) {
        exponent += exponent * 10 + d as i32;
        u.next();
    }
    Ok(exponent * exponent_sign)
}

/// Unmarshals a float from an int that has already started being read.
/// Starts from either a decimal point or from the exponent.
/// The sign must be passed in addition to the int in case the float began
/// with a negative 0.
pub(crate) fn unmarshal_float_from_int(
    i: i64,
    sign: f64,
    u: &mut UnmarshalIter,
) -> Result<f64, UnmarshalError> {
    match u.next() {
        // Unmarshal decimal and maybe exponent
        Some('.') => {
            let mut decimal = 0.0;
            let mut dec_place = 0.1;
            decimal += u.try_next_digit(10)? as f64 * dec_place;
            while let Some(d) = u.peek().and_then(|c| c.to_digit(10)) {
                dec_place /= 10.0;
                decimal += d as f64 * dec_place;
                u.next();
            }

            let f = i as f64 + (decimal * sign);
            match u.peek() {
                Some(&('e' | 'E')) => {
                    u.next();
                    Ok(f * 10f64.powi(try_unmarshal_exponent(u)?))
                }
                _ => Ok(f),
            }
        }

        // Unmarshal exponent
        Some('e' | 'E') => Ok(i as f64 * 10f64.powi(try_unmarshal_exponent(u)?)),

        // Function expects u to continue to a float
        unexpected => Err(u.unexpected(unexpected)),
    }
}
