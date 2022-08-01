use super::{UnmarshalError, UnmarshalIter, Unmarshalable};

impl Unmarshalable for i64 {
    fn unmarshal_json_with_state(u: &mut UnmarshalIter) -> Result<Self, UnmarshalError> {
        let sign = if Some(&'-') == u.peek_non_whitespace() {
            u.next();
            -1
        } else {
            1
        };
        let mut i = u.try_next_digit(10)? as i64;
        while let Some(d) = u.peek().and_then(|c| c.to_digit(10)) {
            i = i * 10 + d as i64;
            u.next();
        }
        Ok(i * sign)
    }
}
