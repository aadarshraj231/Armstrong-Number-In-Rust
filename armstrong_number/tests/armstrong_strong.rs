extern crate armstrong_numbers;
use armstrong_numbers::*;

#[test]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(main(5))
}

#[test]
fn test_there_are_no_2_digit_armstring_numbers() {
    assert!(!main(10))
}
