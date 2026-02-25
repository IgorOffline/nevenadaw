use crate::ast::{BosonogaCommand, BosonogaElement, BosonogaVariable};
use crate::bosonoga::{BosonogaBTreeParser, BosonogaParser};
use pretty_assertions::assert_eq;
use std::collections::BTreeSet;

#[test]
fn test_bosonoga_commands() {
    println!("test_bosonoga_commands");
    let input = r"
        SET INAT first 10
        SET INAT second 20
        SET BUL third false
        SET INAT first 30
        SET INAT second 40
        SET BUL third true
        SET INAT rectangle_count 5
        TALI
    ";
    let parser = BosonogaBTreeParser::new();
    let btree = parser.parse(input).unwrap();
    let first_var = BosonogaVariable::new_i32("first".to_string(), 30);
    let second_var = BosonogaVariable::new_i32("second".to_string(), 40);
    let third_var = BosonogaVariable::new_bool("third".to_string(), true);
    let rectangle_count_var = BosonogaVariable::new_i32("rectangle_count".to_string(), 5);
    let mut expected_btree: BTreeSet<BosonogaVariable> = BTreeSet::new();
    expected_btree.insert(first_var);
    expected_btree.insert(second_var);
    expected_btree.insert(third_var);
    expected_btree.insert(rectangle_count_var);
    assert_eq!(btree, expected_btree);

    for var in &btree {
        let expected_var = expected_btree.get(var).unwrap();
        assert_eq!(
            var.value, expected_var.value,
            "Value mismatch for variable {}",
            var.name
        );
    }
}

#[test]
fn test_neo_reket_di() {
    println!("test_neo_reket_di");
    let input = "NEO-REKET-DI 3 4 NEO-REKET-X 20 NEO-REKET-Y 100";
    let parser = BosonogaParser::new();
    let result = parser.parse(input).unwrap();
    assert_eq!(result.len(), 1);
    match &result[0] {
        BosonogaElement::Command(BosonogaCommand::SpawnRectangles(count_x, count_y, x, y)) => {
            assert_eq!(*count_x, 3);
            assert_eq!(*count_y, 4);
            assert_eq!(*x, 20);
            assert_eq!(*y, 100);
        }
        _ => panic!("Expected SpawnRectangles command"),
    }
}
