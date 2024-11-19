use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PhoneNumberParser;

pub fn parse_file(filename: &str) -> Result<Vec<String>, String> {
    let contents = fs::read_to_string(filename).map_err(|e| e.to_string())?;
    let mut results = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match PhoneNumberParser::parse(Rule::phone_number, line) {
            Ok(parsed) => {
                let country_code = extract_country_code(line);
                let country = country_name(&country_code).unwrap_or("Unknown Country");
                results.push(format!("Line {}: {} - Valid ({})", line_number + 1, line, country));
            }
            Err(err) => {
                results.push(format!(
                    "Line {}: {} - Invalid (Error: {})",
                    line_number + 1,
                    line,
                    err
                ));
            }
        }
    }

    Ok(results)
}

pub fn extract_country_code(phone_number: &str) -> String {
    phone_number
        .trim_start_matches('+')
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect()
}

pub fn country_name(country_code: &str) -> Option<&'static str> {
    let mut country_map = HashMap::new();
    country_map.insert("1", "USA/Canada");
    country_map.insert("44", "United Kingdom");
    country_map.insert("86", "China");
    country_map.insert("49", "Germany");
    country_map.insert("81", "Japan");

    country_map.get(country_code).copied()
}
