use crate::bosonoga::BosonogaParser;
use pretty_assertions::assert_eq;

#[test]
fn test_multiple_stmts() {
    let input = r"
        ADA  DI  2 3 5  DO   LOGI 1   EKI  DO
        ADA  DI  2 3 5 10  DO   LOGI 2   EKI  DO   LOGI 9
    ";
    let parser = BosonogaParser::new();
    let bosonoga = parser.parse(input).unwrap();
    assert_eq!(bosonoga, 30);
}
