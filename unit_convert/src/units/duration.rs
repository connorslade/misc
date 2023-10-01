use super::{Conversion, Num};

pub const UNITS: &[&dyn Conversion] = &[&Second, &Minute, &Hour];

pub struct Second;

impl Conversion for Second {
    fn name(&self) -> &'static str {
        "second"
    }

    fn to_base(&self, s: &Num) -> Num {
        *s
    }

    fn from_base(&self, s: &Num) -> Num {
        *s
    }
}

pub struct Minute;

impl Conversion for Minute {
    fn name(&self) -> &'static str {
        "minute"
    }

    fn to_base(&self, m: &Num) -> Num {
        m * 60.0
    }

    fn from_base(&self, s: &Num) -> Num {
        s / 60.0
    }
}

pub struct Hour;

impl Conversion for Hour {
    fn name(&self) -> &'static str {
        "hour"
    }

    fn to_base(&self, h: &Num) -> Num {
        h * 3600.0
    }

    fn from_base(&self, s: &Num) -> Num {
        s / 3600.0
    }
}
