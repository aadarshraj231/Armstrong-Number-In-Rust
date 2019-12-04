extern crate armstrong_numbers;
use armstrong_numbers::*;

#[test]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
fn test_there_are_no_2_digit_armstring_numbers() {
    assert!(!is_armstrong_number(10))
}
