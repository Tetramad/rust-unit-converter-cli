mod area;
mod argparser;
mod length;
mod volume;

use argparser::parse_arguments;
use argparser::ParseResult;

fn main() {
    let (from, to, value) = match parse_arguments() {
        ParseResult::Fail(message) => {
            eprintln!("{}", message);
            return;
        }
        ParseResult::Usage(message) | ParseResult::Version(message) => {
            println!("{}", message);
            return;
        }
        ParseResult::Convert(first_owen, second_owen, third_owen) => {
            (first_owen, second_owen, third_owen)
        }
    };

    let value: f64 = value.parse().unwrap();

    if from.as_str() == "m" || from.as_str() == "yd" || from.as_str() == "리" {
        let value = match from.as_str() {
            "m" => length::Length::Meter(value),
            "yd" => length::Length::Yard(value),
            "리" => length::Length::Ri(value),
            _ => panic!(),
        };

        let converted = match to.as_str() {
            "m" => length::conv_to_meter(value),
            "yd" => length::conv_to_yard(value),
            "리" => length::conv_to_ri(value),
            _ => panic!(),
        };

        let v = match converted {
            length::Length::Meter(m) => m,
            length::Length::Ri(r) => r,
            length::Length::Yard(y) => y,
        };

        println!("{}", v);
    } else if from.as_str() == "m2"
        || from.as_str() == "ha"
        || from.as_str() == "ac"
        || from.as_str() == "평"
    {
        let value = match from.as_str() {
            "m2" => area::Area::SqrMeter(value),
            "ha" => area::Area::Hactare(value),
            "ac" => area::Area::Acre(value),
            "평" => area::Area::Pyeong(value),
            _ => panic!(),
        };

        let converted = match to.as_str() {
            "m2" => area::conv_to_sqr_meter(value),
            "ha" => area::conv_to_hactare(value),
            "평" => area::conv_to_pyeong(value),
            "ac" => area::conv_to_acre(value),
            _ => panic!(),
        };

        let v = match converted {
            area::Area::SqrMeter(m2) => m2,
            area::Area::Hactare(ha) => ha,
            area::Area::Pyeong(pyeong) => pyeong,
            area::Area::Acre(ac) => ac,
        };

        println!("{}", v);
    } else if from.as_str() == "L"
        || from.as_str() == "gal"
        || from.as_str() == "oz"
        || from.as_str() == "되"
    {
        let value = match from.as_str() {
            "L" => volume::Volume::Liter(value),
            "gal" => volume::Volume::Gallon(value),
            "oz" => volume::Volume::Ounce(value),
            "되" => volume::Volume::Doe(value),
            _ => panic!(),
        };

        let converted = match to.as_str() {
            "L" => volume::conv_to_liter(value),
            "gal" => volume::conv_to_gallon(value),
            "oz" => volume::conv_to_ounce(value),
            "되" => volume::conv_to_doe(value),
            _ => panic!(),
        };

        let v = match converted {
            volume::Volume::Liter(l) => l,
            volume::Volume::Gallon(gal) => gal,
            volume::Volume::Ounce(oz) => oz,
            volume::Volume::Doe(doe) => doe,
        };

        println!("{}", v);
    }
}
