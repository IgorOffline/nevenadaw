use crate::ast::*;
use crate::bosonoga::ProgramParser;
use pretty_assertions::assert_eq;

#[test]
fn test_ada_do() {
    let parser = ProgramParser::new();
    let program = parser.parse("ADA DO").unwrap();
    assert_eq!(program, Program::new(vec![Stmt::AdaStmt(AdaStmt::new())]));
}

#[test]
fn test_eki_do() {
    let parser = ProgramParser::new();
    let program = parser.parse("EKI DO").unwrap();
    assert_eq!(program, Program::new(vec![Stmt::EkiStmt(EkiStmt::new())]));
}

#[test]
fn test_multiple_stmts() {
    let parser = ProgramParser::new();
    let program = parser.parse("ADA DO EKI DO ADA DO").unwrap();
    assert_eq!(
        program,
        Program::new(vec![
            Stmt::AdaStmt(AdaStmt::new()),
            Stmt::EkiStmt(EkiStmt::new()),
            Stmt::AdaStmt(AdaStmt::new()),
        ])
    );
}

#[test]
fn test_empty_program() {
    let parser = ProgramParser::new();
    let program = parser.parse("").unwrap();
    assert_eq!(program, Program::new(vec![]));
}

#[test]
fn test_ada_do_whitespace() {
    let parser = ProgramParser::new();
    let program = parser.parse("  ADA   DO  ").unwrap();
    assert_eq!(program, Program::new(vec![Stmt::AdaStmt(AdaStmt::new())]));
}

#[test]
fn test_invalid_input() {
    let parser = ProgramParser::new();
    let result = parser.parse("INVALID");
    assert!(result.is_err());
}
