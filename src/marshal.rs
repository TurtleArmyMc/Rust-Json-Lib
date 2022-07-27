use std::iter;

use super::Element;

fn escape_and_quote(s: &String) -> String {
    format!("\"{}\"", s.escape_default())
}

impl Element {
    pub fn marshal(&self) -> String {
        match self {
            Self::JsonObject(dict) => iter::once("{".to_owned())
                .chain(
                    dict.iter()
                        .map(|e| format!("{}: {}", escape_and_quote(e.0), e.1.marshal()))
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
            Self::JsonString(s) => escape_and_quote(s),
            Self::JsonInt(i) => i.to_string(),
            Self::JsonFloat(f) => f.to_string(),
            Self::JsonBool(b) => b.to_string(),
            Self::JsonNull => "null".to_owned(),
        }
    }
}
