use std::fmt::{Debug, Display};

use crate::{impl_conversion, impl_unit_space, Num};

pub mod duration;

pub const UNIT_SPACES: &[&dyn UnitSpace] = &[&duration::Duration];

pub trait UnitSpace {
    fn name(&self) -> &'static str;
    fn units(&self) -> &'static [&'static dyn Conversion];
}

pub trait Conversion {
    fn name(&self) -> &'static str;
    fn to_base(&self, this: &Num) -> Num;
    fn from_base(&self, s: &Num) -> Num;
}

#[macro_export]
macro_rules! impl_conversion {
    ($struct:ident, $name:expr, $to_base:expr, $from_base:expr) => {
        pub struct $struct;
        impl Conversion for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn to_base(&self, this: &Num) -> Num {
                let exe: fn(&Num) -> Num = $to_base;
                exe(this)
            }

            fn from_base(&self, s: &Num) -> Num {
                let exe: fn(&Num) -> Num = $from_base;
                exe(s)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_unit_space {
    ($struct:ident, $name:expr, $units:expr) => {
        pub struct $struct;
        impl UnitSpace for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn units(&self) -> &'static [&'static dyn Conversion] {
                $units
            }
        }
    };
}

impl Display for dyn UnitSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Display for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.name())
    }
}

impl Debug for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
