mod dimension;
mod input;
mod prefix;
mod units;

use std::str::FromStr;

use dimension::Dimensions;

type Num = f64;

fn main() {
    let inp = "10m/s => cm/s";
    let inp = dbg!(input::Input::from_str(inp)).unwrap();

    let _from_dim = dbg!(Dimensions::from_str(&inp.from_unit)).unwrap();
    let _to_dim = dbg!(Dimensions::from_str(&inp.to_unit)).unwrap();
}

// m/s^2
// dim => m{1}, s{-2}
