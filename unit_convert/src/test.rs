use std::str::FromStr;

use anyhow::Result;
use float_eq::float_eq;

use crate::{dimension::Dimensions, input, Num};

const ERROR: Num = 0.000_1;

fn convert(inp: &str) -> Result<Num> {
    let inp = input::Input::from_str(&inp)?;

    let from_dim = Dimensions::from_str(&inp.from_unit)?;
    let to_dim = Dimensions::from_str(&inp.to_unit)?;

    let val = from_dim.convert(to_dim, inp.value)?;
    Ok(val)
}

macro_rules! tests {
    ($(
        $id:ident => [
            $($test:expr => $res:expr),*
        ]
    ),*) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_convert_ $id>]() {
                    $(
                        float_eq!(convert($test).unwrap(), $res, abs <= ERROR);
                    )*
                }
            )*
        }
    };
}

tests! {
    basic => [
        "10m/s => mi/h" => 22.3693629,
        "10m/s => cm/s" => 1000.0
    ]
}
