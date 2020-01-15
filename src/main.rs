mod argparser;
mod unit_literals;

use argparser::arguments;
use std::convert::TryFrom;
use unit_literals::convert;
use unit_literals::UnitLiteral;

fn main() {
    let args = arguments();
    let (from, to, value) = identify(&args);

    println!("{}", convert(&from, &to, value).unwrap());
}

fn identify(args: &(String, String, String)) -> (UnitLiteral, UnitLiteral, f64) {
    match (
        args.0.parse::<f64>(),
        args.1.parse::<f64>(),
        args.2.parse::<f64>(),
    ) {
        (Ok(value), Err(_), Err(_)) => (
            UnitLiteral::try_from(&args.1).unwrap(),
            UnitLiteral::try_from(&args.2).unwrap(),
            value,
        ),
        (Err(_), Err(_), Ok(value)) => (
            UnitLiteral::try_from(&args.0).unwrap(),
            UnitLiteral::try_from(&args.1).unwrap(),
            value,
        ),
        _ => unimplemented!(),
    }
}
