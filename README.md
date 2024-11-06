# International Phone Number Parser

This project is an **International Phone Number Parser** written in Rust using the **Pest** parsing library. This parser is designed to recognize, validate, and parse mobile phone numbers from different countries, accounting for various formats, country codes, and common number structures. It allows for parsing both formatted (e.g., `+1 (123) 456-7890`) and unformatted numbers (e.g., `+11234567890`), with an emphasis on flexibility for diverse numbering conventions worldwide.

## Project Overview

The goal of this parser is to handle the syntactic structure of phone numbers rather than verifying each number's validity against a database of valid numbers. It will split the phone number into components, such as the country code, area/operator code, and subscriber number, which can be used in applications requiring standardized phone numbers.

## Parsing Process

### 1. Lexical Analysis

Using Pest’s grammar syntax, we first define the token patterns to recognize different parts of a phone number. Our lexer identifies:
   - **Country Code**: Prefixed by `+`, followed by 1-3 digits (e.g., `+1`, `+44`, `+358`).
   - **Area/Operator Code**: Following the country code, area or operator codes are defined based on country-specific patterns (e.g., `123` for North America).
   - **Subscriber Number**: The main number after the area/operator code, typically 7-10 digits depending on the country.
   - **Separators**: Characters such as spaces, hyphens (`-`), and parentheses `()`, which are frequently used in formatting phone numbers.

### 2. Syntactic Analysis

The syntactic analyzer matches phone numbers to specific rules to determine the structure and validity of the format. Pest grammar allows us to define optional parts (such as parentheses around area codes) and enforce consistency in patterns.

Key rules for syntactic analysis include:
   - **Country Code Validation**: Ensures that the number begins with a valid country code pattern (`+` followed by 1-3 digits).
   - **Length Validation**: Different countries have different lengths for phone numbers; for example, North American numbers typically follow a 10-digit format, while others may be shorter or longer.
   - **Flexible Formatting**: Recognizes numbers with or without separators (e.g., spaces, hyphens), making the parser adaptable to various user-input formats.

### 3. Parsing Rules

The parser uses Pest’s grammar capabilities to define phone number rules for several regions, which can be easily extended as needed. Examples of common rules include:
   - **North American Numbering Plan (NANP)**: Matches phone numbers in the format `+1 (123) 456-7890` or `+11234567890`.
   - **European Numbers**: Matches numbers with varying country and area codes, such as `+44 123 456 7890` for the UK.
   - **Flexible Parsing**: The parser can handle missing elements, such as optional separators, without compromising accuracy.

## Result of Parsing

Once the phone number is parsed, the output is structured data containing:
   - **Country Code**: Useful for identifying the caller's country or determining applicable dialing rules.
   - **Area/Operator Code**: Useful for further regional segmentation within countries.
   - **Subscriber Number**: The main phone number, which could be used directly for dialing or display.

This parsed structure can then be used in applications like:
   - **Phone Number Standardization**: Convert numbers to a consistent format.
   - **Validation and Error Handling**: Check for country-specific format violations.
   - **Caller Location Approximation**: Infer country or region based on the parsed country and area codes.

## Usage

To run this parser:

1. **Clone the Repository**
   ```bash
   git clone <repository_url>
   cd international-phone-number-parser
