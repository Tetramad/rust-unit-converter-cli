use std::env;

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
                    let message = VERSION_MESSAGE.replace(VERSION, env!("CARGO_PKG_VERSION"));
                    return ParseResult::Version(message);
                },
                "-h" | "--help" => {
                    let message = USAGE_MESSAGE.replace(PROGRAM_NAME, args[0].as_str());
                    return ParseResult::Usage(message);
                },
                _ => {
                    let message = HELP_MESSAGE.replace(PROGRAM_NAME, args[0].as_str());
                    return ParseResult::Fail(message);
                },
            };
        },
        4 => {
            return ParseResult::Convert((args[1].clone(), args[2].clone(), args[3].clone()));
        },
        _ => {
            let message = HELP_MESSAGE.replace(PROGRAM_NAME, args[0].as_str());
            return ParseResult::Fail(message);
        }
    }
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

const PROGRAM_NAME: &'static str = "{program_name}";
const VERSION: &'static str = "{version}";
const VERSION_MESSAGE: &'static str = "
Unit Converter version {version}
";
const HELP_MESSAGE: &'static str = "
{program_name} [-h|--help] to see how to use.
";
const USAGE_MESSAGE: &'static str = "
[Usage]
{program_name} [input unit] [output unit] [value]
{program_name} [value] [input unit] [output unit]

Example:
  {program_name} m yd 100   convert 100 meters to yards.
  {program_name} 100 m yd   same as the above.

[Unit]
Length
	미터 mm
	야드 yd
	리 里
	해리 海里
Area
	제곱미터 m^2 m2 m²
	헥타르 ha
	에이커 ac
	평
Volume
	리터 L
	갤런 gal
	온스 oz
	되
";
