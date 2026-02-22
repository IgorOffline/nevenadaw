use crate::ast::BosonogaVariable;
use crate::bosonoga::BosonogaBTreeParser;
use pretty_assertions::assert_eq;
use std::collections::BTreeSet;

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
    let first_var = BosonogaVariable::new_i32("first".to_string(), 10);
    let second_var = BosonogaVariable::new_i32("second".to_string(), 20);
    let mut expected_btree: BTreeSet<BosonogaVariable> = BTreeSet::new();
    expected_btree.insert(first_var);
    expected_btree.insert(second_var);
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
    let first_var = BosonogaVariable::new_i32("first".to_string(), 30);
    let second_var = BosonogaVariable::new_i32("second".to_string(), 40);
    let mut expected_btree: BTreeSet<BosonogaVariable> = BTreeSet::new();
    expected_btree.insert(first_var);
    expected_btree.insert(second_var);
    assert_eq!(btree, expected_btree);
}
