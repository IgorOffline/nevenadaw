use crate::bosonoga::BosonogaBTreeParser;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn test_bosonoga_btree() {
    println!("test_bosonoga_btree");
    let input = r"
        SET first 10
        SET second 20
        BUL
        INAT
        TALI
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let mut expected_btree = BTreeMap::new();
    expected_btree.insert("first".to_string(), 10);
    expected_btree.insert("second".to_string(), 20);
    assert_eq!(btree, expected_btree);
}
