use crate::ast::BosonogaVariable;
use crate::bosonoga::BosonogaBTreeParser;
use pretty_assertions::assert_eq;
use std::collections::BTreeSet;

#[test]
fn test_bosonoga() {
    println!("test_bosonoga");
    let input = r"
        SET INAT first 10
        SET INAT second 20
        SET INAT first 30
        SET INAT second 40
        SET BUL third false
        SET BUL third true
        TALI
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let first_var = BosonogaVariable::new_i32("first".to_string(), 30);
    let second_var = BosonogaVariable::new_i32("second".to_string(), 40);
    let third_var = BosonogaVariable::new_bool("third".to_string(), true);
    let mut expected_btree: BTreeSet<BosonogaVariable> = BTreeSet::new();
    expected_btree.insert(first_var);
    expected_btree.insert(second_var);
    expected_btree.insert(third_var);
    assert_eq!(btree, expected_btree);
}
