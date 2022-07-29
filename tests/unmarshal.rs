use std::collections::HashMap;
use std::str::Chars;

use json::Element;
use json::Element::*;

fn assert_unmarshal(c: Chars, expect: &Result<Element, String>) {
    match (&Element::unmarshal(c), expect) {
        // Use epsilon for floats
        (Ok(JsonFloat(got_float)), Ok(JsonFloat(expect_float))) => assert!(
            got_float - expect_float <= f64::EPSILON,
            "assertion failed: `(left ~= right)`
        left: `{}`,
       right: `{}`",
            got_float,
            expect_float
        ),
        (Ok(got), Ok(e)) => assert_eq!(got, e),
        (Err(got), Err(e)) => assert_eq!(got, e),
        (got, e) => panic!("expected \n\t`{:?}`\ngot\n\t`{:?}`", e, got),
    }
}

#[test]
fn unmarshal_int_test() {
    assert_unmarshal("0".chars(), &Ok(JsonInt(0)))
}

#[test]
fn unmarshal_negative_int_test() {
    assert_unmarshal("-1".chars(), &Ok(JsonInt(-1)))
}

#[test]
fn unmarshal_float_half_test() {
    assert_unmarshal("0.5".chars(), &Ok(JsonFloat(0.5)))
}

#[test]
fn unmarshal_float_negative_half_test() {
    assert_unmarshal("-0.5".chars(), &Ok(JsonFloat(-0.5)))
}

#[test]
fn unmarshal_float_one_third_test() {
    assert_unmarshal("0.3333333333333333".chars(), &Ok(JsonFloat(1.0 / 3.0)))
}

#[test]
fn unmarshal_float_exponent_test() {
    assert_unmarshal("3e3".chars(), &Ok(JsonFloat(3000.0)));
    assert_unmarshal("3.0e3".chars(), &Ok(JsonFloat(3000.0)))
}

#[test]
fn unmarshal_float_negative_exponent_test() {
    assert_unmarshal("3e-3".chars(), &Ok(JsonFloat(0.003)));
    assert_unmarshal("3.0e-3".chars(), &Ok(JsonFloat(0.003)))
}

#[test]
fn unmarshal_bool_test() {
    assert_unmarshal("true".chars(), &Ok(JsonBool(true)));
    assert_unmarshal("false".chars(), &Ok(JsonBool(false)));
}

#[test]
fn unmarshal_null_test() {
    assert_unmarshal("null".chars(), &Ok(JsonNull))
}

#[test]
fn unmarshal_empty_string_test() {
    assert_unmarshal(r#""""#.chars(), &Ok(JsonString("".to_owned())))
}

#[test]
fn unmarshal_ascii_string_test() {
    assert_unmarshal(r#""test""#.chars(), &Ok(JsonString("test".to_owned())))
}

#[test]
fn unmarshal_unicode_string_test() {
    assert_unmarshal(r#""\u2764""#.chars(), &Ok(JsonString("❤".to_owned())));
    assert_unmarshal(
        r#""\ud83e\udd80""#.chars(),
        &Ok(JsonString("🦀".to_owned())),
    )
}

#[test]
#[should_panic]
fn unmarshal_invalid_unicode_string_test() {
    json::Element::unmarshal(r#""\ud83e""#.chars()).unwrap();
}

#[test]
fn unmarshal_quoted_string_test() {
    assert_unmarshal(
        r#""\"test\"""#.chars(),
        &Ok(JsonString("\"test\"".to_owned())),
    )
}

#[test]
fn unmarshal_empty_object_test() {
    assert_unmarshal("{}".chars(), &Ok(JsonObject(HashMap::new())));
}

#[test]
fn unmarshal_single_int_object_test() {
    assert_unmarshal(
        r#"{"a": 0}"#.chars(),
        &Ok(JsonObject(HashMap::from([("a".to_owned(), JsonInt(0))]))),
    );
}

#[test]
fn unmarshal_int_object_test() {
    let expect = Ok(JsonObject(HashMap::from([
        ("a".to_owned(), JsonInt(0)),
        ("b".to_owned(), JsonInt(1)),
    ])));
    for c in [r#"{"a": 0, "b": 1}"#.chars(), r#"{"b": 1, "a": 0}"#.chars()] {
        assert_unmarshal(c, &expect);
    }
}

#[test]
fn unmarshal_unicode_key_object_test() {
    assert_unmarshal(
        r#"{"\ud83e\udd80": 0}"#.chars(),
        &Ok(JsonObject(HashMap::from([("🦀".to_owned(), JsonInt(0))]))),
    );
}

#[test]
fn unmarshal_empty_list_test() {
    assert_unmarshal("[]".chars(), &Ok(JsonList(vec![])));
}

#[test]
fn unmarshal_int_list_test() {
    assert_unmarshal(
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]".chars(),
        &Ok(JsonList((1..=10).map(JsonInt).collect())),
    );
}

#[test]
fn unmarshal_float_list_test() {
    assert_unmarshal(
        "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]".chars(),
        &Ok(JsonList(
            (1..=10).map(|n| n as f64).map(JsonFloat).collect(),
        )),
    );
}

#[test]
fn unmarshal_str_list_test() {
    assert_unmarshal(
        r#"["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]"#.chars(),
        &Ok(JsonList(
            (1..=10).map(|n| n.to_string()).map(JsonString).collect(),
        )),
    );
}

#[test]
fn unmarshal_mixed_list_test() {
    assert_unmarshal(
        r#"[1, 2.0, null, true, false]"#.chars(),
        &Ok(JsonList(vec![
            JsonInt(1),
            JsonFloat(2.0),
            JsonNull,
            JsonBool(true),
            JsonBool(false),
        ])),
    );
}
