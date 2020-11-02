use super::super::utils;
use seed::{prelude::*, *};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// test creating VectorValue with string
#[wasm_bindgen_test]
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

// test converting VectorValue to string
#[wasm_bindgen_test]
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
