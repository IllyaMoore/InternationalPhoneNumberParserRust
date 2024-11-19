use std::env;
use std::process;

use international_phone_number_parser::{parse_file};

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
    println!("  -h, --help           Show this help message");
    println!("  -c, --credits        Show credits information");
}

fn print_credits() {
    print!("\n");
    println!("               ---------------CREDITS---------------");
    println!("This learning project was created with the Rust and the Pest parsing.");
    println!("Key Contributions:");
    println!(" Rust:\n  For providing a robust and efficient platform for building performant parsers.");
    println!(" Pest Library:\n  For its powerful yet user-friendly parsing capabilities,\n  enabling the definition of clear and flexible grammar rules.\n");
    println!(" If you find this project helpful, consider contributing or sharing feedback to help improve it further!\n");
    }