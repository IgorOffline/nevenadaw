use crate::bosonoga::BosonogaBTreeParser;
use std::fs;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

mod ast;

#[cfg(test)]
mod tests;

fn main() {
    println!("test_bosonoga");
    let input = fs::read_to_string("bosonoga-input.txt").expect("Unable to read file");
    let parser = BosonogaBTreeParser::new();
    let _ = parser.parse(&input).unwrap();
}
