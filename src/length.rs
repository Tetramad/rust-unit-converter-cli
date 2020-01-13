const METER: f64 = 1.0;
const YARD: f64 = 1.093613;
const RI: f64 = 0.002546;

pub enum Length {
    Meter(f64),
    Yard(f64),
    Ri(f64),
}

pub fn conv_to_meter(length: Length) -> Length {
    match length {
        Length::Meter(m) => Length::Meter(m / METER),
        Length::Yard(y) => Length::Meter(y / YARD),
        Length::Ri(r) => Length::Meter(r / RI),
    }
}

pub fn conv_to_yard(length: Length) -> Length {
    match conv_to_meter(length) {
        Length::Meter(m) => Length::Yard(m * YARD),
        _ => panic!("[in conv_to_yard] conv_to_meter didn't return Length::Meter"),
    }
}

pub fn conv_to_ri(length: Length) -> Length {
    match conv_to_meter(length) {
        Length::Meter(m) => Length::Ri(m * RI),
        _ => panic!("[in conv_to_ri] conv_to_meter didn't return Length::Meter"),
    }
}
