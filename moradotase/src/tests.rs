use crate::ast::{BosonogaCommand, BosonogaElement, BosonogaType};
use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga() {
    println!("test_bosonoga");
    let input = r"
        FUNAK first
        FUNAK first
        BUL
        BUL
        TALI second
        TALI second
        INAT
        INAT
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga.len(), 8);
    assert_eq!(
        bosonoga[0],
        BosonogaElement::Command(BosonogaCommand::Funak("first".to_string()))
    );
    assert_eq!(
        bosonoga[1],
        BosonogaElement::Command(BosonogaCommand::Funak("first".to_string()))
    );
    assert_eq!(bosonoga[2], BosonogaElement::Type(BosonogaType::Bul));
    assert_eq!(bosonoga[3], BosonogaElement::Type(BosonogaType::Bul));
    assert_eq!(
        bosonoga[4],
        BosonogaElement::Command(BosonogaCommand::Tali("second".to_string()))
    );
    assert_eq!(
        bosonoga[5],
        BosonogaElement::Command(BosonogaCommand::Tali("second".to_string()))
    );
    assert_eq!(bosonoga[6], BosonogaElement::Type(BosonogaType::Inat));
    assert_eq!(bosonoga[7], BosonogaElement::Type(BosonogaType::Inat));
}
