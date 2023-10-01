use super::{impl_conversion, impl_unit_space, Conversion, Num, UnitSpace};

impl_unit_space!(Duration, "duration", &[&Second, &Minute, &Hour, &Sol]);

impl_conversion!(Second, "second", |s| *s, |s| *s);
impl_conversion!(Minute, "minute", |m| m * 60.0, |s| s / 60.0);
impl_conversion!(Hour, "hour", |h| h * 3600.0, |s| s / 3600.0);
impl_conversion!(Sol, "sol", |s| s * 88740.244, |s| s / 88740.244);
