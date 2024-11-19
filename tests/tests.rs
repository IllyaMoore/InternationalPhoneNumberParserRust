use international_phone_number_parser::PhoneNumberParser;
use international_phone_number_parser::Rule;
use pest::Parser;

#[test]
fn test_valid_number_without_separators() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+8613800138000");
    assert!(pairs.is_ok());
}

#[test]
fn test_valid_number_with_extension_ext() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+44 20 7946 0958 ext. 123");
    assert!(pairs.is_ok());
}

#[test]
fn test_valid_number_with_extension_x() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1 (123) 456-7890 x123");
    assert!(pairs.is_ok());
}

#[test]
fn test_invalid_number_missing_plus_sign() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "1-800-123-4567");
    assert!(pairs.is_err());
}

#[test]
fn test_invalid_number_with_letters() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1-800-CALL-NOW");
    assert!(pairs.is_err());
}

#[test]
fn test_invalid_number_with_extra_symbols() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1 (123) 456-7890 !!!");
    assert!(pairs.is_err());
}

#[test]
fn test_invalid_number_with_incomplete_country_code() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+ 123 456 7890");
    assert!(pairs.is_err());
}

#[test]
fn test_invalid_number_with_missing_area_code() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1 - 456 7890");
    assert!(pairs.is_err());
}

// Additional test cases to cover potential edge cases
#[test]
fn test_valid_number_with_parentheses() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1 (800) 123-4567");
    assert!(pairs.is_ok());
}

#[test]
fn test_invalid_number_with_extra_symbols_in_extension() {
    let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1-800-123-4567 ext. 123!!!");
    assert!(pairs.is_err());
}
