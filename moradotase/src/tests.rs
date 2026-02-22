use crate::ast::{BosonogaCommand, BosonogaElement, BosonogaType};
use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

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
