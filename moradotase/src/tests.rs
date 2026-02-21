use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga() {
    println!("test_bosonoga");
    let input = "FUNAK first";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 0);
}
