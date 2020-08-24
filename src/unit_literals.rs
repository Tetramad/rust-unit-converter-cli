pub enum UnitLiteral {
    Meter,
    Yard,
    Ri,
    SqrMeter,
    Hactare,
    Acre,
    Pyeong,
    Liter,
    Gallon,
    Ounce,
    Doe,
}

pub fn convert(from: &UnitLiteral, to: &UnitLiteral, value: f64) -> Result<f64, String> {
    if converible(from, to) {
        Ok(value * to.factor() / from.factor())
    } else {
        Err(format!("Invalid conversion from {} to {}.", from.group(), to.group()))
    }
}

fn converible(from: &UnitLiteral, to: &UnitLiteral) -> bool {
    from.group() == to.group()
}

impl UnitLiteral {
    fn factor(&self) -> f64 {
        match self {
            UnitLiteral::Meter => 1.0,
            UnitLiteral::Yard => 1.093613,
            UnitLiteral::Ri => 0.002546,
            UnitLiteral::SqrMeter => 1.0,
            UnitLiteral::Hactare => 0.0001,
            UnitLiteral::Acre => 0.000247,
            UnitLiteral::Pyeong => 0.3025,
            UnitLiteral::Liter => 1.0,
            UnitLiteral::Gallon => 0.264172,
            UnitLiteral::Ounce => 33.814022,
            UnitLiteral::Doe => 0.554354,
        }
    }

    fn group(&self) -> &'static str {
        match self {
            UnitLiteral::Meter | UnitLiteral::Yard | UnitLiteral::Ri => "length",
            UnitLiteral::SqrMeter
            | UnitLiteral::Hactare
            | UnitLiteral::Acre
            | UnitLiteral::Pyeong => "area",
            UnitLiteral::Liter | UnitLiteral::Gallon | UnitLiteral::Ounce | UnitLiteral::Doe => {
                "volume"
            }
        }
    }
}

impl std::convert::TryFrom<&String> for UnitLiteral {
    type Error = String;

    fn try_from(literal: &String) -> Result<Self, Self::Error> {
        match literal.as_str() {
            "미터" | "meter" | "meters" | "m" => Ok(UnitLiteral::Meter),
            "야드" | "yard" | "yards" | "yd" => Ok(UnitLiteral::Yard),
            "리" | "ri" | "里" => Ok(UnitLiteral::Ri),
            "제곱미터" | "m^2" | "m2" | "m²" => Ok(UnitLiteral::SqrMeter),
            "헥타르" | "hactare" | "hactares" | "ha" => Ok(UnitLiteral::Hactare),
            "에이커" | "acre" | "acres" | "ac" => Ok(UnitLiteral::Acre),
            "평" => Ok(UnitLiteral::Pyeong),
            "리터" | "liter" | "liters" | "L" => Ok(UnitLiteral::Liter),
            "갤런" | "gallon" | "gallons" | "gal" => Ok(UnitLiteral::Gallon),
            "온즈" | "ounce" | "ounces" | "oz" => Ok(UnitLiteral::Ounce),
            "되" => Ok(UnitLiteral::Doe),
            _ => Err(format!("'{}' can't be found in any implemented unit literal", literal))
        }
    }
}
