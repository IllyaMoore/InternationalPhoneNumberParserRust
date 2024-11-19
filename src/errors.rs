use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum PhoneNumberParserError {
    #[error("Invalid file path: {0}")]
    InvalidFilePath(PathBuf),

    #[error("File reading error: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Parsing error for line {line_number}: {line}")]
    ParsingError {
        line_number: usize,
        line: String,
    },

    #[error("No input file specified")]
    MissingInputFile,
}