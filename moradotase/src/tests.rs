use crate::bosonoga::ProgramParser;
use pretty_assertions::assert_eq;

#[test]
fn test_multiple_stmts() {
    let input = r"
        ADA  DI  2 3 5  DO  EKI  DO
        ADA  DI  2 3 5 10  DO  EKI  DO
    ";
    let parser = ProgramParser::new();
    let program = parser.parse(input).unwrap();
    assert_eq!(program, 30);
}
