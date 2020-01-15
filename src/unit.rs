pub enum Unit {
    Area,
    Length,
    Volume,
}

fn from_str(s: &str) -> Unit {
    if is_length_unit(s) {
        Unit::Length
    } else if is_area_unit(s) {
        Unit::Area
    } else if is_volume_unit(s) {
        Unit::Volume
    } else {
        panic!()
    }
}

const LENGTH_UNITS: [&'static str; 8] = ["미터", "m", "야드", "yd", "리", "里", "해리", "海里"];
fn is_length_unit(s: &str) -> bool {
    match LENGTH_UNITS.iter().position(|&u| u == s) {
        Some(_) => true,
        None => false,
    }
}

const AREA_UNITS: [&'static str; 9] = [
    "제곱미터",
    "m^2",
    "m²",
    "m2",
    "헥타르",
    "ha",
    "에이커",
    "ac",
    "평",
];
fn is_area_unit(s: &str) -> bool {
    match AREA_UNITS.iter().position(|&u| u == s) {
        Some(_) => true,
        None => false,
    }
}

const VOLUME_UNITS: [&'static str; 6] = ["리터", "L", "갤런", "gal", "온스", "되"];
fn is_volume_unit(s: &str) -> bool {
    match VOLUME_UNITS.iter().position(|&u| u == s) {
        Some(_) => true,
        None => false,
    }
}

pub fn identify(strings: &(String, String, String)) -> (Unit, Unit, f64) {
    if let Ok(val) = strings.0.parse::<f64>() {
        (
            from_str(strings.1.as_str()),
            from_str(strings.2.as_str()),
            val,
        )
    } else if let Ok(val) = strings.2.parse::<f64>() {
        (
            from_str(strings.0.as_str()),
            from_str(strings.1.as_str()),
            val,
        )
    } else {
        panic!();
    }
}

pub fn is_convertable(from: &Unit, to: &Unit) -> bool {
    match (from, to) {
        (Unit::Area, Unit::Area) | (Unit::Length, Unit::Length) | (Unit::Volume, Unit::Volume) => {
            true
        }
        _ => false,
    }
}

pub fn convert(from: &Unit, to: &Unit, value: f64) -> f64 {
    unimplemented!();
}
