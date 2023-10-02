use super::{impl_conversion, impl_unit_space, Conversion, Num, Space, UnitSpace};

impl_unit_space!(
    Duration,
    "duration",
    Duration,
    &[&Second, &Minute, &Hour, &Sol]
);

impl_conversion!(
    Second,
    "second",
    Duration,
    |s| *s,
    |s| *s,
    aliases = ["s", "sec"],
    metric = true
);
impl_conversion!(
    Minute,
    "minute",
    Duration,
    |m| m * 60.0,
    |s| s / 60.0,
    aliases = ["min"]
);
impl_conversion!(
    Hour,
    "hour",
    Duration,
    |h| h * 3600.0,
    |s| s / 3600.0,
    aliases = ["h", "hr"]
);
impl_conversion!(Sol, "sol", Duration, |s| s * 88740.244, |s| s / 88740.244);
