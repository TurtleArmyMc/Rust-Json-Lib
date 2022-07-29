use crate::Element;

use std::{iter, str::Chars};

impl Element {
    pub fn marshal(&self) -> String {
        match self {
            Self::JsonObject(dict) => iter::once("{".to_owned())
                .chain(
                    dict.iter()
                        .map(|e| format!("{}: {}", escape_and_quote(e.0.chars()), e.1.marshal()))
                        .intersperse(", ".to_owned()),
                )
                .chain(iter::once("}".to_owned()))
                .collect(),
            Self::JsonList(list) => iter::once("[".to_owned())
                .chain(
                    list.iter()
                        .map(|e| e.marshal())
                        .intersperse(", ".to_owned()),
                )
                .chain(iter::once("]".to_owned()))
                .collect(),
            Self::JsonString(s) => escape_and_quote(s.chars()),
            Self::JsonInt(i) => i.to_string(),
            Self::JsonFloat(f) => f.to_string(),
            Self::JsonBool(b) => b.to_string(),
            Self::JsonNull => "null".to_owned(),
        }
    }
}

fn escape_and_quote(chars: Chars) -> String {
    let mut out = "\"".to_owned();
    // Note: Escape unicode chars
    for c in chars {
        match c {
            needs_escape @ ('\\' | '"' | '/') => {
                out.push('\\');
                out.push(needs_escape);
            }
            '\x08' => out.push_str("\\b"), // Literal backspace
            '\x0c' => out.push_str("\\f"), // Formfeed
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ if !c.is_ascii() => {
                // Escape unicode value
                let code_point = c as u32;
                if code_point < 0xFFFF {
                    // No surrogates necessary
                    out += &format!("\\u{:04x}", code_point);
                } else {
                    // https://datacadamia.com/data/type/text/surrogate#from_character_code_to_surrogate_pair1
                    const LEAD_OFFSET: u32 = 0xD800 - (0x10000 >> 10);
                    let lead = LEAD_OFFSET + (code_point >> 10);
                    let trail = 0xDC00 + (code_point & 0x3FF);

                    out += &format!("\\u{:04x}\\u{:04x}", lead, trail);
                }
            }
            _ => out.push(c),
        };
    }
    out + "\""
}
