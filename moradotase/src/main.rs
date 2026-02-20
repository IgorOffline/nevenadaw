use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

#[derive(Debug)]
pub enum BosonogaItem {
    Add(String, i32),
    Tali,
}

#[cfg(test)]
mod tests;

fn main() {
    println!("<START>");
    println!("<END>");
}
