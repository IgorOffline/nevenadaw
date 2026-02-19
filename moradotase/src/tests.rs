use crate::bosonoga::ProgramParser;
use pretty_assertions::assert_eq;

#[test]
fn test_multiple_stmts() {
    let parser = ProgramParser::new();
    let program = parser.parse("    ADA  DI  2 3 5  DO  EKI  DO  ").unwrap();
    assert_eq!(program, 10);
}
