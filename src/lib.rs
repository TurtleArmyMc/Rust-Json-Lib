#![feature(iter_intersperse)]

pub mod marshal;
pub mod unmarshal;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Element {
    JsonObject(HashMap<String, Element>),
    JsonList(Vec<Element>),
    JsonString(String),
    JsonInt(i64),
    JsonFloat(f64),
    JsonBool(bool),
    JsonNull,
}
