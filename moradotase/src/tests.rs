use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga_tali() {
    let input = r"
        DI
        TALI
        DO
        TALI
        DI
        TALI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 201);
}

#[test]
fn test_bosonoga_bul_inat() {
    let input = r"
        DI
        DO
        DI
        VAL INAT first 50
        ADA first 49
        TALI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 300);
}
