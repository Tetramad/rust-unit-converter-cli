use std::env;

mod area;
mod length;
mod volume;

const USAGE_MESSAGE: &str = "
$ conv -h
[사용법]
conv [입력 단위] [출력 단위] [값]
conv [값] [입력 단위] [출력 단위]

=예시=
conv m yd 100
conv 100 m yd

[단위]
길이
	미터 mm
	야드 yd
	리 里
	해리 海里
넓이
	제곱미터 m^2 m2
	헥타르 ha
	에이커 ac
	평
부피
	리터 L
	갤런 gal
	온스 oz
	되

[버전]
conv [-v|--version]
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let maybe_option = args.get(1).unwrap();
        match maybe_option.as_str() {
            "-v" | "--version" => println!("Unit Converter version {}", env!("CARGO_PKG_VERSION")),
            _ => println!("{}", USAGE_MESSAGE),
        };
        return;
    } else if args.len() != 3 {
        println!("{}", USAGE_MESSAGE);
        return;
    }

    let from = args.get(1).unwrap();
    let to = args.get(2).unwrap();
    let value: f64 = args.get(3).unwrap().parse().unwrap();

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
