use std::collections::HashSet;

use crate::marshal::Marshalable;

pub fn marshal_list<'a, T, U>(mut iter: T, s: &mut String)
where
    T: Iterator<Item = &'a U>,
    U: Marshalable + 'a,
{
    s.push('[');
    if let Some(e) = iter.next() {
        e.marshal_json_into(s);
        for e in iter {
            s.push_str(", ");
            e.marshal_json_into(s);
        }
    }
    s.push(']');
}

impl<T> Marshalable for &[T]
where
    T: Marshalable,
{
    fn marshal_json_into(&self, s: &mut String) {
        marshal_list(self.iter(), s)
    }
}

impl<T> Marshalable for Vec<T>
where
    T: Marshalable,
{
    fn marshal_json_into(&self, s: &mut String) {
        marshal_list(self.iter(), s)
    }
}

impl<T> Marshalable for HashSet<T>
where
    T: Marshalable,
{
    fn marshal_json_into(&self, s: &mut String) {
        marshal_list(self.iter(), s)
    }
}
