use crate::area;
use crate::length;
use crate::volume;

pub enum Unit {
    Length(length::Length),
    Area(area::Area),
    Volume(volume::Volume),
}

fn from_str(s: &str) -> Unit {
    if let Some(u) = to_length(s) {
        Unit::Length(u)
    } else if let Some(u) = to_area(s) {
        Unit::Area(u)
    } else if let Some(u) = to_volume(s) {
        Unit::Volume(u)
    } else {
        panic!()
    }
}

const METER_UNITS: [&'static str; 2] = ["미터", "m"];
const YARD_UNITS: [&'static str; 2] = ["야드", "yd"];
const RI_UNITS: [&'static str; 2] = ["리", "里"];
const HAERI_UNITS: [&'static str; 2] = ["해리", "海里"];
fn to_length(s: &str) -> Option<length::Length> {
    if let Some(_) = METER_UNITS.iter().position(|&u| u == s) {
        Some(length::Length::Meter(0.0))
    } else if let Some(_) = YARD_UNITS.iter().position(|&u| u == s) {
        Some(length::Length::Yard(0.0))
    } else if let Some(_) = RI_UNITS.iter().position(|&u| u == s) {
        Some(length::Length::Ri(0.0))
    } else {
        None
    }
}

const SQRMETER_UNITS: [&'static str; 4] = ["제곱미터", "m^2", "m²", "m2"];
const HACTARE_UNITS: [&'static str; 2] = ["헥타르", "ha"];
const ACRE_UNITS: [&'static str; 2] = ["에이커", "ac"];
const PYEONG_UNITS: [&'static str; 1] = ["평"];
fn to_area(s: &str) -> Option<area::Area> {
    if let Some(_) = SQRMETER_UNITS.iter().position(|&u| u == s) {
        Some(area::Area::SqrMeter(0.0))
    } else if let Some(_) = HACTARE_UNITS.iter().position(|&u| u == s) {
        Some(area::Area::Hactare(0.0))
    } else if let Some(_) = ACRE_UNITS.iter().position(|&u| u == s) {
        Some(area::Area::Acre(0.0))
    } else if let Some(_) = PYEONG_UNITS.iter().position(|&u| u == s) {
        Some(area::Area::Pyeong(0.0))
    } else {
        None
    }
}

const LITER_UNITS: [&'static str; 2] = ["리터", "L"];
const GALLON_UNITS: [&'static str; 2] = ["갤런", "gal"];
const OUNCE_UNITS: [&'static str; 2] = ["온스", "oz"];
const DOE_UNITS: [&'static str; 1] = ["되"];
fn to_volume(s: &str) -> Option<volume::Volume> {
    if let Some(_) = LITER_UNITS.iter().position(|&u| u == s) {
        Some(volume::Volume::Liter(0.0))
    } else if let Some(_) = GALLON_UNITS.iter().position(|&u| u == s) {
        Some(volume::Volume::Gallon(0.0))
    } else if let Some(_) = OUNCE_UNITS.iter().position(|&u| u == s) {
        Some(volume::Volume::Ounce(0.0))
    } else if let Some(_) = DOE_UNITS.iter().position(|&u| u == s) {
        Some(volume::Volume::Doe(0.0))
    } else {
        None
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
        (Unit::Area(_), Unit::Area(_))
        | (Unit::Length(_), Unit::Length(_))
        | (Unit::Volume(_), Unit::Volume(_)) => true,
        _ => false,
    }
}

pub fn convert(from: &Unit, to: &Unit, value: f64) -> f64 {
    unimplemented!();
}
