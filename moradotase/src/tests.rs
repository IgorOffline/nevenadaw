use crate::ast::{BosonogaCommand, BosonogaElement, BosonogaType};
use crate::bosonoga::{BosonogaBTreeParser, BosonogaParser};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn test_bosonoga() {
    println!("test_bosonoga");
    let input = r"
        FUNAK first
        BUL
        TALI second
        INAT
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga.len(), 4);
    assert_eq!(
        bosonoga[0],
        BosonogaElement::Command(BosonogaCommand::Funak("first".to_string()))
    );
    assert_eq!(bosonoga[1], BosonogaElement::Type(BosonogaType::Bul));
    assert_eq!(
        bosonoga[2],
        BosonogaElement::Command(BosonogaCommand::Tali("second".to_string()))
    );
    assert_eq!(bosonoga[3], BosonogaElement::Type(BosonogaType::Inat));
}

#[test]
fn test_bosonoga_btree() {
    println!("test_bosonoga_btree");
    let input = r"
        FUNAK first
        TALI second
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let mut expected_btree = BTreeMap::new();
    expected_btree.insert("first".to_string(), 10);
    expected_btree.insert("second".to_string(), 20);
    assert_eq!(btree, expected_btree);
}
