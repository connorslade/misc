mod dimension;
mod input;
mod prefix;
mod units;

use std::str::FromStr;

type Num = f64;

fn main() {
    let inp = "10m/s => cm/s";
    let inp = input::Input::from_str(inp);
    let _ = dbg!(inp);
}

// m/s^2
// dim => m{1}, s{-2}
