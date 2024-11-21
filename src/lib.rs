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
                results.push(format!(
                    "Line {}: {} - Valid ({})",
                    line_number + 1,
                    line,
                    country
                ));
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

pub fn file_info(filename: &str) -> Result<String, String> {
    let contents = fs::read_to_string(filename).map_err(|e| e.to_string())?;
    let mut total_lines = 0;
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut country_counts: HashMap<&str, usize> = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        total_lines += 1;

        match PhoneNumberParser::parse(Rule::phone_number, line) {
            Ok(_) => {
                valid_count += 1;
                let country_code = extract_country_code(line);
                let country = country_name(&country_code).unwrap_or("Unknown Country");
                *country_counts.entry(country).or_insert(0) += 1;
            }
            Err(_) => invalid_count += 1,
        }
    }

    let mut country_info = String::new();
    for (country, count) in &country_counts {
        country_info.push_str(&format!("  {}: {}\n", country, count));
    }

    Ok(format!(
        "File: {}\nTotal lines: {}\nValid numbers: {}\nInvalid numbers: {}\nCountries:\n{}",
        filename, total_lines, valid_count, invalid_count, country_info
    ))
}

pub fn validate_file(filename: &str) -> Result<(), String> {
    let contents = fs::read_to_string(filename).map_err(|e| e.to_string())?;
    let mut invalid_lines = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if PhoneNumberParser::parse(Rule::phone_number, line).is_err() {
            invalid_lines.push(format!("Line {}: {} - Invalid", line_number + 1, line));
        }
    }

    if invalid_lines.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Validation failed! Invalid phone numbers found:\n{}",
            invalid_lines.join("\n")
        ))
    }
}
