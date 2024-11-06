use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct PhoneNumberParser;

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_valid_us_number_with_separators() {
        let pairs = PhoneNumberParser::parse(Rule::phone_number, "+1 (123) 456-7890");
        assert!(pairs.is_ok());
    }

    #[test]
    fn test_valid_uk_number_with_spaces() {
        let pairs = PhoneNumberParser::parse(Rule::phone_number, "+44 20 7946 0958");
        assert!(pairs.is_ok());
    }

    #[test]
    fn test_invalid_number_missing_country_code() {
        let pairs = PhoneNumberParser::parse(Rule::phone_number, "123 456 7890");
        assert!(pairs.is_err());
    }
}
