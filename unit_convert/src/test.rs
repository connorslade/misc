use std::str::FromStr;

use anyhow::Result;
use approx::assert_abs_diff_eq;

use crate::{dimension::Dimensions, input, Num};

const ERROR: Num = 0.01;

fn convert(inp: &str) -> Result<Num> {
    let inp = input::Input::from_str(&inp)?;

    let from_dim = Dimensions::from_str(&inp.from_unit)?;
    let to_dim = Dimensions::from_str(&inp.to_unit)?;

    let val = from_dim.convert(&to_dim, inp.value)?;
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
                        assert_abs_diff_eq!(convert($test).unwrap(), $res, epsilon = ERROR);
                    )*
                }
            )*
        }
    };
}

tests! {
    basic => [
        "10m/s => mi/h" => 22.37,
        "10m/s => cm/s" => 1000.0,
        "10m/s^2 => mi/h^2" => 80529.71, // 80530.0?
        "10 m/s^3 => yard/s^3" => 10.94
    ]
}
