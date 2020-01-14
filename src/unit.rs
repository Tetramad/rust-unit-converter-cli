pub enum Unit {
    Area,
    Length,
    Volume,
}

pub fn identify(strings: &(String, String, String)) -> (Unit, Unit, ()) {
    unimplemented!();
}

pub fn is_convertable(from: &Unit, to: &Unit) -> bool {
    match (from, to) {
        (Unit::Area, Unit::Area) | (Unit::Length, Unit::Length) | (Unit::Volume, Unit::Volume) => {
            true
        }
        _ => false,
    }
}

pub fn convert(from: &Unit, to: &Unit, value: ()) -> f64 {
    unimplemented!();
}
