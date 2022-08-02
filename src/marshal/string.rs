use crate::marshal::Marshalable;

impl Marshalable for String {
    fn marshal_json_into(&self, s: &mut String) {
        s.push('"');
        for c in self.chars() {
            match c {
                needs_escape @ ('\\' | '"' | '/') => {
                    s.push('\\');
                    s.push(needs_escape);
                }
                '\x08' => s.push_str("\\b"), // Literal backspace
                '\x0c' => s.push_str("\\f"), // Formfeed
                '\n' => s.push_str("\\n"),
                '\r' => s.push_str("\\r"),
                '\t' => s.push_str("\\t"),
                _ if !c.is_ascii() => {
                    // Escape unicode value
                    let code_point = c as u32;
                    if code_point < 0xFFFF {
                        // No surrogates necessary
                        s.push_str(&format!("\\u{:04x}", code_point));
                    } else {
                        // https://datacadamia.com/data/type/text/surrogate#from_character_code_to_surrogate_pair1
                        const LEAD_OFFSET: u32 = 0xD800 - (0x10000 >> 10);
                        let lead = LEAD_OFFSET + (code_point >> 10);
                        let trail = 0xDC00 + (code_point & 0x3FF);

                        s.push_str(&format!("\\u{:04x}\\u{:04x}", lead, trail));
                    }
                }
                _ => s.push(c),
            };
        }
        s.push('"')
    }
}
