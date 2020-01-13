const LITER: f64 = 1.0;
const GALLON: f64 = 0.264172;
const OUNCE: f64 = 33.814022;
const DOE: f64 = 0.554354;

pub enum Volume {
    Liter(f64),
    Gallon(f64),
    Ounce(f64),
    Doe(f64),
}

pub fn conv_to_liter(volume: Volume) -> Volume {
    match volume {
        Volume::Liter(l) => Volume::Liter(l / LITER),
        Volume::Gallon(gal) => Volume::Liter(gal / GALLON),
        Volume::Ounce(oz) => Volume::Liter(oz / OUNCE),
        Volume::Doe(doe) => Volume::Doe(doe / DOE),
    }
}

pub fn conv_to_gallon(volume: Volume) -> Volume {
    match conv_to_liter(volume) {
        Volume::Liter(l) => Volume::Gallon(l * GALLON),
        _ => panic!("[in conv_to_gallon] conv_to_meter didn't return Volume::Liter"),
    }
}

pub fn conv_to_ounce(volume: Volume) -> Volume {
    match conv_to_liter(volume) {
        Volume::Liter(l) => Volume::Ounce(l * OUNCE),
        _ => panic!("[in conv_to_ounce] conv_to_meter didn't return Volume::Liter"),
    }
}

pub fn conv_to_doe(volume: Volume) -> Volume {
    match conv_to_liter(volume) {
        Volume::Liter(l) => Volume::Doe(l * DOE),
        _ => panic!("[in conv_to_doe] conv_to_meter didn't return Volume::Liter"),
    }
}
