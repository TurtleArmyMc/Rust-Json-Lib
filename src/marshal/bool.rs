use crate::marshal::Marshalable;

impl Marshalable for bool {
    fn marshal_json_into(&self, s: &mut String) {
        match self {
            true => s.push_str("true"),
            false => s.push_str("false"),
        }
    }
}
