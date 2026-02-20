use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga_basic_identifiers() {
    let input = r"
        DI   LOGI   ADA 2000   SUB 1000   DO
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 1111);
}

#[test]
fn test_bosonoga_multiple_subtractions() {
    let input = r"
        DI   SUB-DI 5 5 SUB-DO   DI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 190);
}
