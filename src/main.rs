use std::env;
use std::process;

use international_phone_number_parser::validate_file;
use international_phone_number_parser::{parse_file, file_info};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" | "-h" => print_help(),
        "--credits" | "-c" => print_credits(),
        "--parse" | "-p" => {
            if args.len() < 3 {
                println!("Error: No file specified for parsing");
                print_help();
                process::exit(1);
            }
            match parse_file(&args[2]) {
                Ok(results) => {
                    for result in results {
                        println!("{}", result);
                    }
                }
                Err(err) => {
                    println!("Error parsing file: {}", err);
                    process::exit(1);
                }
            }
        }
        "--info" | "-i" => {
            if args.len() < 3 {
                println!("Error: No file specified for info");
                print_help();
                process::exit(1);
            }
            match file_info(&args[2]) {
                Ok(info) => println!("{}", info),
                Err(err) => {
                    println!("Error getting file info: {}", err);
                    process::exit(1);
                }
            }
        }
        "--validate" | "-v" => {
            if args.len() < 3 {
                println!("Error: No file specified for validation");
                print_help();
                process::exit(1);
            }
            match validate_file(&args[2]) {
                Ok(_) => println!("Validation successful! All phone numbers are valid."),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }

        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Phone Number Parser - Command Line Interface:");
    println!("\nCommands:");
    println!("  -p, --parse <file>    Parse phone numbers from a file");
    println!("  -i, --info <file>     Show information about the file");
    println!("  -h, --help            Show this help message");
    println!("  -c, --credits         Show credits information");
    println!("  -i, --info <file>     Show file info")
}

fn print_credits() {
    println!("\n               ---------------CREDITS---------------");
    println!("This learning project was created with Rust and the Pest parsing.");
    println!("For any feedback, feel free to contribute or share suggestions.\n");
}
