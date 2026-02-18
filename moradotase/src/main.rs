use lalrpop_util::lalrpop_mod;

lalrpop_mod!(cobol);

fn main() {
    println!("<START>");
    println!("<END>");
}

#[cfg(test)]
mod tests {
    use super::cobol;

    #[test]
    fn test_program_id_term() {
        let input = "PROGRAM-ID. sum.";
        let parsed = cobol::ProgramIdTermParser::new().parse(input).unwrap();
        assert_eq!(parsed, "PROGRAM-ID. sum.");
    }

    #[test]
    fn test_pop_init() {
        let sum_string = "sum";
        let pop_result = cobol::TermParser::new().parse(sum_string).unwrap();
        let expected_result = format!("PROGRAM-ID. {}.", sum_string);
        assert_eq!(pop_result, expected_result);
    }
}
