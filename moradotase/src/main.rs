use crate::bosonoga::BosonogaParser;
use std::collections::BTreeSet;
use std::fs;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

mod ast;

#[cfg(test)]
mod tests;

use ast::*;

fn main() {
    println!("test_bosonoga");
    let input = fs::read_to_string("input.bosonoga").expect("Unable to read file");
    let parser = BosonogaParser::new();
    let commands = parser.parse(&input).unwrap();

    let mut variables = BTreeSet::new();

    for command in commands {
        match command {
            BosonogaElement::Command(BosonogaCommand::Set(
                BosonogaType::Inat,
                name,
                BosonogaValue::Inat(v),
            )) => {
                variables.replace(BosonogaVariable::new_i32(name, v));
            }
            BosonogaElement::Command(BosonogaCommand::Set(
                BosonogaType::Bul,
                name,
                BosonogaValue::Bul(v),
            )) => {
                variables.replace(BosonogaVariable::new_bool(name, v));
            }
            BosonogaElement::Command(BosonogaCommand::Game(w, h, title, color)) => {
                game_launch(w as u32, h as u32, title, color, variables.clone());
            }
            BosonogaElement::Command(BosonogaCommand::Tali) => {
                println!("TALI[{:?}]", variables);
            }
            _ => {}
        }
    }
}
