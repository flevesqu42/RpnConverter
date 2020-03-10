/* --- TESTS --- */

use super::RpnConverter;
use super::error;

use std::error::Error;

fn converter() -> RpnConverter<&'static str> {
    RpnConverter::new(["("].to_vec(), [")"].to_vec(), ["!"].to_vec(), ["+", "^", "|"].to_vec())
}

#[test]
fn test_valid1() {

    let standard_notation = "( A | B ) ^ C".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "A B | C ^".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid2() {

    let standard_notation = "A | ( B ^ C )".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "A B C ^ |".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid3() {

    let standard_notation = "B + ( ! A | C )".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "B A ! C | +".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid4() {

    let standard_notation = "( ( A + B ) | ( D | ( ( K + O ) ^ A ) ) ) ^ B".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "A B + D K O + A ^ | | B ^".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid5() {

    let standard_notation = "B ^ ( ( A + B ) | ( D | ( ( K + O ) ^ A ) ) )".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "B A B + D K O + A ^ | | ^".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid6() {

    let standard_notation = "( ( A | B ) + C ) + ( D ^ E ) + ( F | G | I )".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "A B | C + D E ^ + F G | I | +".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_valid7() {

    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + yolo )".split_ascii_whitespace().collect();
    let attempted_rpn : Vec<&str> = "A B + A C ! B + ! + yolo + ^".split_ascii_whitespace().collect();

    let rpn = converter().remove_parenthesis(standard_notation).unwrap();

    assert_eq!(rpn, attempted_rpn);
}

#[test]
fn test_invalid_empty() {
    let standard_notation = "".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::EMPTY_RESULT),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_missing_closing_parenthesis() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + yolo".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::MISSING_CLOSING_PARENTHESIS),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_missing_right_side_value_unary() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + yolo + ! )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::MISSING_RIGHT_SIDE_VALUE_UNARY),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_missing_right_side_value_binary() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + yolo + )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::MISSING_RIGHT_SIDE_VALUE_BINARY),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_missing_left_side_value_binary() {
    let standard_notation = "+ B ^ ( A + ! ( ! C + B ) + yolo )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::MISSING_LEFT_SIDE_VALUE_BINARY),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_unexpected_closing_parenthesis() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + yolo ) )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::UNEXPECTED_CLOSING_PARENTHESIS),
        _                  => panic!("Should not be valid")
    }
}

#[test]
fn test_invalid_successive_two_binary_operands() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B ) + + yolo )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::SUCCESSIVE_TWO_BINARY_OPERANDS),
        _                  => panic!("Should not valid")
    }
}

#[test]
fn test_invalid_successive_two_values() {
    let standard_notation = "A + B ^ ( A + ! ( ! C + B B ) + yolo )".split_ascii_whitespace().collect();

    match converter().remove_parenthesis(standard_notation) {
        Err(error)  => assert_eq!(error.description(), error::SUCCESSIVE_TWO_VALUES),
        _                  => panic!("Should not valid")
    }
}
