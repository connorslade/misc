mod dimension;
mod input;
mod prefix;
mod units;

use std::str::FromStr;

use dimension::Dimensions;

type Num = f64;

fn main() {
    let inp = "10m/s => mi/h";
    let inp = dbg!(input::Input::from_str(inp)).unwrap();

    let from_dim = dbg!(Dimensions::from_str(&inp.from_unit)).unwrap();
    let to_dim = dbg!(Dimensions::from_str(&inp.to_unit)).unwrap();

    let val = from_dim.convert(to_dim, inp.value);
    println!("{}{} => {}{}", inp.value, inp.from_unit, val, inp.to_unit);
}

// m/s^2
// dim => m{1}, s{-2}
