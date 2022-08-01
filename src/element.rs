use std::collections::HashMap;

#[derive(Debug, PartialEq)]
/// An enum describing an unknown JSON element for unmarshaling JSON where the
/// layout is not known ahead of time.
pub enum Element {
    JsonObject(HashMap<String, Element>),
    JsonList(Vec<Element>),
    JsonString(String),
    JsonInt(i64),
    JsonFloat(f64),
    JsonBool(bool),
    JsonNull,
}
