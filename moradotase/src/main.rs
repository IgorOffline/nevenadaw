use crate::bosonoga::BosonogaBTreeParser;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

mod ast;

#[cfg(test)]
mod tests;

fn main() {
    println!("test_bosonoga");
    let input = r"
        GAME 640 360
    ";
    let parser = BosonogaBTreeParser::new();
    let _ = parser.parse(input).unwrap();
}
