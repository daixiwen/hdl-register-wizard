//! Tests for the tokenlist and generating vhdl identifiers

use crate::generate::tokenlist;
use std::default::Default;

#[test]
fn to_vhdl_token() {
    assert_eq!(tokenlist::to_vhdl_token("abcd"), "abcd");
    assert_eq!(tokenlist::to_vhdl_token("_pre"), "pre");
    assert_eq!(tokenlist::to_vhdl_token("double__underscore"), "double_underscore");
    assert_eq!(tokenlist::to_vhdl_token("post_"), "post");
    assert_eq!(tokenlist::to_vhdl_token("1_number_before"), "x1_number_before");
    assert_eq!(tokenlist::to_vhdl_token("_special  &% characters{"), "special_characters");
    assert_eq!(tokenlist::to_vhdl_token("üñîçòdé"), "unicode");
    assert_eq!(tokenlist::to_vhdl_token("smiley😀_icon"), "smiley_icon");    
}

#[test]
#[should_panic]
fn generate_vhdl_wrong_pattern() {
    let mut list : tokenlist::TokenList = Default::default();

    list.generate_token("abcd");
}

#[test]
fn generate_vhdl_token() {
    let mut list : tokenlist::TokenList = Default::default();

    assert_eq!(list.generate_token("abcd{}"), "abcd");
    assert_eq!(list.generate_token("abcd{}"), "abcd_2");
    assert_eq!(list.generate_token("_abcd_{}"), "abcd_3");
    assert_eq!(list.generate_token("{}signal"), "x2signal");
}
