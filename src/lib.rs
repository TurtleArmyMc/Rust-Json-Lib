#![feature(iter_intersperse)]

pub mod marshal;
use std::collections::HashMap;

pub enum Element {
    JsonObject(HashMap<String, Element>),
    JsonList(Vec<Element>),
    JsonString(String),
    JsonInt(i64),
    JsonFloat(f64),
    JsonBool(bool),
    JsonNull,
}
