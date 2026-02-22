use crate::ast::{new_variable_i32, BosonogaVariable};
use crate::bosonoga::BosonogaBTreeParser;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn test_bosonoga_init() {
    println!("test_bosonoga_init");
    let input = r"
        SET first 10
        SET second 20
        BUL
        INAT
        TALI
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let first_var = new_variable_i32("first".to_string(), 10);
    let second_var = new_variable_i32("second".to_string(), 20);
    let mut expected_btree: BTreeMap<String, BosonogaVariable> = BTreeMap::new();
    expected_btree.insert("first".to_string(), first_var);
    expected_btree.insert("second".to_string(), second_var);
    assert_eq!(btree, expected_btree);
}

#[test]
fn test_bosonoga_multiset() {
    println!("test_bosonoga_multiset");
    let input = r"
        SET first 10
        SET second 20
        SET first 30
        SET second 40
        BUL
        INAT
        TALI
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let first_var = new_variable_i32("first".to_string(), 30);
    let second_var = new_variable_i32("second".to_string(), 40);
    let mut expected_btree: BTreeMap<String, BosonogaVariable> = BTreeMap::new();
    expected_btree.insert("first".to_string(), first_var);
    expected_btree.insert("second".to_string(), second_var);
    assert_eq!(btree, expected_btree);
}
