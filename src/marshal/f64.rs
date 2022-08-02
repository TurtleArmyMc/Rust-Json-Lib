use crate::marshal::Marshalable;

impl Marshalable for f64 {
    fn marshal_json_into(&self, s: &mut String) {
        s.push_str(&self.to_string())
    }
}
