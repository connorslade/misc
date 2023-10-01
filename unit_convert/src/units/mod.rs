use std::fmt::Display;

type Num = f64;

mod duration;

pub trait Conversion {
    fn name(&self) -> &'static str;
    fn to_base(&self, this: &Num) -> Num;
    fn from_base(&self, s: &Num) -> Num;
}

pub const UNIT_TYPES: &[&str] = &["duration"];
pub const UNIT_CONVERSIONS: &[&[&dyn Conversion]] = &[duration::UNITS];

impl Display for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.name())
    }
}
