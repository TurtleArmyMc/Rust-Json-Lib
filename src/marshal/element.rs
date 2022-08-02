use crate::{Element, Element::*, Marshalable};

impl Marshalable for Element {
    fn marshal_json_into(&self, s: &mut String) {
        match self {
            JsonObject(e) => e.marshal_json_into(s),
            JsonList(e) => e.marshal_json_into(s),
            JsonString(e) => e.marshal_json_into(s),
            JsonInt(e) => e.marshal_json_into(s),
            JsonFloat(e) => e.marshal_json_into(s),
            JsonBool(e) => e.marshal_json_into(s),
            JsonNull => s.push_str("null"),
        }
    }
}
