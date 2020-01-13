const SQR_METER: f64 = 1.0;
const HACTARE: f64 = 0.0001;
const ACRE: f64 = 0.000247;
const PYEONG: f64 = 0.3025;

pub enum Area {
    SqrMeter(f64),
    Hactare(f64),
    Acre(f64),
    Pyeong(f64),
}

pub fn conv_to_sqr_meter(area: Area) -> Area {
    match area {
        Area::SqrMeter(m2) => Area::SqrMeter(m2 / SQR_METER),
        Area::Hactare(ha) => Area::SqrMeter(ha / HACTARE),
        Area::Acre(ac) => Area::SqrMeter(ac / ACRE),
        Area::Pyeong(pyeong) => Area::SqrMeter(pyeong / PYEONG),
    }
}

pub fn conv_to_hactare(area: Area) -> Area {
    match conv_to_sqr_meter(area) {
        Area::SqrMeter(m2) => Area::Hactare(m2 * HACTARE),
        _ => panic!("[in conv_to_hactare] conv_to_meter didn't return Area::SqrMeter"),
    }
}

pub fn conv_to_acre(area: Area) -> Area {
    match conv_to_sqr_meter(area) {
        Area::SqrMeter(m2) => Area::Acre(m2 * ACRE),
        _ => panic!("[in conv_to_acre] conv_to_meter didn't return Area::SqrMeter"),
    }
}

pub fn conv_to_pyeong(area: Area) -> Area {
    match conv_to_sqr_meter(area) {
        Area::SqrMeter(m2) => Area::Pyeong(m2 * PYEONG),
        _ => panic!("[in conv_to_pyeong] conv_to_meter didn't return Area::SqrMeter"),
    }
}
