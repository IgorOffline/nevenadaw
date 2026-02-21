use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

mod ast;

#[cfg(test)]
mod tests;

fn main() {
    println!("<START>");
    println!("<END>");
}
