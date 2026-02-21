use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga_tali() {
    println!("test_bosonoga_tali");
    let input = r"
        DI
        DO
        DI
        TALI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 201);
}

#[test]
fn test_bosonoga_bul_inat() {
    println!("test_bosonoga_inat TODO bul");
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

#[test]
fn test_funak_veda() {
    println!("test_funak_veda");
    let input = r"
        DI
        FUNAK fnfirst
        DO
        VEDA fnfirst
        DI
        TALI
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 201);
}
