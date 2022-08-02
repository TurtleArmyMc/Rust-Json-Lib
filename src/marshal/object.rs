use std::collections::HashMap;

use crate::marshal::marshalable::Marshalable;

impl<T> Marshalable for HashMap<String, T>
where
    T: Marshalable,
{
    fn marshal_json_into(&self, s: &mut String) {
        s.push('{');
        let mut iter = self.iter();
        if let Some((k, e)) = iter.next() {
            k.marshal_json_into(s);
            s.push_str(": ");
            e.marshal_json_into(s);
            for (k, e) in iter {
                s.push_str(", ");
                k.marshal_json_into(s);
                s.push_str(": ");
                e.marshal_json_into(s);
            }
        }
        s.push('}');
    }
}
