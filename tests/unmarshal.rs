use std::collections::HashMap;

use json::{Element, Element::*, UnmarshalError, Unmarshalable};

#[test]
fn unmarshal_int_test() {
    assert_eq!(i64::unmarshal_json("0".chars()).unwrap(), 0);
    assert_eq!(Element::unmarshal_json("0".chars()).unwrap(), JsonInt(0));
}

#[test]
fn unmarshal_negative_int_test() {
    assert_eq!(i64::unmarshal_json("-1".chars()).unwrap(), -1);
    assert_eq!(i64::unmarshal_json("-15".chars()).unwrap(), -15);
}

#[test]
fn unmarshal_float_half_test() {
    assert_eq!(f64::unmarshal_json("0.5".chars()).unwrap(), 0.5);
    assert_eq!(f64::unmarshal_json("00.51".chars()).unwrap(), 0.51);

    assert_eq!(
        Element::unmarshal_json("0.5".chars()).unwrap(),
        JsonFloat(0.5)
    );
    assert_eq!(
        Element::unmarshal_json("00.51".chars()).unwrap(),
        JsonFloat(0.51)
    );
}

#[test]
fn unmarshal_float_negative_half_test() {
    assert_eq!(f64::unmarshal_json("-0.5".chars()).unwrap(), -0.5);
    assert_eq!(f64::unmarshal_json("-00.51".chars()).unwrap(), -0.51);
}

#[test]
fn unmarshal_float_one_third_test() {
    assert_eq!(
        f64::unmarshal_json("0.3333333333333333".chars()).unwrap(),
        1.0 / 3.0
    );
}

#[test]
fn unmarshal_float_exponent_test() {
    assert_eq!(f64::unmarshal_json("3e3".chars()).unwrap(), 3000.0);
    assert_eq!(f64::unmarshal_json("3.0e3".chars()).unwrap(), 3000.0);
    assert_eq!(f64::unmarshal_json("3E3".chars()).unwrap(), 3000.0);
    assert_eq!(f64::unmarshal_json("3.0E3".chars()).unwrap(), 3000.0);
}

#[test]
fn unmarshal_float_negative_exponent_test() {
    assert_eq!(f64::unmarshal_json("3e-3".chars()).unwrap(), 0.003);
    assert_eq!(f64::unmarshal_json("3.0e-3".chars()).unwrap(), 0.003);
    assert_eq!(f64::unmarshal_json("3E-3".chars()).unwrap(), 0.003);
    assert_eq!(f64::unmarshal_json("3.0E-3".chars()).unwrap(), 0.003);
}

#[test]
fn unmarshal_float_leading_decimal_test() {
    assert_eq!(
        f64::unmarshal_json(".3".chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: '.',
            row: 1,
            col: 1
        }
    );
    assert_eq!(
        f64::unmarshal_json("-.3".chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: '.',
            row: 1,
            col: 2
        }
    );
}

#[test]
fn unmarshal_float_trailing_decimal_test() {
    assert_eq!(
        f64::unmarshal_json("3.".chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
}

#[test]
fn unmarshal_float_trailing_exponent_test() {
    assert_eq!(
        f64::unmarshal_json("3e".chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
    assert_eq!(
        f64::unmarshal_json("3.0e".chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
    assert_eq!(
        f64::unmarshal_json("3E".chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
    assert_eq!(
        f64::unmarshal_json("3.0E".chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
}

#[test]
fn unmarshal_bool_test() {
    assert_eq!(bool::unmarshal_json("true".chars()).unwrap(), true);
    assert_eq!(bool::unmarshal_json("false".chars()).unwrap(), false);

    assert_eq!(
        Element::unmarshal_json("true".chars()).unwrap(),
        JsonBool(true)
    );
    assert_eq!(
        Element::unmarshal_json("false".chars()).unwrap(),
        JsonBool(false)
    );
}

#[test]
fn unmarshal_null_test() {
    assert_eq!(
        Option::<HashMap<String, i64>>::unmarshal_json("null".chars()).unwrap(),
        None
    );
    assert_eq!(
        Option::<Vec<i64>>::unmarshal_json("null".chars()).unwrap(),
        None
    );
    assert_eq!(
        Option::<String>::unmarshal_json("null".chars()).unwrap(),
        None
    );
    assert_eq!(
        Option::<bool>::unmarshal_json("null".chars()).unwrap(),
        None
    );
    assert_eq!(Option::<i64>::unmarshal_json("null".chars()).unwrap(), None);
    assert_eq!(Option::<f64>::unmarshal_json("null".chars()).unwrap(), None);
    assert_eq!(Option::<()>::unmarshal_json("null".chars()).unwrap(), None);

    assert_eq!(Element::unmarshal_json("null".chars()).unwrap(), JsonNull);
}

#[test]
fn unmarshal_empty_string_test() {
    assert_eq!(
        String::unmarshal_json(r#""""#.chars()).unwrap(),
        "".to_owned()
    );

    assert_eq!(
        Element::unmarshal_json(r#""""#.chars()).unwrap(),
        JsonString("".to_owned())
    );
}

#[test]
fn unmarshal_ascii_string_test() {
    assert_eq!(
        String::unmarshal_json(r#""test""#.chars()).unwrap(),
        "test".to_owned()
    );

    assert_eq!(
        Element::unmarshal_json(r#""test""#.chars()).unwrap(),
        JsonString("test".to_owned())
    );
}

#[test]
fn unmarshal_unicode_string_test() {
    assert_eq!(
        String::unmarshal_json(r#""\u2764""#.chars()).unwrap(),
        "❤".to_owned()
    );
    assert_eq!(
        String::unmarshal_json(r#""\ud83e\udd80""#.chars()).unwrap(),
        "🦀".to_owned()
    )
}

#[test]
#[should_panic]
fn unmarshal_invalid_unicode_string_test() {
    String::unmarshal_json(r#""\ud83e""#.chars()).unwrap();
}

#[test]
fn unmarshal_quoted_string_test() {
    assert_eq!(
        String::unmarshal_json(r#""\"test\"""#.chars()).unwrap(),
        "\"test\"".to_owned()
    );
}

#[test]
fn unmarshal_unterminated_string_test() {
    assert_eq!(
        String::unmarshal_json(r#""test"#.chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
}

#[test]
fn unmarshal_empty_object_test() {
    assert_eq!(
        HashMap::<String, HashMap<String, i64>>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, Vec<i64>>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, String>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, bool>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, f64>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );
    assert_eq!(
        HashMap::<String, Option<i64>>::unmarshal_json("{}".chars()).unwrap(),
        HashMap::new()
    );

    assert_eq!(
        Element::unmarshal_json("{}".chars()).unwrap(),
        JsonObject(HashMap::new())
    );
}

#[test]
fn unmarshal_single_int_object_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a": 0}"#.chars()).unwrap(),
        HashMap::<String, i64>::from([("a".to_owned(), 0)]),
    );
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a":0}"#.chars()).unwrap(),
        HashMap::<String, i64>::from([("a".to_owned(), 0)]),
    );
}

#[test]
fn unmarshal_single_overriden_int_object_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a": 0, "a": 1}"#.chars()).unwrap(),
        HashMap::<String, i64>::from([("a".to_owned(), 1)]),
    );
}

#[test]
fn unmarshal_int_object_test() {
    let expect = HashMap::from([("a".to_owned(), 0), ("b".to_owned(), 1)]);
    for c in [r#"{"a": 0, "b": 1}"#.chars(), r#"{"b": 1, "a": 0}"#.chars()] {
        assert_eq!(HashMap::<String, i64>::unmarshal_json(c).unwrap(), expect);
    }
}

#[test]
fn unmarshal_unicode_key_object_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"\ud83e\udd80": 0}"#.chars()).unwrap(),
        HashMap::from([("🦀".to_owned(), 0)])
    );
}

#[test]
fn unmarshal_unquoted_key_object_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{a: 0}"#.chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: 'a',
            row: 1,
            col: 2
        }
    );
}

#[test]
fn unmarshal_unterminated_object_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"#.chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a": 0"#.chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a": 0,"#.chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
}

#[test]
fn unmarshal_object_leading_comma_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{,}"#.chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: ',',
            row: 1,
            col: 2
        }
    );
}

#[test]
fn unmarshal_object_trailing_comma_test() {
    assert_eq!(
        HashMap::<String, i64>::unmarshal_json(r#"{"a": 0,}"#.chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: '}',
            row: 1,
            col: 9
        }
    );
}

#[test]
fn unmarshal_empty_list_test() {
    assert_eq!(
        Vec::<HashMap<String, i64>>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<HashMap<String, i64>>::new()
    );
    assert_eq!(
        Vec::<Vec<i64>>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<Vec<i64>>::new()
    );
    assert_eq!(
        Vec::<String>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(
        Vec::<bool>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<bool>::new()
    );
    assert_eq!(
        Vec::<i64>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<i64>::new()
    );
    assert_eq!(
        Vec::<f64>::unmarshal_json("[]".chars()).unwrap(),
        Vec::<f64>::new()
    );

    assert_eq!(
        Element::unmarshal_json("[]".chars()).unwrap(),
        JsonList(Vec::new())
    );
}

#[test]
fn unmarshal_int_list_test() {
    assert_eq!(
        Vec::<i64>::unmarshal_json("[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]".chars()).unwrap(),
        (1..=10).collect::<Vec<_>>()
    );
}

#[test]
fn unmarshal_float_list_test() {
    assert_eq!(
        Vec::<f64>::unmarshal_json("[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]".chars())
            .unwrap(),
        (1..=10).map(|n| n as f64).collect::<Vec<_>>()
    );
}

#[test]
fn unmarshal_str_list_test() {
    assert_eq!(
        Vec::<String>::unmarshal_json(
            r#"["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]"#.chars()
        )
        .unwrap(),
        (1..=10).map(|n| n.to_string()).collect::<Vec<_>>()
    );
}

#[test]
fn unmarshal_optional_int_list_test() {
    assert_eq!(
        Vec::<Option<i64>>::unmarshal_json("[1, null, 3]".chars()).unwrap(),
        vec![Some(1), None, Some(3)]
    );
}

#[test]
fn unmarshal_mixed_list_element_test() {
    assert_eq!(
        Element::unmarshal_json(r#"[{}, [], "", 1, 2.0, true, false, null]"#.chars()).unwrap(),
        JsonList(vec![
            JsonObject(HashMap::new()),
            JsonList(Vec::new()),
            JsonString(String::new()),
            JsonInt(1),
            JsonFloat(2.0),
            JsonBool(true),
            JsonBool(false),
            JsonNull,
        ])
    );
}

#[test]
fn unmarshal_unterminated_list_test() {
    assert_eq!(
        Vec::<Option<()>>::unmarshal_json(r#"["#.chars()).unwrap_err(),
        UnmarshalError::EndOfChars
    );
}

#[test]
fn unmarshal_trailing_comma_list_test() {
    assert_eq!(
        Vec::<i64>::unmarshal_json(r#"[1,]"#.chars()).unwrap_err(),
        UnmarshalError::UnexpectedChar {
            c: ']',
            row: 1,
            col: 4
        }
    );
}
