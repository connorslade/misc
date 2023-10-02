use super::{impl_conversion, impl_unit_space, Conversion, Num, UnitSpace};

impl_unit_space!(Length, "length", &[&Meter, &Inch, &Foot, &Yard, &Mile]);

impl_conversion!(
    Meter,
    "meter",
    |m| *m,
    |m| *m,
    aliases = ["m", "metre"],
    metric = true
);
impl_conversion!(
    Inch,
    "inch",
    |i| i * 0.0254,
    |m| m / 0.0254,
    aliases = ["in"]
);
impl_conversion!(
    Foot,
    "foot",
    |f| f * 0.3048,
    |m| m / 0.3048,
    aliases = ["ft"]
);
impl_conversion!(
    Yard,
    "yard",
    |y| y * 0.9144,
    |m| m / 0.9144,
    aliases = ["yd"]
);
impl_conversion!(
    Mile,
    "mile",
    |mi| mi * 1609.344,
    |m| m / 1609.344,
    aliases = ["mi"]
);
