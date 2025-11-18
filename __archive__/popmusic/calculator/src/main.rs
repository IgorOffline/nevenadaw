mod calculator2b;

fn main() {
    println!("<MAIN>");
}

#[test]
fn calculator2b() {
    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("foo33").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");

    let result = calculator2b::TermParser::new().parse("(222)").unwrap();
    assert_eq!(result, "222");
}
