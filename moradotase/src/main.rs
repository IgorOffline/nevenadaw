use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(pub bosonoga);

#[cfg(test)]
mod tests;

fn main() {
    println!("<START>");
    println!("<END>");
}
