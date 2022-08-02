use crate::marshal::Marshalable;

impl<T> Marshalable for Option<T>
where
    T: Marshalable,
{
    fn marshal_json_into(&self, s: &mut String) {
        match self {
            Some(e) => e.marshal_json_into(s),
            None => s.push_str("null"),
        }
    }
}

impl Marshalable for Option<()> {
    fn marshal_json_into(&self, s: &mut String) {
        s.push_str("null")
    }
}
