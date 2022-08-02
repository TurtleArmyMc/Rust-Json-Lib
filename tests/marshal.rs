use std::collections::HashMap;

use json::{Element::*, Marshalable};

#[test]
fn marshal_int_test() {
    assert_eq!(0.marshal_json(), "0");
    assert_eq!(JsonInt(0).marshal_json(), "0");
}

#[test]
fn marshal_negative_int_test() {
    assert_eq!((-1).marshal_json(), "-1");
    assert_eq!((-15).marshal_json(), "-15");
}

#[test]
fn marshal_float_half_test() {
    assert_eq!(0.5.marshal_json(), "0.5");
    assert_eq!(0.51.marshal_json(), "0.51");

    assert_eq!(JsonFloat(0.5).marshal_json(), "0.5");
    assert_eq!(JsonFloat(0.51).marshal_json(), "0.51");
}

#[test]
fn marshal_float_negative_half_test() {
    assert_eq!((-0.5).marshal_json(), "-0.5");
    assert_eq!((-0.51).marshal_json(), "-0.51");
}

#[test]
fn marshal_float_one_third_test() {
    assert_eq!((1.0 / 3.0).marshal_json(), "0.3333333333333333");
}

#[test]
fn marshal_bool_test() {
    assert_eq!(true.marshal_json(), "true");
    assert_eq!(false.marshal_json(), "false");

    assert_eq!(JsonBool(true).marshal_json(), "true");
    assert_eq!(JsonBool(false).marshal_json(), "false");
}

#[test]
fn marshal_null_test() {
    assert_eq!(Option::<HashMap<String, i64>>::None.marshal_json(), "null");
    assert_eq!(Option::<Vec<i64>>::None.marshal_json(), "null");
    assert_eq!(Option::<String>::None.marshal_json(), "null");
    assert_eq!(Option::<bool>::None.marshal_json(), "null");
    assert_eq!(Option::<i64>::None.marshal_json(), "null");
    assert_eq!(Option::<f64>::None.marshal_json(), "null");
    assert_eq!(Option::<()>::None.marshal_json(), "null");

    assert_eq!(JsonNull.marshal_json(), "null");
}

#[test]
fn marshal_option_some_test() {
    assert_eq!(
        Option::<HashMap<String, i64>>::Some(HashMap::new()).marshal_json(),
        "{}"
    );
    assert_eq!(Option::<Vec<i64>>::Some(Vec::new()).marshal_json(), "[]");
    assert_eq!(
        Option::<String>::Some(String::new()).marshal_json(),
        r#""""#
    );
    assert_eq!(Option::<bool>::Some(false).marshal_json(), "false");
    assert_eq!(Option::<i64>::Some(0).marshal_json(), "0");
    assert_eq!(Option::<f64>::Some(0.0).marshal_json(), "0");
    assert_eq!(Option::<()>::Some(()).marshal_json(), "null");
}

#[test]
fn marshal_empty_string_test() {
    assert_eq!("".to_owned().marshal_json(), r#""""#);

    assert_eq!(JsonString("".to_owned()).marshal_json(), r#""""#);
}

#[test]
fn marshal_ascii_string_test() {
    assert_eq!("test".to_owned().marshal_json(), r#""test""#);

    assert_eq!(JsonString("test".to_owned()).marshal_json(), r#""test""#);
}

#[test]
fn marshal_unicode_string_test() {
    assert_eq!("❤".to_owned().marshal_json(), r#""\u2764""#,);
    assert_eq!("🦀".to_owned().marshal_json(), r#""\ud83e\udd80""#,)
}

#[test]
fn marshal_quoted_string_test() {
    assert_eq!("\"test\"".to_owned().marshal_json(), r#""\"test\"""#);
}

#[test]
fn marshal_empty_object_test() {
    assert_eq!(
        HashMap::<String, HashMap<String, i64>>::new().marshal_json(),
        "{}"
    );
    assert_eq!(HashMap::<String, Vec<i64>>::new().marshal_json(), "{}");
    assert_eq!(HashMap::<String, String>::new().marshal_json(), "{}");
    assert_eq!(HashMap::<String, bool>::new().marshal_json(), "{}");
    assert_eq!(HashMap::<String, i64>::new().marshal_json(), "{}");
    assert_eq!(HashMap::<String, f64>::new().marshal_json(), "{}");
    assert_eq!(HashMap::<String, Option<i64>>::new().marshal_json(), "{}");

    assert_eq!(JsonObject(HashMap::new()).marshal_json(), "{}");
}

#[test]
fn marshal_single_int_object_test() {
    assert_eq!(
        HashMap::<String, i64>::from([("a".to_owned(), 0)]).marshal_json(),
        r#"{"a": 0}"#
    );
}

#[test]
fn marshal_int_object_test() {
    let expect = [r#"{"a": 0, "b": 1}"#, r#"{"b": 1, "a": 0}"#];
    assert!(expect.contains(
        &HashMap::from([("a".to_owned(), 0), ("b".to_owned(), 1)])
            .marshal_json()
            .as_str()
    ))
}

#[test]
fn marshal_unicode_key_object_test() {
    assert_eq!(
        HashMap::from([("🦀".to_owned(), 0)]).marshal_json(),
        r#"{"\ud83e\udd80": 0}"#,
    );
}

#[test]
fn marshal_empty_list_test() {
    assert_eq!(Vec::<HashMap<String, i64>>::new().marshal_json(), "[]");
    assert_eq!(Vec::<Vec<i64>>::new().marshal_json(), "[]");
    assert_eq!(Vec::<String>::new().marshal_json(), "[]");
    assert_eq!(Vec::<bool>::new().marshal_json(), "[]");
    assert_eq!(Vec::<i64>::new().marshal_json(), "[]");
    assert_eq!(Vec::<f64>::new().marshal_json(), "[]");

    assert_eq!(JsonList(Vec::new()).marshal_json(), "[]");
}

#[test]
fn marshal_int_list_test() {
    assert_eq!(
        (1..=10).collect::<Vec<i64>>().marshal_json(),
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
    );
}

#[test]
fn marshal_float_list_test() {
    assert_eq!(
        (1..=10)
            .map(|i| i as f64)
            .collect::<Vec<f64>>()
            .marshal_json(),
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
    );
}

#[test]
fn marshal_float_half_list_test() {
    assert_eq!(
        (1..=10)
            .map(|i| i as f64 + 0.5)
            .collect::<Vec<f64>>()
            .marshal_json(),
        "[1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5]"
    );
}

#[test]
fn marshal_str_list_test() {
    assert_eq!(
        (1..=10)
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .marshal_json(),
        r#"["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]"#
    );
}

#[test]
fn marshal_optional_int_list_test() {
    assert_eq!(vec![Some(1), None, Some(3)].marshal_json(), "[1, null, 3]");
}

#[test]
fn marshal_mixed_list_element_test() {
    assert_eq!(
        JsonList(vec![
            JsonObject(HashMap::new()),
            JsonList(Vec::new()),
            JsonString(String::new()),
            JsonInt(1),
            JsonFloat(2.5),
            JsonBool(true),
            JsonBool(false),
            JsonNull,
        ])
        .marshal_json(),
        r#"[{}, [], "", 1, 2.5, true, false, null]"#
    );
}
