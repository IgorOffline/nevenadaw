use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga() {
    use crate::ast::BosonogaCommand;
    println!("test_bosonoga");
    let input = r"
        FUNAK first
        FUNAK first
        TALI second
        TALI second
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    println!("Return value: {:?}", bosonoga);
    assert_eq!(bosonoga.len(), 4);
    assert_eq!(bosonoga[0], BosonogaCommand::Funak("first".to_string()));
    assert_eq!(bosonoga[1], BosonogaCommand::Funak("first".to_string()));
    assert_eq!(bosonoga[2], BosonogaCommand::Tali("second".to_string()));
    assert_eq!(bosonoga[3], BosonogaCommand::Tali("second".to_string()));
}
