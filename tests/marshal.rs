use json::Element::*;
use std::collections::HashMap;

#[test]
fn marshal_int_test() {
    assert_eq!("0", JsonInt(0).marshal());
}

#[test]
fn marshal_float_half_test() {
    assert_eq!("0.5", JsonFloat(0.5).marshal());
}

#[test]
fn marshal_float_one_third_test() {
    assert_eq!("0.3333333333333333", JsonFloat(1.0 / 3.0).marshal());
}

#[test]
fn marshal_bool_test() {
    assert_eq!("true", JsonBool(true).marshal());
    assert_eq!("false", JsonBool(false).marshal());
}

#[test]
fn marshal_null_test() {
    assert_eq!("null", JsonNull.marshal());
}

#[test]
fn marshal_empty_string_test() {
    assert_eq!(r#""""#.to_owned(), JsonString(String::new()).marshal());
}

#[test]
fn marshal_ascii_string_test() {
    assert_eq!(
        r#""test""#.to_owned(),
        JsonString("test".to_owned()).marshal()
    );
}

#[test]
fn marshal_quoted_string_test() {
    assert_eq!(
        r#""\"test\"""#.to_owned(),
        JsonString("\"test\"".to_owned()).marshal()
    );
}

#[test]
fn marshal_empty_object_test() {
    assert_eq!("{}", JsonObject(HashMap::new()).marshal());
}

#[test]
fn marshal_single_int_object_test() {
    assert_eq!(
        r#"{"a": 0}"#,
        JsonObject(HashMap::from([("a".to_owned(), JsonInt(0))])).marshal()
    );
}

#[test]
fn marshal_int_object_test() {
    // The order in which map keys are serialized is non-deterministic
    let valid = [
        r#"{"a": 0, "b": 1}"#.to_owned(),
        r#"{"b": 1, "a": 0}"#.to_owned(),
    ];
    let marshaled = JsonObject(HashMap::from([
        ("a".to_owned(), JsonInt(0)),
        ("b".to_owned(), JsonInt(1)),
    ]))
    .marshal();
    assert!(
        valid.contains(&marshaled),
        "assertion failed: valid serializations\n\t`{}`\ndo not contain serialization\n\t`{}`",
        valid.join("`\n\t`"),
        &marshaled
    );
}

#[test]
fn marshal_empty_list_test() {
    assert_eq!("[]", JsonList(vec![]).marshal());
}

#[test]
fn marshal_int_list_test() {
    assert_eq!(
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]",
        JsonList((1..=10).map(JsonInt).collect()).marshal()
    );
}

#[test]
fn marshal_str_list_test() {
    assert_eq!(
        r#"["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]"#,
        JsonList((1..=10).map(|n| n.to_string()).map(JsonString).collect()).marshal()
    );
}
