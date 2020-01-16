use std::env;
use crate::message;

enum ParseResult {
    Usage(String),
    Version(String),
    Convert((String, String, String)),
    Fail(String),
}

fn parse() -> ParseResult {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let option = args[1].as_str();
            match option {
                "-v" | "--version" => {
                    return ParseResult::Version(message::version_message());
                },
                "-h" | "--help" => {
                    return ParseResult::Usage(message::usage_message());
                },
                _ => {
                    return ParseResult::Fail(message::help_message());
                },
            };
        },
        4 => {
            return ParseResult::Convert((args[1].clone(), args[2].clone(), args[3].clone()));
        },
        _ => {
            return ParseResult::Fail(message::help_message());
        }
    }
}

pub fn program_name() -> String {
    env::args().nth(0).unwrap()
}

pub fn arguments() -> (String, String, String) {
    match parse() {
        ParseResult::Fail(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
        ParseResult::Usage(message) | ParseResult::Version(message) => {
            println!("{}", message);
            std::process::exit(0);
        }
        ParseResult::Convert(strings) => {
            strings
        }
    }
}
