//! Tests for the conversions from and to Option<Vec<String>>

use super::super::utils;

/// test creating Vec<String> with string
#[test]
fn from_str() {
    let expected = vec!["first line", "second line", "third line"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    //let opt_expected : Option<Vec<String>> = Some(expected);

    assert_eq!(
        Some(expected),
        utils::textarea_to_opt_vec_str(&"first line\nsecond line\nthird line".to_string())
    );

    assert_eq!(None, utils::textarea_to_opt_vec_str(&"".to_string()));
}

/// test converting Vec<String> to string
#[test]
fn to_str() {
    let value: Vec<String> = vec!["line one", "line two", "line three"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    assert_eq!("".to_string(), utils::opt_vec_str_to_textarea(&None));
    assert_eq!(
        "line one".to_string(),
        utils::opt_vec_str_to_textarea(&Some(vec![value[0].clone()]))
    );
    assert_eq!(
        "line one\nline two\nline three".to_string(),
        utils::opt_vec_str_to_textarea(&Some(value))
    );
}
