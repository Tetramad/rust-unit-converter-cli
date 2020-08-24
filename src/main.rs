mod argparser;
mod unit_literals;
mod message;

use argparser::arguments;
use std::convert::TryFrom;
use unit_literals::convert;
use unit_literals::UnitLiteral;

fn main() {
    let args = arguments();
    let (from, to, value) = identify(&args);

    match convert(&from, &to, value) {
        Ok(converted) => {
            println!("{}", converted);
            std::process::exit(0);
        }
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }
}

fn identify(args: &(String, String, String)) -> (UnitLiteral, UnitLiteral, f64) {
    let on_failure = |m: String| {
        eprintln!("{}", m);
        std::process::exit(1);
    };

    match (
        args.0.parse::<f64>(),
        args.1.parse::<f64>(),
        args.2.parse::<f64>(),
    ) {
        (Ok(value), Err(_), Err(_)) => (
            UnitLiteral::try_from(&args.1).unwrap_or_else(on_failure),
            UnitLiteral::try_from(&args.2).unwrap_or_else(on_failure),
            value,
        ),
        (Err(_), Err(_), Ok(value)) => (
            UnitLiteral::try_from(&args.0).unwrap_or_else(on_failure),
            UnitLiteral::try_from(&args.1).unwrap_or_else(on_failure),
            value,
        ),
        _ => {
            eprintln!("{}", message::help_message());
            std::process::exit(1);
        }
    }
}
