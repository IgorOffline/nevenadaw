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
        DI   ADA-DI 10 10 ADA-DO   SUB-DI 5 5 SUB-DO   DI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 210);
}

#[test]
fn test_bosonoga_loop() {
    let input = r"
        DI
        VALA-DI 5
          ADA 10
        VALA-DO
        DI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 250);
}

#[test]
fn test_bosonoga_types() {
    let input = r"
        DI
        BATIP BUL   BATIP INAT
        DI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 200);
}
