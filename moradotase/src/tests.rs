use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_bosonoga() {
    println!("test_bosonoga");
    let input = r"
        FUNAK todomakeuniquepertype
        FUNAK todomakeuniquepertype
        NEO-RAEKORDIO todomakeuniquepertype
        NEO-RAEKORDIO todomakeuniquepertype
        TALI todomakeuniquepertype
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 0);
}
