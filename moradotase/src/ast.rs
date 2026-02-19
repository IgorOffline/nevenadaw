#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    AdaStmt(AdaStmt),
    EkiStmt(EkiStmt),
}

#[derive(Debug, PartialEq)]
pub struct AdaStmt {}

impl AdaStmt {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq)]
pub struct EkiStmt {}

impl EkiStmt {
    pub fn new() -> Self {
        Self {}
    }
}
